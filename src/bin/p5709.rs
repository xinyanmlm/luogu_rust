use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let apples: u32 = iter.next().ok_or("")?.parse()?;
    let per_time: u32 = iter.next().ok_or("")?.parse()?;
    let past_time: u32 = iter.next().ok_or("")?.parse()?;

    if per_time == 0 {
        println!("{}", 0);
        return Ok(());
    }

    let eat = (past_time / per_time).min(apples);
    let remainer = apples - eat;

    if remainer > 0 && past_time % per_time > 0 {
        println!("{}", remainer - 1);
    } else {
        println!("{}", remainer)
    }

    Ok(())
}