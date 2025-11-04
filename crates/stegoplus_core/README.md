# StegoPlus Skeleton

Minimal **Rust core** (AES-256-GCM + scrypt + PNG LSB) with **Dart FFI** stub for a Flutter app on Android.
This is a working starting point for your April deadline.

## Layout
```
stegoplus/
  core/            # Rust crate (builds to cdylib + rlib)
  flutter_stub/
    lib/
      stegoplus_ffi.dart
      main.dart    # demo UI (reads /sdcard/Download/cover.png)
```

## Prereqs
- Rust + Cargo
- Android NDK (for .so builds) and `cargo-ndk` (`cargo install cargo-ndk`)
- Flutter SDK (for the stub UI)

## Build the Rust core for Android (ARM64)
From `stegoplus/core`:
```bash
# Install Android targets once:
rustup target add aarch64-linux-android

# Build release .so with cargo-ndk
cargo ndk -t arm64-v8a -o ../flutter_app/android/app/src/main/jniLibs build --release
# If you don't have a full Flutter app yet, output somewhere convenient:
cargo ndk -t arm64-v8a -o ../jniLibs build --release
```
This produces `libstegoplus_core.so` for ARM64.

> For multi-ABI: add `-t armeabi-v7a -t arm64-v8a -t x86_64`

## Use in a Flutter app
1. Create a Flutter app:
```bash
flutter create stegoplus_app
```
2. Copy files:
   - `stegoplus/flutter_stub/lib/stegoplus_ffi.dart` → `stegoplus_app/lib/stegoplus_ffi.dart`
   - `stegoplus/flutter_stub/lib/main.dart` → `stegoplus_app/lib/main.dart` (replace the starter main)
3. Place your built `.so` at:
   - `stegoplus_app/android/app/src/main/jniLibs/arm64-v8a/libstegoplus_core.so`
4. Run on an ARM64 device:
```bash
flutter run
```
5. For the demo button to work, put a PNG at `/sdcard/Download/cover.png` on your device.

## iOS (next step)
- Build a static lib for iOS (`aarch64-apple-ios`) and expose symbols; on iOS, `DynamicLibrary.process()` finds symbols linked into the app.
- Alternatively, use a Swift wrapper for nicer integration. For v1 schedule, get Android validated first, then mirror on iOS.

## API (FFI)
- `stgplus_encrypt_embed_png(cover_ptr, cover_len, msg_ptr, msg_len, pass_ptr, pass_len, out_ptr*, out_len*) -> int`
- `stgplus_extract_decrypt_png(stego_ptr, stego_len, pass_ptr, pass_len, out_ptr*, out_len*) -> int`
- `stgplus_free(ptr, len)`

All buffers are raw bytes; return `0` on success, negative otherwise.

## Header & Payload
- Header (14 bytes): `MAGIC(4)="STG+" | VER(1)=0x01 | FLAGS(1)=0x01 | LEN(4, BE) | CRC32(4)`
- Payload: `salt(16) | nonce(12) | ciphertext||tag(16B tag appended by AES-GCM)`

## Notes
- This skeleton uses 1 LSB per RGB channel sequentially (simple, good for v1).
- Capacity = `width * height * 3 / 8` bytes. App checks and errors if too small.
- scrypt params: N=2^15, r=8, p=1.

## Next actions for you
- Build Android .so via `cargo-ndk` and drop into a fresh Flutter app's `jniLibs`.
- Replace the demo file IO with an Image Picker + proper permissions.
- Add passphrase UI + validation, then wire to the FFI calls.
- Start writing unit tests around the Rust core (CLI harness optional).
