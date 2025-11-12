use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: u32 = input.trim().parse()?;
    
    let result: f64 = if count <= 150 {
        count as f64 * 0.4463
    }else if count <= 400 {
        150.0 * 0.4463 + (count - 150) as f64 * 0.4663
    }else {
        150.0 * 0.4463 + 250.0 * 0.4663 + (count - 400) as f64 * 0.5663
    };
    
    println!("{:.1}", result);

    Ok(())
}