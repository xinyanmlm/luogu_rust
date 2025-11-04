use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut numbers: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    numbers.sort();
    for number in &numbers {
        print!("{} ", number);
    }

    Ok(())
}