# StegoPlus Security Rationale

Version: 1.0.1\
Author: Kevin Dunton\
Year: 2026

------------------------------------------------------------------------

## Overview

StegoPlus is a Rust-based desktop application that integrates
authenticated encryption with PNG-based steganography to securely
conceal and recover hidden payload data.

This document outlines the security decisions, threat model,
implementation choices, and limitations of the system.

The goal of StegoPlus is not to function as a hardened high-assurance
cryptographic system, but to demonstrate secure software engineering
principles in a practical desktop application environment.

------------------------------------------------------------------------

## Threat Model

StegoPlus is designed to protect against:

-   Passive observers inspecting image files
-   Casual forensic inspection of file contents
-   Unauthorized access without the correct passphrase
-   Payload tampering or corruption

StegoPlus does NOT protect against:

-   Advanced statistical steganalysis
-   Endpoint compromise (malware, keyloggers, memory inspection)
-   Active adversarial manipulation of image structure
-   Metadata-based analysis
-   Lossy image transformations (JPEG conversion, resizing)

------------------------------------------------------------------------

## Cryptographic Design

### Encryption Algorithm

StegoPlus uses **AES-256-GCM**, an industry-standard authenticated
encryption scheme providing:

-   Confidentiality
-   Integrity
-   Authentication

Benefits of AES-GCM in this context:

-   Tampering detection
-   Authentication failure on incorrect passphrase
-   No silent corruption of decrypted output
-   Hardware acceleration support on modern CPUs

Each encryption operation generates:

-   A unique random nonce
-   A unique random salt

Secure randomness is sourced from `OsRng`.

------------------------------------------------------------------------

## Key Derivation Strategy

Passphrases are generated using a Diceware-style wordlist.

Advantages:

-   High entropy per word
-   Human-readable and memorable
-   Resistant to brute-force attacks when sufficient word count is used
-   Avoids weak user-selected passwords

Future enhancement consideration:

-   Argon2-based key derivation for stronger key stretching

------------------------------------------------------------------------

## Steganographic Strategy

### Image Format Selection: PNG

PNG was selected because:

-   It uses lossless compression
-   Pixel structure remains predictable
-   It survives cloud/email transport reliably
-   No lossy recompression artifacts are introduced

### Embedding Method

StegoPlus uses Least Significant Bit (LSB) modification of the Red and
Blue channels.

Design considerations:

-   Minimal perceptible visual distortion
-   Controlled embedding capacity
-   Reduced implementation complexity (smaller attack surface)
-   Structured header ensures deterministic extraction

Before embedding:

-   Image capacity is calculated
-   Payload size is validated
-   Structured header is constructed

During extraction:

-   Header is parsed
-   Ciphertext is reconstructed
-   AES-GCM authentication is verified

Failure conditions are explicit and controlled.

------------------------------------------------------------------------

## Memory & Secret Handling (v1.0.1 Enhancements)

StegoPlus includes additional defensive measures to reduce secret
persistence:

### Clipboard Controls

-   Clipboard copy operations include TTL-based automatic clearing
-   Reduces exposure to clipboard scraping or unintended persistence

### Explicit Zeroization

-   Generated passwords and revealed payloads can be explicitly zeroized
    in memory
-   Sensitive buffers are overwritten before being dropped
-   Reduces memory remanence risk

### Debug Logging Controls

-   Cryptographic debug output is gated to development builds only
-   Release builds do not log nonce, salt, or ciphertext internals

These measures demonstrate applied secure coding practices beyond
baseline cryptographic correctness.

------------------------------------------------------------------------

## Data Handling Guarantees

StegoPlus ensures:

-   Incorrect passphrase → authentication failure
-   Corrupt image → structured extraction failure
-   Insufficient image capacity → embedding blocked
-   No plaintext logging in release builds

------------------------------------------------------------------------

## Security Assumptions

StegoPlus assumes:

-   The host system is secure
-   No active memory inspection by adversaries
-   No adversarial image recompression
-   The user protects the passphrase

It is not intended for high-threat nation-state environments.

------------------------------------------------------------------------

## Known Limitations

-   Vulnerable to advanced statistical steganalysis
-   Not robust against image resizing or lossy conversion
-   Does not strip PNG metadata
-   Does not defend against endpoint compromise
-   Clipboard history may persist beyond application control

------------------------------------------------------------------------

## Future Enhancements

-   Argon2 key derivation
-   PNG metadata stripping
-   Enhanced steganalysis resistance testing
-   Hardware-backed key storage
-   Cross-platform support
-   Secure enclave integration (long-term research)

------------------------------------------------------------------------

## Conclusion

StegoPlus demonstrates the practical integration of:

-   Authenticated encryption
-   Structured steganographic concealment
-   Defensive memory handling
-   Secure desktop application design

While not designed for high-threat operational environments, it
represents a strong educational and applied security engineering
project.
