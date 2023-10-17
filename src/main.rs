extern crate clap;

use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read};

fn main() {
    let app_name = "rust-cli";

    let matches = App::new(format!("App: {app_name}"))
        .version("Version: 1.0.1")
        .author("Author: Sam")
        .about("About: 用 Rust 打造的第一个命令行工具")
        .arg(Arg::with_name("FILE")
            .help("印出檔案內容.")
            .empty_values(false)
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            println!("{file} : 檔案存在!!");
            let mut f = File::open(file).expect("[rust-cli Error] 檔案找不到.");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("[rust-cli Error] 無法讀取檔案.");
            println!("{}", data);
        }
        else {
            eprintln!("[rust-cli Error] 檔案不存在.");
            process::exit(1);// 程序错误终止时的标准退出码
        }
    }
}
