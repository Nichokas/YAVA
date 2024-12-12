use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use sha2::Digest;
use colored::*;

fn modify_hash(input_file: &str, new_hash: Option<String>) {
    // Leer y descomprimir el archivo
    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not open file '{}'", input_file).red());
            std::process::exit(1);
        }
    };

    let mut decoder = XzDecoder::new(file);
    let mut content = String::new();
    if decoder.read_to_string(&mut content).is_err() {
        eprintln!("{}", "Error: Failed to decode file".red());
        std::process::exit(1);
    }

    // Separar metadata y datos
    let parts: Vec<&str> = content.split("---BEGIN COMPRESSED DATA---\n").collect();
    if parts.len() != 2 {
        eprintln!("{}", "Error: Invalid file format".red());
        std::process::exit(1);
    }

    let metadata = parts[0];
    let data = parts[1];

    // Generar o usar el nuevo hash proporcionado
    let new_hash = new_hash.unwrap_or_else(|| {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..64).map(|_| {
            let n: u8 = rng.gen_range(0..16);
            format!("{:x}", n)
        }).collect()
    });

    // Crear nueva metadata
    let new_metadata = metadata.lines()
        .map(|line| {
            if line.starts_with("Compressed by: ") {
                format!("Compressed by: {}", new_hash)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Crear nombre para el archivo modificado
    let path = Path::new(input_file);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let output_file = format!("{}_modified.yava", stem);

    // Crear y comprimir el nuevo archivo
    let file = match File::create(&output_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{}", format!("Error: Could not create file '{}'", output_file).red());
            std::process::exit(1);
        }
    };

    let mut encoder = XzEncoder::new(file, 6);

    // Escribir la nueva metadata y datos
    if encoder.write_all(format!("{}---BEGIN COMPRESSED DATA---\n", new_metadata).as_bytes()).is_err() {
        eprintln!("{}", "Error: Failed to write metadata".red());
        std::process::exit(1);
    }

    if encoder.write_all(data.as_bytes()).is_err() {
        eprintln!("{}", "Error: Failed to write data".red());
        std::process::exit(1);
    }

    if encoder.finish().is_err() {
        eprintln!("{}", "Error: Failed to finish compression".red());
        std::process::exit(1);
    }

    // Mostrar información
    println!("{}", "\n🔧 Hash Modification Complete 🔧".bright_green().bold());
    println!("{}", "═".repeat(50).yellow());
    println!("Original file: {}", input_file.bright_blue());
    println!("Modified file: {}", output_file.bright_blue());
    println!("\nOriginal hash: {}", metadata.lines()
        .find(|line| line.starts_with("Compressed by: "))
        .and_then(|line| line.strip_prefix("Compressed by: "))
        .unwrap_or("").yellow());
    println!("New hash: {}", new_hash.yellow());
    println!("{}", "═".repeat(50).yellow());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Usage: program <file.yava> [new_hash]".red());
        std::process::exit(1);
    }

    let input_file = &args[1];
    let new_hash = args.get(2).cloned();

    modify_hash(input_file, new_hash);
}