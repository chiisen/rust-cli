/// # 輸入數字加 7
/// ```
/// cargo run -- -n 168
/// ```
pub fn command_number(num_str: Option<&str>) {
    match num_str {
        None => println!("請輸入你喜歡的數字."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("你的幸運數字是 {}.", n + 7),
                Err(_) => println!("這不是數字喔! {}", s),
            }
        }
    }
}
