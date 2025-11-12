use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut max_unhappy = 0;
    let mut unhappy_day = 0;

    for (index, line) in lines.enumerate().take(7) {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let hour1: u8 = parts[0].parse()?;
        let hour2: u8 = parts[1].parse()?;
        if hour1 + hour2 > 8 as u8 && max_unhappy < hour1 + hour2 {
            max_unhappy = hour1 + hour2;
            unhappy_day = index + 1;
        }
    }
    println!("{}", unhappy_day);

    Ok(())
}