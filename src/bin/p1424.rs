use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let x: u32 = iter.next().ok_or("")?.parse()?;
    let n: u32 = iter.next().ok_or("")?.parse()?;

    let head_week: u32 = 7 - x + 1;
    let head_days = if head_week >= 2 {
        head_week - 2
    } else { 0 };

    let whole_weeks: u32 = (n - head_week) / 7;

    let tail_week: u32 = (n - head_week) % 7;
    let tail_days = if tail_week > 5 {
        5
    } else { tail_week };

    let count: u32 = (head_days + whole_weeks * 5 + tail_days) * 250;
    println!("{}", count);

    Ok(())
}