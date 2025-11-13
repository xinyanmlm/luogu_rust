use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let app_h: Vec<u16> = lines
        .next()
        .ok_or("")??
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let max_h: u16 = lines
        .next()
        .ok_or("")??
        .trim()
        .parse()?;

    let num = app_h.iter().filter(|&&height_i| height_i <= max_h + 30).count();

    println!("{}", num);
    
    Ok(())
}