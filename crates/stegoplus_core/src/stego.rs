use anyhow::{Context, Result};
use image::{ImageBuffer, Rgba, DynamicImage, GenericImageView, GenericImage};
use image::Pixel;
use std::path::Path;
use crate::header::{Header, MAGIC, Flags, Cipher};
use crate::crypto; // your encrypt/decrypt functions
use zstd::stream::encode_all;
use std::fs;

macro_rules! dprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    };
}

pub struct CoverAnalysis {
    pub pixels: u64,
    pub bits_per_pixel_used: u8, // using 2 bits (R,B)
    pub capacity_bytes: u64,
}

pub fn analyze_cover(path: &Path) -> Result<CoverAnalysis> {
    let img = image::open(path)
        .with_context(|| format!("Failed to open image: {}", path.display()))?;
    let (w, h) = img.dimensions();
    let pixels = (w as u64) * (h as u64);
    let channels_used = 2u64; // R + B
    let _capacity_bits = pixels * channels_used * 8; // 8 pixels per byte? No: we’ll store 1 bit per channel per pixel.
    // Correct capacity for 2 bits per pixel:
    let capacity_bytes = (pixels * 2) / 8;
    Ok(CoverAnalysis {
        pixels,
        bits_per_pixel_used: 2,
        capacity_bytes,
    })
}

/* ---------------- LSB helpers (R and B channel only) ---------------- */

#[inline]
fn set_lsb(byte: u8, bit: u8) -> u8 { (byte & 0xFE) | (bit & 1) }

#[inline]
fn get_lsb(byte: u8) -> u8 { byte & 1 }

// Embed bits into an RGBA8 buffer (R then B channels per pixel)
fn embed_bits_into_rgba8(buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, bits: &[u8]) -> anyhow::Result<()> {
    let (w, h) = buf.dimensions();
    let mut bit_idx = 0usize;
    for y in 0..h {
        for x in 0..w {
            if bit_idx >= bits.len() { return Ok(()); }
            let px = buf.get_pixel_mut(x, y);
            // R
            if bit_idx < bits.len() {
                px.0[0] = set_lsb(px.0[0], bits[bit_idx]); bit_idx += 1;
            }
            // B
            if bit_idx < bits.len() {
                px.0[2] = set_lsb(px.0[2], bits[bit_idx]); bit_idx += 1;
            }
            if bit_idx >= bits.len() { return Ok(()); }
        }
    }
    anyhow::bail!("Not enough capacity in image");
}

