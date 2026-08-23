use reqwest::blocking::get;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
 
fn main() {
    println!("🌐 Parallel Web Crawler");
 
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.google.com",
        "https://docs.rs",
        "https://httpbin.org/delay/1",
        "https://thisurldoesnotexist.com", // intentionally fails
    ];
 
    let pool_size = 4;
    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
 
    for i in 0..pool_size {
        let rx = Arc::clone(&rx);
        thread::spawn(move || loop {
            let url = match rx.lock().unwrap().recv() {
                Ok(u) => u,
                Err(_) => break,
            };
            println!("🧵 Worker {} fetching: {}", i, url);
            match fetch_url(&url) {
                Ok(status) => println!("✅ {} => {}", url, status),
                Err(e) => println!("❌ {} => {}", url, e),
            }
        });
    }
 
    for url in urls {
        tx.send(url.to_string()).unwrap();
    }
 
    // Give threads time to finish
    thread::sleep(Duration::from_secs(5));
}
 
fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    Ok(format!("Status: {}", response.status()))
}