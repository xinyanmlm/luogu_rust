// 将输出逻辑提取到函数中以便测试
fn get_hello_world() -> String {
    "Hello,World!".to_string()
}

fn main() {
    println!("{}", get_hello_world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_runs_without_panic() {
        // 最简单的测试：确保main函数不会panic
        main();
    }

    #[test]
    fn test_hello_world_output() {
        // 如果要测试具体的输出逻辑，可以提取到函数中
        assert_eq!(get_hello_world(), "Hello,World!");
    }
}