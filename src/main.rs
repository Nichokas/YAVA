use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use xz2::write::XzEncoder;
use xz2::read::XzDecoder;
use chrono::Utc;
use sha2::{Sha256, Digest};
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", "Usage: program <filename>".red());
        std::process::exit(1);
    }

    let input_file = &args[1];

    // Verify if the file is a .yava
    if input_file.ends_with(".yava") {
        decompress_file(input_file);
    } else {
        compress_file(input_file);
    }
}

fn decompress_file(input_file: &str) {
    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not open file '{}'", input_file).red());
            std::process::exit(1);
        }
    };

    let mut decoder = XzDecoder::new(file);
    let mut content = String::new();
    if let Err(_) = decoder.read_to_string(&mut content) {
        eprintln!("{}", "Error: Failed to decode file".red());
        std::process::exit(1);
    }

    let parts: Vec<&str> = content.split("---BEGIN COMPRESSED DATA---\n").collect();
    if parts.len() != 2 {
        eprintln!("{}", "Error: Invalid file format".red());
        std::process::exit(1);
    }

    let metadata = parts[0];
    let data = parts[1];

    println!("{}", "\n📂 File Metadata Information 📂".bright_green().bold());
    println!("{}", "═".repeat(50).yellow());

    for line in metadata.lines() {
        if line.starts_with("Original Extension: ") {
            println!("{}: {}", "Original Extension".bright_blue(),
                     line.strip_prefix("Original Extension: ").unwrap().bright_white());
        } else if line.starts_with("Compressed Date: ") {
            println!("{}: {}", "Compressed Date".bright_blue(),
                     line.strip_prefix("Compressed Date: ").unwrap().bright_white());
        } else if line.starts_with("Compressed by: ") {
            println!("{}: {}", "SHA-256 Hash".bright_blue(),
                     line.strip_prefix("Compressed by: ").unwrap().yellow());
        }
    }

    println!("{}", "═".repeat(50).yellow());

    // Get the original hash and calculate the current hash
    let original_hash = metadata
        .lines()
        .find(|line| line.starts_with("Compressed by: "))
        .and_then(|line| line.strip_prefix("Compressed by: "))
        .unwrap_or("");

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let current_hash = format!("{:x}", hasher.finalize());

    // Integrity verification
    println!("\n{}", "🔐 Security Verification 🔐".bright_yellow().bold());
    println!("Verification Time (UTC): {}",
             Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().cyan());
    println!("System Username: {}\n", "Nichokas".cyan());

    if original_hash == current_hash {
        println!("{}", "✅ File integrity verified - No modifications detected".green());
    } else {
        println!("{}", "❌ WARNING: File integrity check failed!".red().bold());
        println!("Expected hash: {}", original_hash.yellow());
        println!("Current hash:  {}", current_hash.yellow());
        println!("\n{}", "The file may have been tampered with!".red().bold());
        
        eprintln!("{}", "Aborting decompression for security".red());
        std::process::exit(1);
    }

    // Get the original extension
    let ext = metadata
        .lines()
        .find(|line| line.starts_with("Original Extension: "))
        .and_then(|line| line.strip_prefix("Original Extension: "))
        .unwrap_or("unknown");

    let output_name = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("decoded");
    let output_file = format!("{}.{}", output_name, ext);

    if let Err(_) = fs::write(&output_file, data.as_bytes()) {
        eprintln!("{}", format!("Error: Could not write file '{}'", output_file).red());
        std::process::exit(1);
    }

    println!("\n{}", "✨ Decompression Result ✨".bright_green().bold());
    println!("Original file restored: {}", output_file.bright_blue());
    println!("{}", "Decompression completed successfully! ✅".green());
}

fn compress_file(input_file: &str) {
    let name: String = Path::new(&input_file.trim().to_string())
        .with_extension("yava")
        .to_str()
        .unwrap()
        .to_string();

    let original_ext = Path::new(input_file)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown");

    let data = match fs::read(input_file) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not read file '{}'", input_file).red());
            std::process::exit(1);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let metadata = format!(
        "Original Extension: {}\nCompressed Date: {}\nCompressed by: {}\n---BEGIN COMPRESSED DATA---\n",
        original_ext,
        current_time,
        hash
    );

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

    println!("{}", "\n📂 File Metadata Information 📂".bright_green().bold());
    println!("{}", "═".repeat(40).yellow());
    println!("{}: {}", "Original Extension".bright_blue(), original_ext.bright_white());
    println!("{}: {}", "Compressed Date".bright_blue(), current_time.bright_white());
    println!("{}: {}", "SHA-256 Hash".bright_blue(), hash.bright_white());
    println!("{}", "═".repeat(40).yellow());

    println!("\n{}", "✨ Compression Result ✨".bright_green().bold());
    println!("Compressed file created: {}", name.bright_blue());
    println!("{}", "Compression completed successfully! ✅".green());
}