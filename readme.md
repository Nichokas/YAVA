# YAVA (Yeah Another Verification App)

YAVA is a command-line application written in Rust that provides secure file compression with integrity verification through SHA-256 checksums.

## Installation

To install YAVA, run the following command:

```bash
 TODO: bash script for installing the binary
```

## Usage

```bash
# To compress a file
yava <filename>

# To decompress a .yava file
yava <filename.yava>
```

## Features

- File compression with metadata preservation
- SHA-256 checksum verification
- Creation date tracking
- Original file extension preservation
- Secure decompression with integrity checks

## How It Works

### Compression
When compressing a file, YAVA:
1. Calculates the SHA-256 hash of the file content
2. Records the current timestamp and original file extension
3. Compresses the file using XZ compression
4. Saves the compressed data along with metadata in a `.yava` file

### Decompression
When decompressing a `.yava` file, YAVA:
1. Extracts the metadata (original hash, date, extension)
2. Calculates a new SHA-256 hash of the compressed content
3. Verifies the integrity by comparing the original and new hashes
4. If verification passes, decompresses the file to its original format


## Security Features

    SHA-256 hash verification ensures file integrity
    Timestamp tracking for compression date
    Automatic integrity checks during decompression
    Clear security verification output

## Technical Details

    Written in Rust
    Uses XZ compression (via xz2 crate)
    SHA-256 for checksum verification
    Preserves original file metadata

### Requirements

    Rust (latest stable version)
    Compatible with Windows, Linux, and macOS

# Building from Source
```bash
git clone https://github.com/Nichokas/YAVA.git
cd YAVA
cargo build --release
```