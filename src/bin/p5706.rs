use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let (capacity, person): (f64, i32) = {
        let mut iter = input.split_whitespace();
        (iter.next().ok_or("")?.parse()?, iter.next().ok_or("")?.parse()?)
    };

    if person == 0 {
        return Ok(());
    }

    let per_cap = capacity / person as f64;
    let numbers = person * 2;

    println!("{:.*}\n{}", 3, per_cap, numbers);

    Ok(())
}