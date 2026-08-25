use aes::Aes256;
use cipher::{KeyIvInit, StreamCipher};
use std::fs;
use std::io::{self, Write};

type Aes256Ctr = ctr::Ctr128BE<Aes256>;

const IV: &[u8; 16] = b"uniqueinitvector";

fn main() {
    println!("🔐 File Encryption Tool");
    println!("1. Encrypt file");
    println!("2. Decrypt file");

    let choice = prompt("Choose an option: ");
    match choice.as_str() {
        "1" => {
            let file = prompt("Enter path to file to encrypt: ");
            let key = prompt("Enter 32-byte key (hex): ");
            let out = prompt("Output file path: ");
            encrypt_file(&file, &key, &out).unwrap_or_else(|e| eprintln!("❌ Error: {}", e));
        }
        "2" => {
            let file = prompt("Enter path to file to decrypt: ");
            let key = prompt("Enter 32-byte key (hex): ");
            let out = prompt("Output file path: ");
            decrypt_file(&file, &key, &out).unwrap_or_else(|e| eprintln!("❌ Error: {}", e));
        }
        _ => println!("❌ Invalid option."),
    }
}

fn encrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()> {
    let key = hex::decode(key_hex).expect("Invalid hex key");
    let mut data = fs::read(path)?;

    // Initialize the cipher
    let mut cipher = Aes256Ctr::new_from_slices(&key, IV).unwrap();
    cipher.apply_keystream(&mut data);

    fs::write(out_path, data)?;
    println!("✅ File encrypted to '{}'", out_path);
    Ok(())
}

fn decrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()> {
    // Because AES-CTR is a symmetric stream cipher, encryption and decryption
    // are the exact same operation (applying the keystream to the data).
    encrypt_file(path, key_hex, out_path) 
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}