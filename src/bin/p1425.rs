use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut iter = input.split_whitespace();
    let a: u16 = iter.next().ok_or("")?.parse()?;
    let b: u16 = iter.next().ok_or("")?.parse()?;
    let c: u16 = iter.next().ok_or("")?.parse()?;
    let d: u16 = iter.next().ok_or("")?.parse()?;

    let total_t = c * 60 + d - (a * 60 + b);
    println!("{} {}", total_t / 60, total_t % 60);

    Ok(())
}