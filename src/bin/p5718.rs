use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = io::stdin().lock().lines();
    let _ = lines.next();
    let numbers = lines.next().ok_or("")??
        .split_whitespace()
        .filter_map(|x| x.parse::<u16>().ok())
        .min().ok_or("")?;

    println!("{}", numbers);

    Ok(())
}