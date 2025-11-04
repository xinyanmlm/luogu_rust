use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: u8 = input.trim().parse()?;
    if input == 0 || input == 1 {
        println!("Today, I ate {} apple.", input);
    }else {
        println!("Today, I ate {} apples.", input);
    }

    Ok(())
}