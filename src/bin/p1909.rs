use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut min_money = u32::MAX;

    let n: u32 = lines.next().ok_or("")??.trim().parse()?;
    for line in lines.take(3) {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let per_num: u32 = parts[0].parse()?;
        let price: u32 = parts[1].parse()?;
        let money = n.div_ceil(per_num) * price;
        if money < min_money {
            min_money = money;
        }
    }

    println!("{}", min_money);

    Ok(())
}