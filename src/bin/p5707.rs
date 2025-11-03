use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let s: u32 = iter.next().ok_or("")?.parse()?;
    let v: u32 = iter.next().ok_or("")?.parse()?;

    let t = s.div_ceil(v) + 10;
    let total_t = (8 * 60 - t as i32 + 24 * 60) % (24 * 60);

    println!("{:02}:{:02}", total_t / 60, total_t % 60);

    Ok(())
}