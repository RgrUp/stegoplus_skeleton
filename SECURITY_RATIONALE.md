# StegoPlus Security Rationale

## Overview

StegoPlus is a Rust-based desktop application that combines authenticated encryption with PNG-based steganography to securely embed and recover hidden payload data.

This document outlines the security decisions, threat model, and limitations of the system.

---

## Threat Model

StegoPlus is designed to protect against:

- Passive observers inspecting image files
- Casual forensic inspection of file contents
- Unauthorized access without the correct passphrase
- Payload tampering

StegoPlus does NOT protect against:

- Advanced steganalysis techniques
- Endpoint compromise (malware/keyloggers)
- Active attackers modifying image structure deliberately
- Metadata leakage

---

## Cryptographic Design

### Encryption Algorithm

**AES-256-GCM**

Chosen because:
- Industry standard authenticated encryption
- Provides confidentiality + integrity
- Resistant to known practical attacks
- Hardware-accelerated on modern CPUs

GCM ensures:
- Tampering detection
- Authentication failure on incorrect password
- No silent corruption

---

## Key Derivation Strategy

Passphrases are generated using a Diceware-style wordlist.

Advantages:
- High entropy
- Human-readable
- Resistant to brute force when sufficient word count is used
- Avoids weak user-selected passwords

Future improvement:
- PBKDF2 / Argon2 integration for stronger key stretching

---

## Steganographic Strategy

### Image Format: PNG

PNG selected because:
- Lossless compression
- Predictable pixel structure
- Survives email/cloud transport reliably
- No lossy recompression artifacts

### Embedding Method

Least Significant Bit (LSB) modification of Red/Blue channels.

Reasons:
- Minimal visual distortion
- Stable pixel control
- Simpler implementation reduces error surface
- Controlled capacity calculation prevents overflow

---

## Data Handling

Before embedding:
- Image capacity is calculated
- Payload size is validated
- Structured header ensures recoverability

During extraction:
- Header is parsed
- Ciphertext extracted
- AES-GCM authentication verified

Failure cases:
- Wrong password → authentication error
- Corrupt image → structured failure
- Insufficient capacity → blocked before embed

---

## Security Assumptions

- Host system is secure
- No active memory inspection
- No adversarial image recompression
- User protects passphrase

---

## Known Limitations

- Vulnerable to advanced statistical steganalysis
- Does not resist image resizing or format conversion
- Does not strip PNG metadata
- Not designed for high-threat environments

---

## Future Enhancements

- Argon2 key derivation
- Metadata stripping
- Multi-layer steganographic patterns
- Steganalysis resistance testing
- Hardware-backed key storage
- Mobile version

---

## Conclusion

StegoPlus demonstrates practical integration of modern authenticated encryption with steganographic concealment in a desktop application environment.

It is suitable for educational, experimental, and low-risk privacy use cases.

Author: Kevin Dunton  
Year: 2026