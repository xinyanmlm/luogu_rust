use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let m: f64 = iter.next().ok_or("")?.parse()?;
    let h: f64 = iter.next().ok_or("")?.parse()?;
    
    let bmi = m / (h * h);
    if bmi.total_cmp(&18.5).is_lt() {
        println!("Underweight");
    } else if bmi.total_cmp(&18.5).is_ge() && bmi.total_cmp(&24.0).is_lt() {
        println!("Normal");
    } else {
        println!("{:.4}\nOverweight", bmi);
    }

    Ok(())
}