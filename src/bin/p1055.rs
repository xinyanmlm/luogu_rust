use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let isbn: Vec<char> = input.trim().chars().collect();

    let mut result: u32 = 0;
    let mut count = 0;
    for (index, ch) in isbn.iter().enumerate() {
        if index == 1 || index == 5 || index == 11 {
            continue;
        } else if index == 12 {
            result %= 11;
            break;
        } else {
            if let Some(digit) = ch.to_digit(10) {
                result += digit * (count + 1);
            }
            count += 1;
        }
    }

    let checksum: char = if result == 10 {'X'} else {
        char::from_digit(result, 10).ok_or("")?
    };

    if checksum == isbn[12] {
        print!("Right");
    } else {
        for (index, ch) in isbn.iter().enumerate() {
            if index < 12 {
                print!("{}", ch);
            }
            else {
                print!("{}", checksum);
            }
        }
    }

    println!();

    Ok(())
}