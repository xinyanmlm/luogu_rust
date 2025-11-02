use std::io;

fn main() ->Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let total: i32 = input
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .sum::<Result<i32, _>>()?;

    println!("{total}");
    Ok(())
}