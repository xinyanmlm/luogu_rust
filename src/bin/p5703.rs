use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    let product = input[0] * input[1];
    println!("{}", product);

    Ok(())
    
}