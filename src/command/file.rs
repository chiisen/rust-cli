use std::process;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

/// # 顯示檔案內容
/// ```
/// cargo run -- -f "路徑與檔名"
/// ```
pub fn command_file(file: &str) {
    if Path::new(&file).exists() {
        println!("【{file}】 : 檔案存在!!");
        let mut f = File::open(file).expect("[rust-cli Error] 檔案找不到.");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("[rust-cli Error] 無法讀取檔案.");
        println!("{}", data);
    } else {
        eprintln!("[rust-cli Error] 檔案不存在.");
        process::exit(1);// 程式錯誤終止時的標準退出碼
    }
}