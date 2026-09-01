use std::fs::File;
use std::io::{self, Read};
use std::env;
 
fn main() {
    println!("🔍 Binary File Parser (Hex Dump)");
 
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
 
    match parse_binary_file(&args[1]) {
        Ok(_) => println!("✅ Done."),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
 
fn parse_binary_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 16];
    let mut offset = 0;
 
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
 
        print!("{:08x}: ", offset);
 
        for i in 0..16 {
            if i < bytes_read {
                print!("{:02x} ", buffer[i]);
            } else {
                print!("   ");
            }
            if i == 7 {
                print!(" "); // extra space in the middle
            }
        }
 
        print!("|");
 
        for i in 0..bytes_read {
            let c = buffer[i];
            let display = if c.is_ascii_graphic() || c == b' ' { c as char } else { '.' };
            print!("{}", display);
        }
 
        println!("|");
        offset += bytes_read;
    }
 
    Ok(())
}