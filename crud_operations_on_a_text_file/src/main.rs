use std::fs::{self, OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};
 
const FILE_PATH: &str = "notes.txt";
 
fn main() {
    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");
 
        match input("Choose an option: ").as_str() {
            "1" => {
                let content = input("Enter new content: ");
                fs::write(FILE_PATH, content + "\n").expect("❌ Failed to write");
                println!("✅ File overwritten.");
            }
            "2" => {
                if let Ok(file) = File::open(FILE_PATH) {
                    println!("📄 File Content:");
                    for (i, line) in BufReader::new(file).lines().enumerate() {
                        println!("{}: {}", i + 1, line.unwrap());
                    }
                } else {
                    println!("📭 File not found.");
                }
            }
            "3" => {
                let line_no = input("Line to update: ").parse::<usize>().unwrap_or(0);
                let new_text = input("New content: ");
                update_line(line_no, &new_text);
            }
            "4" => {
                let line_no = input("Line to delete: ").parse::<usize>().unwrap_or(0);
                delete_line(line_no);
            }
            "5" => break,
            _ => println!("❌ Invalid choice."),
        }
    }
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
 
fn update_line(line_no: usize, new_text: &str) {
    let file = OpenOptions::new().read(true).open(FILE_PATH);
    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();
 
        if line_no > 0 && line_no <= lines.len() {
            lines[line_no - 1] = new_text.to_string();
            fs::write(FILE_PATH, lines.join("\n") + "\n").expect("❌ Write failed");
            println!("✅ Line {} updated.", line_no);
        } else {
            println!("❌ Invalid line number.");
        }
    }
}
 
fn delete_line(line_no: usize) {
    let file = OpenOptions::new().read(true).open(FILE_PATH);
    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();
 
        if line_no > 0 && line_no <= lines.len() {
            lines.remove(line_no - 1);
            fs::write(FILE_PATH, lines.join("\n") + "\n").expect("❌ Write failed");
            println!("✅ Line {} deleted.", line_no);
        } else {
            println!("❌ Invalid line number.");
        }
    }
}