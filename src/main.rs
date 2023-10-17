extern crate clap;


use clap::{Arg, App};


mod command
{
    #[path = "file.rs"]
    pub mod file;
    // 中間要空一行
    #[path = "number.rs"]
    pub mod number;
}


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
        command::file::command_file(file);
    }

    if let Some(num_str) = matches.value_of("NUMBER"){
        command::number::command_number(Option::from(num_str));
    }
}

