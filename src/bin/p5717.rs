use std::io;

fn judge(a: u32, b: u32, c: u32) {
    if a + b <= c {
        println!("Not triangle");
        return;
    }

    if a*a + b*b == c*c {
        println!("Right triangle");
    }else if a*a + b*b > c*c {
        println!("Acute triangle");
    }else if a*a + b*b < c*c {
        println!("Obtuse triangle");
    }

    if a == b {
        println!("Isosceles triangle");
    }
    if a == c {
        println!("Equilateral triangle");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut lengths: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    lengths.sort();
    judge(lengths[0], lengths[1], lengths[2]);

    Ok(())
}