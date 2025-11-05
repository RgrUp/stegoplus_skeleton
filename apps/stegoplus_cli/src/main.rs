use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "stegoplus_cli", version, about = "StegoPlus CLI")]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Analyze a cover image and report max embedding capacity
    Analyze {
        /// Path to a cover image (e.g., PNG/BMP)
        cover: PathBuf,
    },

    /// Hide a payload inside a cover image
    Hide {
        cover: PathBuf,
        /// Passphrase (or later: --pass/--key-file)
        passphrase: String,
        /// Optional output file
        #[arg(long)]
        out: Option<PathBuf>,
        /// Optional payload file (else read from stdin later)
        #[arg(long)]
        msg_file: Option<PathBuf>,
    },

    /// Reveal an embedded payload from a stego image
    Reveal {
        stego: PathBuf,
        passphrase: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Cmd::Analyze { cover } => {
            let a = stegoplus_core::stego::analyze_cover(&cover)?;
            println!(
                "Pixels: {pixels}\nCapacity: {cap} bytes (using {bpp} LSBs per pixel in R/B)",
                pixels = a.pixels,
                cap = a.capacity_bytes,
                bpp = a.bits_per_pixel_used
            );
            Ok(())
        }

                Cmd::Hide { cover, passphrase, out, msg_file } => {
            let out_path = out.unwrap_or_else(|| {
                let mut p = cover.clone();
                // e.g., cover.png -> cover.stego.png
                let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
                p.set_file_name(format!("{stem}.stego.png"));
                p
            });
            let payload = msg_file
                .ok_or_else(|| anyhow::anyhow!("--msg-file <path> is required for now"))?;
            // compress level 6 is a sensible default
            stegoplus_core::stego::hide_file(&cover, &out_path, passphrase.as_bytes(), &payload, 6)?;
            println!("Wrote {}", out_path.display());
            Ok(())
        }

        Cmd::Reveal { stego, passphrase, out } => {
            let out_path = out.unwrap_or_else(|| {
                let mut p = stego.clone();
                p.set_file_name("revealed.bin");
                p
            });
            stegoplus_core::stego::reveal_file(&stego, passphrase.as_bytes(), &out_path)?;
            println!("Revealed → {}", out_path.display());
            Ok(())
        }

    }
}
