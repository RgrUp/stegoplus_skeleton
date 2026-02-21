# StegoPlus

📚 Academic Context

Developed as a Senior Cybersecurity Capstone Project (2026).

Author: Kevin Dunton
Language: Rust
Framework: Tauri
Platform: Windows (MSI)

📜 License

Educational use. Not intended for malicious or unlawful purposes.

**StegoPlus** is a Rust-based steganography and encryption desktop application developed as a Cybersecurity Senior Project.

It securely encrypts payload data using AES-256-GCM and embeds it inside PNG images using Least Significant Bit (LSB) steganography.

---

## 🔐 Features

- AES-256-GCM authenticated encryption
- Diceware-based passphrase generation
- PNG capacity analysis before embedding
- LSB embedding (Red/Blue channels)
- Secure extraction and authenticated decryption
- Desktop GUI built with Tauri
- CLI version for direct terminal use
- Windows MSI installer build

---

## 🧠 Architecture

StegoPlus is structured as a Rust workspace:
STEGOPLUS_SKELETON_V1.0.0
│
├── apps/
│ ├── stegoplus_cli
│ └── stegoplus_desktop (Tauri GUI)
│
└── crates/
└── stegoplus_core
├── crypto.rs
├── stego.rs
├── header.rs
├── ffi.rs
└── errors.rs

```mermaid
flowchart TD

    A[User Interface<br> Tauri Frontend] --> B[FFI Bridge]
    B --> C[StegoPlus Core Library]

    C --> D[Crypto Module<br> AES-256-GCM]
    C --> E[Diceware Generator]
    C --> F[Stego Engine<br> LSB PNG Embed/Extract]

    F --> G[PNG File System I/O]
    D --> H[Encrypted Payload Output]

    G --> F
    H --> F


### Core Layer
- Encryption: AES-256-GCM
- Passphrase: Diceware wordlist
- Steganography: LSB embedding in PNG RB channels

### Desktop Layer
- Tauri frontend
- Rust backend bridge via FFI
- Splash screen on launch
- Windows MSI packaging

---

## 🛡 Security Design

### Encryption
- AES-256-GCM provides:
  - Confidentiality
  - Integrity
  - Authentication

Wrong passwords fail authentication cleanly.

### Passphrase Strategy
Diceware-style word generation provides human-usable entropy without relying on weak short passwords.

### Image Strategy
PNG format chosen because:
- Lossless compression
- Stable pixel structure
- Resistant to corruption during cloud/email transport

---

## ⚠ Limitations

- Not resistant to advanced steganalysis
- Not robust against lossy transformations (JPEG conversion, resizing)
- Endpoint security is assumed secure
- Metadata is not currently stripped

---

## 🚀 Building

### CLI

```bash
cargo run -p stegoplus_cli

### Desktop (Tauri)

cd apps/stegoplus_desktop
cargo tauri build

## Demo Steps

1. Launch StegoPlus
2. Select a PNG cover image
3. Enter or load payload text
4. Generate Diceware passphrase
5. Click Embed
6. Save stego image
7. Select stego image
8. Click Extract
9. Enter passphrase
10. Verify recovered payload