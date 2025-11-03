use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char)?;
    let input_char = input_char.trim();
    
    for i in 0..3 {
        println!("{:^5}", input_char.repeat(2*i+1));
    }

    Ok(())
}