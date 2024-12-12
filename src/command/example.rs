/// # 輸入數字加 1
/// ```
/// cargo run -- -e 1
/// ```
pub fn command_example(num_str: Option<&str>) {
    match num_str {
        None => println!("請輸入你喜歡的數字."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("你的幸運數字是 {}.", n + 1),
                Err(_) => println!("這不是數字喔! {}", s),
            }
        }
    }
}