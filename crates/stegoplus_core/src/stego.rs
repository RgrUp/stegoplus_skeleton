use crate::errors::StegoError;
use crate::header::{Header, MAGIC, VERSION, FLAG_AES_GCM};
use crc32fast::Hasher;

// Import image types + the trait that provides `write_image`.
use image::{DynamicImage, Rgb, RgbImage};
use image::ImageEncoder;
use image::ExtendedColorType;


fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for &b in bytes {
        for i in (0..8).rev() {
            bits.push((b >> i) & 1);
        }
    }
    bits
}

fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity((bits.len() + 7) / 8);
    let mut acc = 0u8;
    for (i, &bit) in bits.iter().enumerate() {
        acc = (acc << 1) | (bit & 1);
        if i % 8 == 7 {
            out.push(acc);
            acc = 0;
        }
    }
    let rem = bits.len() % 8;
    if rem != 0 {
        acc <<= 8 - rem;
        out.push(acc);
    }
    out
}

/// Returns (image, capacity_bytes)
fn load_rgb_image_and_capacity(png_bytes: &[u8]) -> Result<(RgbImage, usize), StegoError> {
    let img = image::load_from_memory(png_bytes)?;
    let rgb = match img {
        DynamicImage::ImageRgb8(rgb) => rgb,
        _ => img.to_rgb8(),
    };
    // 1 LSB per channel (R,G,B) = 3 bits per pixel
    let capacity_bits = (rgb.width() as usize) * (rgb.height() as usize) * 3;
    let capacity_bytes = capacity_bits / 8;
    Ok((rgb, capacity_bytes))
}

pub fn embed_payload_into_png(png_bytes: &[u8], payload: &[u8]) -> Result<Vec<u8>, StegoError> {
    let (mut rgb, capacity_bytes) = load_rgb_image_and_capacity(png_bytes)?;
    let needed = payload.len();
    if needed > capacity_bytes {
        return Err(StegoError::Capacity { needed, have: capacity_bytes });
    }

    let bits = bytes_to_bits(payload);
    let mut bit_iter = bits.into_iter();
    let mut done = false;

    'rows: for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let p = rgb.get_pixel(x, y);
            let mut r = p[0];
            let mut g = p[1];
            let mut b = p[2];

            if let Some(bit) = bit_iter.next() { r = (r & 0xFE) | bit; } else { rgb.put_pixel(x, y, Rgb([r, g, b])); done = true; break; }
            if let Some(bit) = bit_iter.next() { g = (g & 0xFE) | bit; } else { rgb.put_pixel(x, y, Rgb([r, g, b])); done = true; break; }
            if let Some(bit) = bit_iter.next() { b = (b & 0xFE) | bit; } else { rgb.put_pixel(x, y, Rgb([r, g, b])); done = true; break; }

            rgb.put_pixel(x, y, Rgb([r, g, b]));
        }
        if done { break 'rows; }
    }

    let mut out = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut out);
    encoder.write_image(
        rgb.as_raw(),
        rgb.width(),
        rgb.height(),
        ExtendedColorType::Rgb8
    ).map_err(|e| StegoError::ImageError(e.to_string()))?;
    Ok(out)
}

pub fn extract_payload_from_png(png_bytes: &[u8], total_bytes: usize) -> Result<Vec<u8>, StegoError> {
    let (rgb, capacity_bytes) = load_rgb_image_and_capacity(png_bytes)?;
    if total_bytes > capacity_bytes {
        return Err(StegoError::Capacity { needed: total_bytes, have: capacity_bytes });
    }

    let mut bits: Vec<u8> = Vec::with_capacity(total_bytes * 8);
    'outer: for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let p = rgb.get_pixel(x, y);
            bits.push(p[0] & 1);
            if bits.len() == total_bytes * 8 { break 'outer; }
            bits.push(p[1] & 1);
            if bits.len() == total_bytes * 8 { break 'outer; }
            bits.push(p[2] & 1);
            if bits.len() == total_bytes * 8 { break 'outer; }
        }
    }
    Ok(bits_to_bytes(&bits))
}

pub fn make_header_and_payload(encrypted_blob: &[u8]) -> Vec<u8> {
    let crc32 = {
        let mut h = Hasher::new();
        h.update(encrypted_blob);
        h.finalize()
    };
    let header = Header {
        magic: MAGIC,
        version: VERSION,
        flags: FLAG_AES_GCM,
        len: encrypted_blob.len() as u32,
        crc32,
    };
    let mut out = Vec::with_capacity(14 + encrypted_blob.len());
    out.extend_from_slice(&header.to_bytes());
    out.extend_from_slice(encrypted_blob);
    out
}

pub fn parse_header_and_payload(all_bytes: &[u8]) -> Option<(Header, &[u8])> {
    let header = Header::from_bytes(all_bytes.get(0..14)?)?;
    let payload = all_bytes.get(14..(14 + header.len as usize))?;
    Some((header, payload))
}
