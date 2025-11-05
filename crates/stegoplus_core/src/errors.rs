use thiserror::Error;

#[derive(Error, Debug)]
pub enum StegoError {
    #[error("Image parsing error: {0}")]
    ImageError(String),
    #[error("Insufficient capacity: need {needed} bytes, have {have} bytes")]
    Capacity { needed: usize, have: usize },
    #[error("Bad header or not a stego image")]
    BadHeader,
    #[error("Payload truncated or corrupt")]
    Truncated,
    #[error("Decryption failed")]
    DecryptFailed,
    #[error("Scrypt failed")]
    ScryptFailed,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Crypto initialization failed")]
    CryptoInit,
    #[error("Encryption failed")]
    EncryptFailed,
}

impl From<image::ImageError> for StegoError {
    fn from(e: image::ImageError) -> Self {
        StegoError::ImageError(e.to_string())
    }
}
