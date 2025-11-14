fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n: u32 = iter.next().ok_or("")?.parse()?;
    let k: u32 = iter.next().ok_or("")?.parse()?;

    let mut could_div_sum: u32 = 0;
    let mut could_div_sum_count: u32 = 0;
    let mut cannt_div_sum: u32 = 0;
    for num in 1..=n {
        if num % k == 0 {
            could_div_sum += num;
            could_div_sum_count += 1;
        } else {
            cannt_div_sum += num;
        }
    }

    println!("{:.1} {:.1}", 
        could_div_sum as f64 / could_div_sum_count as f64,
        cannt_div_sum as f64 / (n - could_div_sum_count) as f64
    );

    Ok(())
}