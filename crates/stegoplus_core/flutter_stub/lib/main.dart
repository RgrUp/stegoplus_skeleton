import 'dart:typed_data';
import 'dart:io';
import 'package:flutter/material.dart';
import 'stegoplus_ffi.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});
  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final stego = StegoPlusFFI();
  String status = 'Idle';
  Uint8List? outputPng;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('StegoPlus (Stub)')),
        body: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(status),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: _demoRoundTrip,
                child: const Text('Demo: Embed "hello" into cover.png'),
              ),
              if (outputPng != null) ...[
                const SizedBox(height: 12),
                Text('Output PNG size: ${outputPng!.length} bytes'),
              ],
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _demoRoundTrip() async {
    try {
      setState(() => status = 'Loading cover.png from assets...');
      // In a real app, use ImagePicker or FilePicker. For stub, read a file from app dir.
      // Place a PNG at /sdcard/Download/cover.png (Android) for quick testing, or adjust path.
      final path = '/sdcard/Download/cover.png';
      final cover = await File(path).readAsBytes();
      final pass = 'test-pass';
      final msg = Uint8List.fromList('hello world'.codeUnits);

      setState(() => status = 'Encrypting+Embedding...');
      final stegoPng = stego.encryptAndEmbedPng(coverPng: cover, message: msg, passphrase: pass);

      final outPath = '/sdcard/Download/stego_output.png';
      await File(outPath).writeAsBytes(stegoPng);
      setState(() => outputPng = stegoPng);

      setState(() => status = 'Extracting+Decrypting...');
      final recovered = stego.extractAndDecryptPng(stegoPng: stegoPng, passphrase: pass);
      final recoveredStr = String.fromCharCodes(recovered);
      setState(() => status = 'Recovered: $recoveredStr\nSaved: $outPath');
    } catch (e) {
      setState(() => status = 'Error: $e');
    }
  }
}
