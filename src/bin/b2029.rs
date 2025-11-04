use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();

    let h: u32 = iter.next().ok_or("")?.parse()?;
    let r: u32 = iter.next().ok_or("")?.parse()?;

    let capacity: f64 = 3.14 * (r * r * h) as f64;
    let count = (20000.0 / capacity).ceil() as u32;
    println!("{}", count);

    Ok(())
}