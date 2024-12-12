# rust-cli
用 Rust 打造的第一个命令行工具

## 指令說明
```bash
cargo run -- -h
```

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\rust-cli.exe -h`
App: rust-cli Version: 1.1.0
Author: Sam (@chiisen)
About: 用 Rust 打造的第一个命令行工具

USAGE:
    rust-cli.exe [OPTIONS]
****

FLAGS:
    -h, --help       Prints help information
OPTIONS:--version    Prints version information
    -e, --example <EXAMPLE>    cargo run -- -e 1 => 輸入數字加 1.
    -f, --file <FILE>          cargo run -- -f Cargo.toml => 印出檔案內容.
    -n, --number <NUMBER>      cargo run -- -n 168 => 輸入數字加 7.

```

## 顯示版本號
```bash
cargo run -- -V
```

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -V`
rust-cli 0.1.0
```

## 範例
```bash
cargo run -- -e 1
```

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\rust-cli.exe -e 1`
你的幸運數字是 2.
```

## 顯示檔案內容
```bash
cargo run -- -f Cargo.toml
```

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -f Cargo.toml`
【Cargo.toml】 : 檔案存在!!
[package]
name = "rust-cli"
version = "1.1.0"
edition = "2021"
authors = ["Sam (@chiisen)"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = "~2.32"
```

## 輸入數字加7
```bash
cargo run -- -n 168
```

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -n 168`
你的幸運數字是 175.
```

# git commit message
- 常用描述
```
✨ feat: 需求功能描述
🐛 fix: 修正 bug 的問題描述
💄 optimize: 最佳化程式碼或功能流程
🔧 chore: 雜事，例如: 調整設定檔案等等 
```

