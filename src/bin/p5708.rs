use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let (a, b, c): (f64, f64, f64) = {
        let mut iter = input.split_whitespace();
        (
            iter.next().ok_or("")?.parse()?,
            iter.next().ok_or("")?.parse()?,
            iter.next().ok_or("")?.parse()?
        )
    };
    
    let p = (a + b + c) / 2.0;
    println!("{:.1}", (p*(p-a)*(p-b)*(p-c)).sqrt());

    Ok(())

}