use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let y: u16 = iter.next().ok_or("")?.parse()?;
    let m: u8 = iter.next().ok_or("")?.parse()?;

    const MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if (y % 400 == 0 || y % 4 == 0 && y % 100 != 0) && m == 2 {
        println!("29");
    } else {
        println!("{}", MONTH[m as usize - 1]);
    }

    Ok(())
}