use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut numbers: Vec<u16> = lines
        .next()
        .ok_or("")??
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let ordor: Vec<char> = lines
        .next()
        .ok_or("")??
        .chars()
        .collect();
    
    numbers.sort();
    for ch in &ordor {
        match ch {
            'A' => print!("{} ", numbers[0]),
            'B' => print!("{} ", numbers[1]),
            _ => print!("{} ", numbers[2]),
        }
    }
    println!("");

    Ok(())
}