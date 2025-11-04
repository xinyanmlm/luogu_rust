use std::io;

fn judge_year(year: u16) -> u8 {
    let cond1 = year % 4 == 0 && year % 100 != 0;
    let cond2 = year % 100 == 0 && year % 400 == 0;
    (cond1 || cond2) as u8
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: u16 = input.trim().parse()?;

    println!("{}", judge_year(input));

    Ok(())
}