extern crate clap;

use clap::{Arg, App};

fn main() {
    let _matches = App::new("rust-cli")
        .version("1.0.1")
        .author("Sam")
        .about("用 Rust 打造的第一个命令行工具")
        .arg(Arg::with_name("FILE")
            .help("File to print.")
            .empty_values(false)
        )
        .get_matches();

    if let Some(file) = _matches.value_of("FILE") {
        println!("Value for file argument: {}", file);
    }
}
