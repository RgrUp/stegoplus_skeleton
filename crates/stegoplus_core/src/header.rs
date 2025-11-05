use aes_gcm::aes::cipher;
use crc32fast::Hasher;

pub const MAGIC: [u8;4] = *b"STG+";
pub const VERSION: u8 = 0x01;

// Flags bit layout (v1):
// bit0 = AES-GCM (1)
// bit1 = compressed (zstd) (0 for now)
pub const FLAG_AES_GCM: u8 = 0b0000_0001;

#[derive(Debug, Clone)]
pub struct Header {
    pub magic: [u8;4],
    pub version: u8,
    pub flags: u8,
    pub cipher: u8,
    pub len: u32,
    pub crc32: u32,
    pub payload_len: u64,
    pub meta_crc32: u32,
    pub nonce: [u8;12],
    pub salt: [u8;16],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    Aes256Gcm,
    // Add other ciphers here if needed
}

impl Cipher {
    pub const AES256_GCM: u8 = 0x01; // AES-256-GCM
}

// If you intended to use a Flags type, define it like this:
#[derive(Debug, Clone, Copy)]
pub struct Flags(pub u8);

impl Flags {
    pub const COMPRESSED: u8 = 0b0000_0010;
    pub fn from_bits_truncate(bits: u8) -> u8 {
        bits & 0b0000_0011 // only bits 0 and 1 are defined for now
    }
    pub fn bits(&self) -> u8 {
        self.0
    }
}

impl Header {
    pub fn to_bytes(&self) -> [u8;14] {
        let mut out = [0u8;14];
        out[0..4].copy_from_slice(&self.magic);
        out[4] = self.version;
        out[5] = self.flags;
        out[6..10].copy_from_slice(&self.len.to_be_bytes());
        out[10..14].copy_from_slice(&self.crc32.to_be_bytes());
        out
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Header> {
        if buf.len() < 14 { return None; }
        let mut magic = [0u8;4];
        magic.copy_from_slice(&buf[0..4]);
        if magic != MAGIC { return None; }
        let version = buf[4];
        let flags = buf[5];
        let len = u32::from_be_bytes(buf[6..10].try_into().ok()?);
        let crc32 = u32::from_be_bytes(buf[10..14].try_into().ok()?);

        // Fields not encoded in the 14-byte header are set to sensible defaults.
        let cipher: u8 = 0;
        let payload_len: u64 = 0;
        let meta_crc32: u32 = 0;
        let nonce: [u8;12] = [0u8;12];
        let salt: [u8;16] = [0u8;16];

        Some(Header {
            magic,
            version,
            flags,
            cipher,
            len,
            crc32,
            payload_len,
            meta_crc32,
            nonce,
            salt,
        })
    }

    pub fn crc32(data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }
}
impl cipher::KeySizeUser for Header {
    type KeySize = cipher::consts::U16;
}

// If you intended to use a Flags type, define it like this:
// #[derive(Debug, Clone, Copy)]
// pub struct Flags(pub u8);
