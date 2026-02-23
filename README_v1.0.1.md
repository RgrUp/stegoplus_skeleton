# StegoPlus v1.0.1

StegoPlus is a Rust-powered desktop application that integrates
authenticated encryption with PNG-based steganography to securely embed
and recover hidden payload data.

Developed as a Senior Cybersecurity Capstone Project (2026), StegoPlus
demonstrates practical application of modern cryptography, secure coding
practices, and defensive software design in a desktop environment.

**Author:** Kevin Dunton\
**Language:** Rust\
**Framework:** Tauri\
**Platform:** Windows (MSI & NSIS installers)

------------------------------------------------------------------------

## Overview

StegoPlus encrypts user-provided data using AES-256-GCM and embeds the
encrypted payload into PNG images using Least Significant Bit (LSB)
steganography. The application includes both a desktop GUI (Tauri) and a
CLI interface.

The project emphasizes:

-   Authenticated encryption
-   Secure randomness
-   Defensive memory handling
-   Explicit threat modeling
-   Clean architectural separation between UI and cryptographic core

------------------------------------------------------------------------

## Key Features

-   AES-256-GCM authenticated encryption
-   Unique nonce and salt per encryption operation
-   Diceware-based passphrase generation (6 or 8 words)
-   28-character high-entropy password generator
-   PNG capacity analysis prior to embedding
-   LSB embedding in Red/Blue channels
-   Structured header for reliable extraction
-   Secure extraction and authentication validation
-   Desktop GUI built with Tauri
-   CLI version for direct terminal use
-   Windows MSI and NSIS installer builds

### Security-Oriented Enhancements (v1.0.1)

-   Clipboard copy with automatic TTL-based clearing
-   Reveal-to-clipboard functionality
-   Explicit in-memory zeroization command
-   Debug logging disabled in release builds
-   Stable Windows installer icon packaging

------------------------------------------------------------------------

## Architecture

StegoPlus is structured as a Rust workspace:

    STEGOPLUS_SKELETON_V1.0.0
    │
    ├── apps/
    │   ├── stegoplus_cli
    │   └── stegoplus_desktop  (Tauri GUI)
    │
    └── crates/
        └── stegoplus_core
            ├── crypto.rs
            ├── stego.rs
            ├── header.rs
            ├── ffi.rs
            └── errors.rs

### Layered Design

**Core Library (`stegoplus_core`)** - AES-256-GCM encryption - Secure
random generation (OsRng) - Diceware passphrase generator - PNG LSB
embed/extract engine - Structured payload header management

**Desktop Layer (`stegoplus_desktop`)** - Tauri frontend - Rust backend
bridge via command invocation - Secure clipboard operations - Installer
packaging

**CLI Layer (`stegoplus_cli`)** - Direct terminal interface to core
functionality

------------------------------------------------------------------------

## Cryptographic Design

### Encryption

StegoPlus uses **AES-256-GCM**, providing:

-   Confidentiality
-   Integrity
-   Authentication

Each encryption operation generates:

-   A unique random nonce
-   A unique random salt

------------------------------------------------------------------------

## Steganographic Strategy

### Format: PNG

PNG was selected because:

-   It uses lossless compression
-   Pixel structure remains stable
-   It survives email/cloud transport
-   It avoids lossy recompression artifacts

### Embedding Method

-   Least Significant Bit (LSB) modification
-   Red and Blue channel usage
-   Capacity validated before embedding
-   Structured header ensures recoverability

------------------------------------------------------------------------

## Memory & Secret Handling

StegoPlus incorporates secure coding practices beyond basic encryption:

-   Clipboard copy includes TTL-based automatic clearing
-   Explicit zeroization command wipes stored secrets in memory
-   Debug logging is gated to development builds only
-   No plaintext secrets are logged in release builds

------------------------------------------------------------------------

## Threat Model

StegoPlus is designed to protect against:

-   Passive observers inspecting image files
-   Casual forensic inspection
-   Unauthorized access without the correct passphrase
-   Payload tampering

StegoPlus does NOT protect against:

-   Advanced statistical steganalysis
-   Endpoint compromise (malware, memory inspection)
-   Active image structure manipulation
-   Metadata leakage
-   Lossy image transformation (e.g., JPEG conversion)

------------------------------------------------------------------------

## Installation

Download the latest release from the GitHub Releases page.

Run the Windows installer:

-   NSIS setup executable (recommended for demo)
-   MSI package (enterprise-style installer)

------------------------------------------------------------------------

## Building from Source

### Run in Development Mode

    cd apps/stegoplus_desktop
    npm run tauri dev

### Build Release Installers

    npm run tauri build

Release artifacts will be located in:

    src-tauri/target/release/bundle/

------------------------------------------------------------------------

## Demo Workflow

1.  Launch StegoPlus
2.  Select a PNG cover image
3.  Generate password or Diceware passphrase
4.  Enter payload data
5.  Click Embed
6.  Save stego image
7.  Select stego image
8.  Click Reveal
9.  Enter passphrase
10. Verify recovered payload
11. Optionally copy to clipboard or zeroize secrets

------------------------------------------------------------------------

## License

Educational use. Not intended for malicious or unlawful purposes.
