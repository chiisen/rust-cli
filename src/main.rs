extern crate clap;

use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read};

/// # Rust CLI
/// 程式碼進入點
/// ```
/// 打造的第一个命令行工具
/// ```
fn main() {
    // Cargo.toml 的 package name
    const APP_NAME: &str = env!("CARGO_PKG_NAME");
    // Cargo.toml 的 package version
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
    // Cargo.toml 的 package authors
    const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

    let matches = App::new(format!("App: {APP_NAME}"))
        .version(&*format!("Version: {APP_VERSION}"))
        .author(&*format!("Author: {APP_AUTHOR}"))
        .about("About: 用 Rust 打造的第一个命令行工具")
        .arg(Arg::with_name("FILE")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("印出檔案內容.")
            .empty_values(false)
        )
        .arg(Arg::with_name("NUMBER")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("輸入數字加 7."))
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            println!("【{file}】 : 檔案存在!!");
            let mut f = File::open(file).expect("[rust-cli Error] 檔案找不到.");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("[rust-cli Error] 無法讀取檔案.");
            println!("{}", data);
        }
        else {
            eprintln!("[rust-cli Error] 檔案不存在.");
            process::exit(1);// 程式錯誤終止時的標準退出碼
        }
    }

    let num_str = matches.value_of("NUMBER");
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
