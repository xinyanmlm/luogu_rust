use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: u16 = input.trim().parse()?;

    let time_local = 5 * input;
    let time_luogu = 11 + 3 * input;
    if time_local <= time_luogu {println!("Local");} else {println!("Luogu");}

    Ok(())
}