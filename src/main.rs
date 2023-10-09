extern crate clap;

use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read};

fn main() {
    let matches = App::new("rust-cli")
        .version("1.0.1")
        .author("Sam")
        .about("用 Rust 打造的第一个命令行工具")
        .arg(Arg::with_name("FILE")
            .help("File to print.")
            .empty_values(false)
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            println!("File exist!!");
            let mut f = File::open(file).expect("[rust-cli Error] File not found.");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("[rust-cli Error] Unable to read the  file.");
            println!("{}", data);
        }
        else {
            eprintln!("[rust-cli Error] No such file or directory.");
            process::exit(1);
        }
    }
}
