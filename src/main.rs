use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use xz2::write::XzEncoder;
use chrono::Utc;
use sha2::{Sha256, Digest};
use colored::*;

fn main() {
    // cli
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", "Usage: program <filename>".red());
        std::process::exit(1);
    }

    let txt = &args[1];

    // extension and filename
    let name: String = Path::new(&txt.trim().to_string())
        .with_extension("yava")
        .to_str()
        .unwrap()
        .to_string();

    let original_ext = Path::new(txt)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown");

    // hash and read file
    let data = match fs::read(txt) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not read file '{}'", txt).red());
            std::process::exit(1);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    // metadata
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let metadata = format!(
        "Original Extension: {}\nCompressed Date: {}\nCompressed by: {}\n---BEGIN COMPRESSED DATA---\n",
        original_ext,
        current_time,
        hash
    );

    // compress and save
    let file = match File::create(&name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not create file '{}'", name).red());
            std::process::exit(1);
        }
    };

    let mut encoder = XzEncoder::new(file, 6);

    if let Err(_) = encoder.write_all(metadata.as_bytes()) {
        eprintln!("{}", "Error: Failed to write metadata".red());
        std::process::exit(1);
    }

    if let Err(_) = encoder.write_all(&data) {
        eprintln!("{}", "Error: Failed to write data".red());
        std::process::exit(1);
    }

    if let Err(_) = encoder.finish() {
        eprintln!("{}", "Error: Failed to finish compression".red());
        std::process::exit(1);
    }

    // Success message
    println!("{}", "✨ Success! ✨".green());
    println!("Created: {}", name.blue());
    println!("Checksum: {}", hash[..8].yellow());
    println!("Time: {}", current_time.cyan());
}