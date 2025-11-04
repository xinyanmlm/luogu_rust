use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();

    let a: u16 = iter.next().ok_or("")?.parse()?;
    let b: u16 = iter.next().ok_or("")?.parse()?;
    let c: u16 = iter.next().ok_or("")?.parse()?;

    println!("{}", (a * 20 + b * 30 + c * 50) / 100);

    Ok(())
}