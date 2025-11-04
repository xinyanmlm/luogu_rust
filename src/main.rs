fn main() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let nan = f64::NAN;
    
    // total_cmp 提供总排序，包括 NaN
    println!("a < b: {}", a.total_cmp(&b).is_lt());
    println!("a == b: {}", a.total_cmp(&b).is_eq());
    // println!("a < b: {:?}", a.total_cmp(&b));
    // println!("a == b: {:?}", a.total_cmp(&b));
    
    // NaN 可以与其他值比较（NaN 被视为最大）
    println!("nan cmp 1.0: {:?}", nan.total_cmp(&1.0));
}