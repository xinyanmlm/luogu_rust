fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n: u8 = input.trim().parse()?;

    let mut num: u8 = 1;
    for row in 1..=n {
        for col in 1..=(n - row + 1){
            print!("{:02}", num);
            num += 1;
        }
        println!();
    }
    Ok(())
}