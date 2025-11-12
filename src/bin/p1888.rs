use std::io;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut numbers: Vec<u32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    
    numbers.sort();
    let div_num = gcd(numbers[0], numbers[2]);

    println!("{}/{}", numbers[0] / div_num, numbers[2] / div_num);

    Ok(())
}