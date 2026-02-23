# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog principles and this project
follows Semantic Versioning.

------------------------------------------------------------------------

## \[1.0.1\] - 2026-02-23

### Added

-   Clipboard copy functionality for generated passwords
-   Clipboard copy support for revealed payloads
-   Automatic TTL-based clipboard clearing
-   Explicit memory zeroization command for stored secrets
-   Secure clear ("panic clear") option in GUI

### Improved

-   Debug logging gated to development builds only
-   Windows installer icon handling
-   Installer stability across install locations
-   Presentation-ready release build configuration

### Security

-   Reduced secret persistence via TTL clipboard clearing
-   Explicit in-memory zeroization of sensitive buffers
-   No cryptographic internals logged in release builds

------------------------------------------------------------------------

## \[1.0.0\] - 2026-02-20

### Initial Stable Release

-   AES-256-GCM authenticated encryption
-   Unique nonce and salt per encryption operation
-   PNG LSB steganographic embedding (Red/Blue channels)
-   Structured header format for reliable extraction
-   Diceware passphrase generation (6 & 8 word modes)
-   28-character high-entropy password generator
-   PNG capacity analysis prior to embedding
-   Desktop GUI via Tauri
-   CLI interface for direct terminal usage
-   Windows MSI installer generation

------------------------------------------------------------------------

## Versioning

This project follows Semantic Versioning:

-   MAJOR version for incompatible API changes
-   MINOR version for added functionality in a backward-compatible
    manner
-   PATCH version for backward-compatible bug fixes and security
    improvements