// Extract EXACT number of bytes from an RGBA8 buffer at a byte offset
fn extract_bytes_from_rgba8(buf: &ImageBuffer<Rgba<u8>, Vec<u8>>, start_bytes: usize, length_bytes: usize) -> Vec<u8> {
    let (w, h) = buf.dimensions();
    let start_bits = start_bytes * 8;
    let need_bits  = length_bytes * 8;

    let mut out_bits = Vec::with_capacity(need_bits);
    let mut bit_pos: usize = 0;

    'outer: for y in 0..h {
        for x in 0..w {
            let px = buf.get_pixel(x, y);
            // R bit
            if bit_pos >= start_bits && out_bits.len() < need_bits { out_bits.push(get_lsb(px.0[0])); }
            bit_pos += 1; if out_bits.len() >= need_bits { break 'outer; }
            // B bit
            if bit_pos >= start_bits && out_bits.len() < need_bits { out_bits.push(get_lsb(px.0[2])); }
            bit_pos += 1; if out_bits.len() >= need_bits { break 'outer; }
        }
    }

    // Pack MSB-first to bytes
    let mut out = Vec::with_capacity(length_bytes);
    let mut cur: u8 = 0;
    for (i, b) in out_bits.iter().enumerate() {
        cur = (cur << 1) | (b & 1);
        if i % 8 == 7 { out.push(cur); cur = 0; }
    }
    if out_bits.len() % 8 != 0 {
        cur <<= (8 - (out_bits.len() % 8)) as u8;
        out.push(cur);
    }
    out
}

/* ---------------- Encode/decode bit stream over image ---------------- */

fn _embed_bits_into_image(img: &mut DynamicImage, bits: &[u8]) -> Result<()> {
    // iterate pixels; for each pixel use R then B channel LSB
    let (w,h) = img.dimensions();
    let mut bit_idx = 0usize;
    for y in 0..h {
        for x in 0..w {
            if bit_idx >= bits.len() { return Ok(()); }
            let mut px = img.get_pixel(x, y).to_rgba();
            // R channel
            if bit_idx < bits.len() {
                let b = bits[bit_idx];
                px.0[0] = set_lsb(px.0[0], b);
                bit_idx += 1;
            }
            // B channel
            if bit_idx < bits.len() {
                let b = bits[bit_idx];
                px.0[2] = set_lsb(px.0[2], b);
                bit_idx += 1;
            }
            img.put_pixel(x, y, Rgba(px.0));
            if bit_idx >= bits.len() { return Ok(()); }
        }
    }
    anyhow::bail!("Not enough capacity in image");
}

fn _extract_bytes_from_image(img: &image::DynamicImage, start_bytes: usize, length_bytes: usize) -> Vec<u8> {
    use image::GenericImageView;
    let (w, h) = img.dimensions();
    let start_bits = start_bytes * 8;
    let need_bits  = length_bytes * 8;

    let mut out_bits = Vec::with_capacity(need_bits);
    let mut bit_pos: usize = 0;

    'outer: for y in 0..h {
        for x in 0..w {
            // R then B
            let px = img.get_pixel(x, y).to_rgba();

            // R bit
            if bit_pos >= start_bits && out_bits.len() < need_bits {
                out_bits.push(px.0[0] & 1);
            }
            bit_pos += 1;
            if out_bits.len() >= need_bits { break 'outer; }

            // B bit
            if bit_pos >= start_bits && out_bits.len() < need_bits {
                out_bits.push(px.0[2] & 1);
            }
            bit_pos += 1;
            if out_bits.len() >= need_bits { break 'outer; }
        }
    }

    // pack MSB-first (matches your bytes_to_bits)
    let mut out = Vec::with_capacity(length_bytes);
    let mut cur: u8 = 0;
    for (i, b) in out_bits.iter().enumerate() {
        cur = (cur << 1) | (b & 1);
        if i % 8 == 7 {
            out.push(cur);
            cur = 0;
        }
    }
    if out_bits.len() % 8 != 0 {
        cur <<= (8 - (out_bits.len() % 8)) as u8;
        out.push(cur);
    }
    out
}

// vs code had me add this double check later!!!
//fn extract_bits_from_image(img: &image::DynamicImage, length_bits: usize) -> Vec<u8> {
  //  let bytes_needed = (length_bits + 7) / 8;
    //let bytes = extract_bytes_from_image(img, 0, bytes_needed);
    //let mut out = Vec::with_capacity(length_bits);
    //for byte in bytes {
      //  for i in (0..8).rev() {
        //    if out.len() < length_bits {
          //      out.push((byte >> i) & 1);
            //}
//        }
//    }
//   out
//}

/* ---------------- Bit packing helpers ---------------- */

fn bytes_to_bits(data: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(data.len() * 8);
    for &byte in data {
        for i in (0..8).rev() {
            bits.push(((byte >> i) & 1) as u8);
        }
    }
    bits
}

fn _bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity((bits.len() + 7) / 8);
    let mut cur: u8 = 0;
    for (i, &b) in bits.iter().enumerate() {
        cur = (cur << 1) | (b & 1);
        if i % 8 == 7 {
            out.push(cur);
            cur = 0;
        }
    }
    // If bits not multiple of 8, left-pad the final byte
    let rem = bits.len() % 8;
    if rem != 0 {
        cur <<= (8 - rem) as u8;
        out.push(cur);
    }
    out
}

/* ---------------- Public API: hide / reveal ---------------- */

pub fn hide_file(
    cover_path: &Path,
    out_path: &Path,
    passphrase: &[u8],
    payload_file: &Path,
    compress_level: i32, // 0 = off, else 0..=21 zstd
) -> Result<()> {
  
   // load cover as RGBA8 buffer
    let mut rgba = image::open(cover_path)
        .with_context(|| format!("Failed to open cover: {}", cover_path.display()))?
        .to_rgba8();

    // read payload
    let plaintext = fs::read(payload_file)
        .with_context(|| format!("Failed to read payload: {}", payload_file.display()))?;

    // compress
    let (flags, payload_after_comp) = if compress_level > 0 {
        let comp = encode_all(&*plaintext, compress_level as i32)
            .context("zstd compress failed")?;
        (Flags::COMPRESSED, comp)
    } else {
        (Flags::from_bits_truncate(0), plaintext)
    };

    // encrypt
    let enc = crypto::encrypt_aes_gcm_scrypt(&payload_after_comp, passphrase)
        .context("encrypt failed")?;

    // build header
    let header = Header {
        magic: MAGIC,
        version: 1,
        cipher: Cipher::Aes256Gcm as u8,
        flags,
        nonce: enc.nonce,
        salt: enc.salt, // assuming your Encrypted carries salt used for KDF
        payload_len: enc.ciphertext_and_tag.len() as u64,
        meta_crc32: crate::header::Header::crc32(&payload_after_comp), // optional integrity of plaintext (inside enc scope is better; ok for v1 demo)
        len: (enc.ciphertext_and_tag.len() as u32),
        crc32: crate::header::Header::crc32(&payload_after_comp),
    };

    let header_bytes = header.to_bytes();

dprintln!("HIDE payload len: {}", payload_after_comp.len());
dprintln!("HIDE header_bytes.len = {}", header_bytes.len());
dprintln!("HIDE nonce: {:02x?}", enc.nonce);
dprintln!("HIDE  salt: {:02x?}", enc.salt);
dprintln!("HIDE  ct+tag len: {}", enc.ciphertext_and_tag.len());

    // compute capacity
    // capacity check still based on width*height*2 bits:
    let (w, h) = rgba.dimensions();
    let pixels = (w as u64) * (h as u64);
    let capacity_bytes = (pixels * 2) / 8;
    let needed = header_bytes.len() + 12 + 16 + enc.ciphertext_and_tag.len();
    if (needed as u64) > capacity_bytes {
        anyhow::bail!("payload+header too large: need {} bytes, capacity {}", needed, capacity_bytes);
    }

    // build stream [header | nonce | salt | ct||tag]
    let mut to_embed = Vec::with_capacity(needed);
    to_embed.extend_from_slice(&header_bytes);
    to_embed.extend_from_slice(&enc.nonce);
    to_embed.extend_from_slice(&enc.salt);
    to_embed.extend_from_slice(&enc.ciphertext_and_tag);

    // embed into buffer
    let bitstream = bytes_to_bits(&to_embed);
    embed_bits_into_rgba8(&mut rgba, &bitstream)?;

    // save from RGBA8 buffer
    image::DynamicImage::ImageRgba8(rgba).save(out_path)
        .with_context(|| format!("Failed to save stego image: {}", out_path.display()))?;
    Ok(())
}

pub fn reveal_file(
    stego_path: &Path,
    passphrase: &[u8],
    out_path: &Path,
) -> Result<()> {
    // load stego
    let rgba = image::open(stego_path)
    .with_context(|| format!("Failed to open image: {}", stego_path.display()))?
    .to_rgba8();

    // We must first extract the fixed-size header
    
    const HEADER_BYTES: usize = 14;
    const NONCE_BYTES: usize  = 12;
    const SALT_BYTES: usize   = 16;

    // 1) header
    let header_raw  = extract_bytes_from_rgba8(&rgba, 0, HEADER_BYTES);
    let header      = Header::from_bytes(&header_raw[..HEADER_BYTES]).context("invalid header")?;
    if header.magic != MAGIC { anyhow::bail!("bad magic in header"); }

    let ct_len = header.len as usize;

    // 2) nonce, salt, ct using absolute byte offsets
    let nonce = <[u8;NONCE_BYTES]>::try_from(&extract_bytes_from_rgba8(&rgba, HEADER_BYTES, NONCE_BYTES)[..]).unwrap();
    let salt  = <[u8;SALT_BYTES ]>::try_from(&extract_bytes_from_rgba8(&rgba, HEADER_BYTES + NONCE_BYTES, SALT_BYTES)[..]).unwrap();
    let ct    =  extract_bytes_from_rgba8(&rgba, HEADER_BYTES + NONCE_BYTES + SALT_BYTES, ct_len);

    // (optional) debug to confirm match
    dprintln!("REVEAL nonce: {:02x?}", nonce);
    dprintln!("REVEAL  salt: {:02x?}", salt);
    dprintln!("REVEAL  ct+tag len: {}", ct.len());

    // 3) Decrypt
    let encrypted = crate::crypto::Encrypted { nonce, salt, ciphertext_and_tag: ct };
    let mut pt = crate::crypto::decrypt_aes_gcm_scrypt(&encrypted, passphrase)
        .context("decrypt failed (bad passphrase or corrupted data)")?;

    // 4) Decompress if you actually set a compression flag
    // (if your flags is a u8 and you aren't setting a bit yet, skip this)
    const FLAG_COMPRESSED: u8 = 0b0000_0010; // adjust only if you really use this bit
    if (header.flags & FLAG_COMPRESSED) != 0 {
        pt = zstd::stream::decode_all(&*pt).context("zstd decompress failed")?;
    }

    // 6) If your current header doesn't serialize crc32, skip the CRC check.
    // Otherwise, if it DOES, compare it here.
    // fs::write ...

    std::fs::write(out_path, &pt)
        .with_context(|| format!("Failed to write {}", out_path.display()))?;
    Ok(())
}
