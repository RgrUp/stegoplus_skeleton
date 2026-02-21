#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use std::sync::mpsc;
use once_cell::sync::Lazy;
use rand::{rngs::OsRng, Rng};
use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;

#[tauri::command]
fn ping() -> String {
    println!("[tauri] PING CALLED");
    "pong".to_string()
}

#[tauri::command]
fn analyze_capacity(cover_path: String) -> Result<String, String> {
    println!("==================== ANALYZE CALLED ====================");
    println!("[tauri] analyze_capacity: {}", cover_path);

    println!("[tauri] analyze_capacity: {cover_path}");

    let cover = PathBuf::from(&cover_path);
    let a = stegoplus_core::stego::analyze_cover(&cover).map_err(|e| e.to_string())?;

    Ok(format!(
        "Pixels: {}\nCapacity: {} bytes (using {} LSBs per pixel in R/B)",
        a.pixels, a.capacity_bytes, a.bits_per_pixel_used
    ))
}

#[tauri::command]
fn hide_message(
    cover_path: String,
    passphrase: String,
    message: String,
    out_path: String,
) -> Result<String, String> {
    println!("[tauri] hide_message: cover={cover_path} out={out_path} msg_len={}", message.len());

    let cover = PathBuf::from(&cover_path);
    let out = PathBuf::from(&out_path);

    // temp payload file (core hides files)
    let mut tmp = std::env::temp_dir();
    tmp.push(format!("stegoplus_payload_{}.txt", std::process::id()));

    fs::write(&tmp, message.as_bytes()).map_err(|e| format!("write temp payload failed: {e}"))?;

    // same default as CLI: compression level 6
    stegoplus_core::stego::hide_file(&cover, &out, passphrase.as_bytes(), &tmp, 6)
        .map_err(|e| e.to_string())?;

    let _ = fs::remove_file(&tmp);

    Ok(format!("Wrote {}", out.display()))
}

#[tauri::command]
fn reveal_message(stego_path: String, passphrase: String) -> Result<String, String> {
    println!("[tauri] reveal_message: stego={stego_path}");

    let stego = PathBuf::from(&stego_path);

    let mut out = std::env::temp_dir();
    out.push(format!("stegoplus_revealed_{}.bin", std::process::id()));

    stegoplus_core::stego::reveal_file(&stego, passphrase.as_bytes(), &out)
        .map_err(|e| e.to_string())?;

    let bytes = fs::read(&out).map_err(|e| format!("read revealed failed: {e}"))?;
    let _ = fs::remove_file(&out);

    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Ok(format!("[non-UTF8 output; base64]\n{}", B64.encode(e.into_bytes()))),
    }
}

#[tauri::command]
fn pick_png_or_bmp(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .add_filter("Images", &["png", "bmp"])
        .pick_file(move |file| {
            let _ = tx.send(file);
        });

    let file = rx.recv().map_err(|e| e.to_string())?;
    Ok(file.and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string())))
}

#[tauri::command]
fn pick_any_file(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .pick_file(move |file| {
            let _ = tx.send(file);
        });

    let file = rx.recv().map_err(|e| e.to_string())?;
    Ok(file.and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string())))
}

#[tauri::command]
fn save_png(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .add_filter("PNG Image", &["png"])
        .save_file(move |file| {
            let _ = tx.send(file);
        });

    let file = rx.recv().map_err(|e| e.to_string())?;
    Ok(file.and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string())))
}
static DICEWARE_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let raw = include_str!("diceware_eff.txt");
    let mut map = HashMap::new();

    for (_i, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // supports either tab or spaces between roll and word
        let mut parts = line.split_whitespace();
        let roll = parts.next().unwrap_or("").to_string();
        let word = parts.next().unwrap_or("").to_string();

        if roll.len() == 5 && !word.is_empty() {
            map.insert(roll, word);
        } else {
            // ignore malformed lines
            // (optional) eprintln!("Skipping malformed line {}: {}", i+1, line);
        }
    }

    map
});

fn roll_5d6(rng: &mut impl Rng) -> String {
    let mut s = String::with_capacity(5);
    for _ in 0..5 {
        let d = rng.gen_range(1..=6);
        s.push(char::from(b'0' + d as u8));
    }
    s
}

/// Generates a diceware passphrase with N words.
/// Returns: "word1 word2 word3 ..."
#[tauri::command]
fn generate_dicephrase(words: Option<u8>) -> Result<String, String> {
    let count = words.unwrap_or(6).clamp(3, 10) as usize; // sane defaults
    if DICEWARE_MAP.is_empty() {
        return Err("Diceware list failed to load (map is empty). Check diceware_eff.txt formatting.".into());
    }

    let mut rng = rand::thread_rng();
    let mut out: Vec<String> = Vec::with_capacity(count);

    for _ in 0..count {
        // try a few times in case a roll isn't present (shouldn't happen with full list)
        let mut found = None;
        for _attempt in 0..50 {
            let roll = roll_5d6(&mut rng);
            if let Some(w) = DICEWARE_MAP.get(&roll) {
                found = Some(w.clone());
                break;
            }
        }
        out.push(found.ok_or("Could not find word for generated dice roll (wordlist mismatch).")?);
    }

    Ok(out.join(" "))
}

#[tauri::command]
fn generate_password() -> Result<String, String> {
    const LEN: usize = 28;

    // Good default charset: strong + URL/file friendly-ish (no quotes/spaces)
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~!@#$%^&*+=";

    let mut rng = OsRng;
    let mut out = String::with_capacity(LEN);

    for _ in 0..LEN {
        let idx = rng.gen_range(0..CHARS.len());
        out.push(CHARS[idx] as char);
    }

    Ok(out)
}

fn main() {
    tauri::Builder::default()
        
        .plugin(tauri_plugin_dialog::init())

        // 👇 PUT THE SETUP BLOCK RIGHT HERE
        .setup(|app| {
            // Hide main window initially
            if let Some(main) = app.get_webview_window("main") {
                let _ = main.hide();
            }

            // Show splash window
            if let Some(splash) = app.get_webview_window("splash") {
                let _ = splash.show();
            }

            let app_handle = app.handle().clone();
            let splash_ms: u64 = 4500;

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_millis(splash_ms));

                if let Some(splash) = app_handle.get_webview_window("splash") {
                    let _ = splash.close();
                }

                if let Some(main) = app_handle.get_webview_window("main") {
                    let _ = main.show();
                    let _ = main.set_focus();
                }
            });

            Ok(())
        })

        // 👇 THEN YOUR COMMAND HANDLER
        .invoke_handler(tauri::generate_handler![
            pick_png_or_bmp,
            pick_any_file,
            save_png,
            ping,
            analyze_capacity,
            hide_message,
            reveal_message,
            generate_dicephrase,
            generate_password
])


        // 👇 THEN RUN
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

