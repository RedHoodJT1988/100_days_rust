use std::cmp::Ordering;
use std::io::{self, Write};
 
fn main() {
    println!("🔍 Generic Binary Search CLI");
 
    let numbers = vec![1, 3, 5, 7, 9, 11, 13];
    let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];
 
    println!("\n1. Search Numbers");
    println!("2. Search Words");
    let mode = input("Choose list type: ");
 
    match mode.as_str() {
        "1" => {
            let query = input("Enter a number to search: ");
            if let Ok(q) = query.parse::<i32>() {
                match binary_search(&numbers, &q) {
                    Some(idx) => println!("✅ Found {} at index {}", q, idx),
                    None => println!("❌ Not found."),
                }
            } else {
                println!("❌ Invalid number.");
            }
        }
        "2" => {
            let query = input("Enter a word to search: ");
            match binary_search(&words, &query.as_str()) {
                Some(idx) => println!("✅ Found '{}' at index {}", query, idx),
                None => println!("❌ Not found."),
            }
        }
        _ => println!("❌ Invalid choice."),
    }
}
 
/// Generic binary search for sorted slices
fn binary_search<T: PartialOrd>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();
 
    while low < high {
        let mid = (low + high) / 2;
        match list[mid].partial_cmp(target).unwrap() {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
 
    None
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}