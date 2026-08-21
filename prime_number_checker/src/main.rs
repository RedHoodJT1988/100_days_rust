use std::io;

fn main() {
    println!("🔢 Prime Number Checker");
    println!("Enter a positive integer to check if it's prime:");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("❌ Invalid input. Please enter a positive integer.");
            return;
        }
    };

    if number <= 1 {
        println!("❌ The number must be greater than 1.");
        return;
    }

    if is_prime(number) {
        println!("✅ {} is a prime number.", number);
    } else {
        println!("❌ {} is not a prime number.", number);
    }
}

/// Reads user input and attempts to parse it as u32
fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

/// Checks if a number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true; // 2 is the only even prime number
    }
    if n % 2 == 0 {
        return false; // Eliminate even numbers greater than 2
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}