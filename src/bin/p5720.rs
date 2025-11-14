fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut a: u32 = input.trim().parse()?;

    let mut count: u32 = 0;
    while a > 1 {
        a /= 2;
        count += 1;
    }
    println!("{}", count + 1);

    Ok(())
}