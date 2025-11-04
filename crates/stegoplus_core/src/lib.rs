// Public modules for access from CLI or other crates
pub mod crypto;
pub mod header;
pub mod stego;
pub mod errors;

// Re-export commonly used functions and types
pub use crypto::{encrypt_aes_gcm_scrypt, decrypt_aes_gcm_scrypt, Encrypted};
pub use stego::{
    embed_payload_into_png,
    extract_payload_from_png,
    make_header_and_payload,
    parse_header_and_payload,
};
