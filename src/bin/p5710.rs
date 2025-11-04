use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: u32 = input.trim().parse()?;

    let condition1: bool = input % 2 == 0;
    let condition2: bool = input > 4 && input <= 12;

    let p1 = (condition1 && condition2) as u8;
    let p2 = (condition1 || condition2) as u8;
    let p3 = (condition1 != condition2) as u8;
    let p4 = (!condition1 && !condition2) as u8;

    println!("{} {} {} {}", p1, p2, p3, p4);

    Ok(())
}