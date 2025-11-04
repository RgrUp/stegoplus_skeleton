import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

typedef _EncryptNative = ffi.Int32 Function(
  ffi.Pointer<ffi.Uint8>, ffi.Uint64, // cover ptr/len
  ffi.Pointer<ffi.Uint8>, ffi.Uint64, // msg ptr/len
  ffi.Pointer<ffi.Uint8>, ffi.Uint64, // pass ptr/len
  ffi.Pointer<ffi.Pointer<ffi.Uint8>>, ffi.Pointer<ffi.Uint64> // out ptr/len
);

typedef _DecryptNative = ffi.Int32 Function(
  ffi.Pointer<ffi.Uint8>, ffi.Uint64, // stego ptr/len
  ffi.Pointer<ffi.Uint8>, ffi.Uint64, // pass ptr/len
  ffi.Pointer<ffi.Pointer<ffi.Uint8>>, ffi.Pointer<ffi.Uint64> // out ptr/len
);

typedef _FreeNative = ffi.Void Function(ffi.Pointer<ffi.Uint8>, ffi.Uint64);

class StegoPlusFFI {
  late final ffi.DynamicLibrary _lib;
  late final _EncryptNative _encrypt;
  late final _DecryptNative _decrypt;
  late final _FreeNative _free;

  StegoPlusFFI({ffi.DynamicLibrary? lib}) {
    _lib = lib ?? _open();
    _encrypt = _lib.lookupFunction<_EncryptNative, _EncryptNative>('stgplus_encrypt_embed_png');
    _decrypt = _lib.lookupFunction<_DecryptNative, _DecryptNative>('stgplus_extract_decrypt_png');
    _free = _lib.lookupFunction<_FreeNative, _FreeNative>('stgplus_free');
  }

  static ffi.DynamicLibrary _open() {
    if (Platform.isAndroid) {
      return ffi.DynamicLibrary.open('libstegoplus_core.so');
    } else if (Platform.isIOS || Platform.isMacOS) {
      return ffi.DynamicLibrary.process();
    } else if (Platform.isWindows) {
      return ffi.DynamicLibrary.open('stegoplus_core.dll');
    } else if (Platform.isLinux) {
      return ffi.DynamicLibrary.open('libstegoplus_core.so');
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Uint8List encryptAndEmbedPng({
    required Uint8List coverPng,
    required Uint8List message,
    required String passphrase,
  }) {
    final coverPtr = malloc.allocate<ffi.Uint8>(coverPng.length);
    coverPtr.asTypedList(coverPng.length).setAll(0, coverPng);

    final msgPtr = malloc.allocate<ffi.Uint8>(message.length);
    msgPtr.asTypedList(message.length).setAll(0, message);

    final passBytes = Uint8List.fromList(passphrase.codeUnits);
    final passPtr = malloc.allocate<ffi.Uint8>(passBytes.length);
    passPtr.asTypedList(passBytes.length).setAll(0, passBytes);

    final outPtrPtr = malloc.allocate<ffi.Pointer<ffi.Uint8>>(1);
    final outLenPtr = malloc.allocate<ffi.Uint64>(1);

    try {
      final rc = _encrypt(
        coverPtr, coverPng.length,
        msgPtr, message.length,
        passPtr, passBytes.length,
        outPtrPtr, outLenPtr,
      );
      if (rc != 0) {
        throw Exception('encrypt+embed failed: rc=$rc');
      }
      final outPtr = outPtrPtr.value;
      final outLen = outLenPtr.value;
      final out = outPtr.asTypedList(outLen);
      final result = Uint8List.fromList(out);
      _free(outPtr, outLen);
      return result;
    } finally {
      malloc.free(coverPtr);
      malloc.free(msgPtr);
      malloc.free(passPtr);
      malloc.free(outPtrPtr);
      malloc.free(outLenPtr);
    }
  }

  Uint8List extractAndDecryptPng({
    required Uint8List stegoPng,
    required String passphrase,
  }) {
    final stegoPtr = malloc.allocate<ffi.Uint8>(stegoPng.length);
    stegoPtr.asTypedList(stegoPng.length).setAll(0, stegoPng);

    final passBytes = Uint8List.fromList(passphrase.codeUnits);
    final passPtr = malloc.allocate<ffi.Uint8>(passBytes.length);
    passPtr.asTypedList(passBytes.length).setAll(0, passBytes);

    final outPtrPtr = malloc.allocate<ffi.Pointer<ffi.Uint8>>(1);
    final outLenPtr = malloc.allocate<ffi.Uint64>(1);

    try {
      final rc = _decrypt(
        stegoPtr, stegoPng.length,
        passPtr, passBytes.length,
        outPtrPtr, outLenPtr,
      );
      if (rc != 0) {
        throw Exception('extract+decrypt failed: rc=$rc');
      }
      final outPtr = outPtrPtr.value;
      final outLen = outLenPtr.value;
      final out = outPtr.asTypedList(outLen);
      final result = Uint8List.fromList(out);
      _free(outPtr, outLen);
      return result;
    } finally {
      malloc.free(stegoPtr);
      malloc.free(passPtr);
      malloc.free(outPtrPtr);
      malloc.free(outLenPtr);
    }
  }
}
