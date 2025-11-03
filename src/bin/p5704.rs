use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char)?;

    println!("{}", input_char.trim().to_ascii_uppercase());
    
    Ok(())
}