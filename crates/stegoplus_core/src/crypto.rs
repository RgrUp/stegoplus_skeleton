use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit}; // bring the trait into scope
use rand::RngCore;
use scrypt::{Params, scrypt};
use zeroize::Zeroizing;
use crate::errors::StegoError;

pub struct Encrypted {
    pub salt: [u8;16],
    pub nonce: [u8;12],
    pub ciphertext_and_tag: Vec<u8>, // ciphertext||tag
}

pub fn encrypt_aes_gcm_scrypt(plaintext: &[u8], passphrase: &[u8]) -> Result<Encrypted, StegoError> {
    // scrypt params roughly N=2^15, r=8, p=1, output len=32
    let params = Params::new(15, 8, 1, 32).map_err(|_| StegoError::ScryptFailed)?;

    // random salt
    let mut salt = [0u8;16];
    rand::thread_rng().fill_bytes(&mut salt);

    // derive 32-byte key
    let mut key_bytes = Zeroizing::new([0u8;32]);
    scrypt(passphrase, &salt, &params, &mut *key_bytes).map_err(|_| StegoError::ScryptFailed)?;

    // cipher + random nonce
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes[..]);
    let cipher = Aes256Gcm::new(key);

    let mut nonce = [0u8;12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_obj = Nonce::from_slice(&nonce);

    let aad = b"stego+v1";
    let ct = cipher.encrypt(nonce_obj, aes_gcm::aead::Payload { msg: plaintext, aad })
        .map_err(|_| StegoError::DecryptFailed)?; // reuse error type

    Ok(Encrypted { salt, nonce, ciphertext_and_tag: ct })
}

pub fn decrypt_aes_gcm_scrypt(encrypted: &Encrypted, passphrase: &[u8]) -> Result<Vec<u8>, StegoError> {
    let params = Params::new(15, 8, 1, 32).map_err(|_| StegoError::ScryptFailed)?;

    // derive same 32B key from passphrase+salt
    let mut key_bytes = Zeroizing::new([0u8;32]);
    scrypt(passphrase, &encrypted.salt, &params, &mut *key_bytes).map_err(|_| StegoError::ScryptFailed)?;

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes[..]);
    let cipher = Aes256Gcm::new(key);
    let nonce_obj = Nonce::from_slice(&encrypted.nonce);
    let aad = b"stego+v1";

    let pt = cipher.decrypt(nonce_obj, aes_gcm::aead::Payload {
        msg: &encrypted.ciphertext_and_tag, aad
    }).map_err(|_| StegoError::DecryptFailed)?;

    Ok(pt)
}
