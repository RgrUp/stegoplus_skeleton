use std::fs;
use stegoplus_core::{
    encrypt_aes_gcm_scrypt, decrypt_aes_gcm_scrypt, Encrypted,
    make_header_and_payload, parse_header_and_payload,
    embed_payload_into_png, extract_payload_from_png,
};
use stegoplus_core::header::Header; // for Header::from_bytes

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage:\n  stegoplus_cli hide <cover.png> <passphrase>\n  stegoplus_cli reveal <stego.png> <passphrase>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "hide" => {
            let cover = fs::read(&args[2]).expect("read cover.png");
            let pass = args[3].as_bytes();

            let msg = b"hello from cli"; // TODO: replace with stdin or file input
            let enc: Encrypted = encrypt_aes_gcm_scrypt(msg, pass).expect("encrypt");

            // salt(16) | nonce(12) | ct||tag
            let mut blob = Vec::with_capacity(28 + enc.ciphertext_and_tag.len());
            blob.extend_from_slice(&enc.salt);
            blob.extend_from_slice(&enc.nonce);
            blob.extend_from_slice(&enc.ciphertext_and_tag);

            let framed = make_header_and_payload(&blob);
            let stego = embed_payload_into_png(&cover, &framed).expect("embed");
            fs::write("output.png", stego).expect("write output.png");
            println!("Wrote output.png");
        }

        "reveal" => {
            let stego_png = fs::read(&args[2]).expect("read stego.png");
            let pass = args[3].as_bytes();

            // 1) Extract just the 14-byte header, then parse HEADER ONLY
            let header_bytes = extract_payload_from_png(&stego_png, 14).expect("extract header");
            let header = Header::from_bytes(&header_bytes[..]).expect("parse header only");

            // 2) Now extract the full frame (header + payload)
            let total = 14 + header.len as usize;
            let full_frame = extract_payload_from_png(&stego_png, total).expect("extract full");
            let (_hdr2, payload) = parse_header_and_payload(&full_frame).expect("parse full frame");

            // 3) Deserialize Encrypted: salt(16) | nonce(12) | ct||tag
            let salt = <[u8; 16]>::try_from(&payload[0..16]).unwrap();
            let nonce = <[u8; 12]>::try_from(&payload[16..28]).unwrap();
            let ct = payload[28..].to_vec();
            let enc = Encrypted { salt, nonce, ciphertext_and_tag: ct };

            // 4) Decrypt
            let pt = decrypt_aes_gcm_scrypt(&enc, pass).expect("decrypt");
            println!("Message: {}", String::from_utf8_lossy(&pt));
        }

        _ => eprintln!("Unknown command"),
    }
}
