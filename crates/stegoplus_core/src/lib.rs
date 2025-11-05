// Public modules for access from CLI or other crates
pub mod crypto;
pub mod header;
pub mod stego;
pub mod errors;

// Re-export commonly used functions and types from crypto
pub use crypto::{encrypt_aes_gcm_scrypt, decrypt_aes_gcm_scrypt, Encrypted};

/*
 The stego module is exposed via `pub mod stego;` above.
 Access stego functionality as `stegoplus_core::stego::...`.
 Do not attempt to re-export individual stego functions here unless those
 symbols actually exist in the `stego` module to avoid unresolved import errors.
*/
