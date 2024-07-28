# rust-cli
用 Rust 打造的第一个命令行工具

## 指令說明
```
cargo run -- -h
```

```
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -h`
rust-cli 0.1.0
Sam
用 Rust 打造的第一个命令行工具

USAGE:
    rust-cli.exe [FILE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    File to print.

```

## 顯示版本號
```
cargo run -- -V
```

```
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -V`
rust-cli 0.1.0
```

## 顯示檔案內容
```
cargo run -- -f Cargo.toml
```

```
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
```
cargo run -- -n 168
```

```
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

