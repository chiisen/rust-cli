# rust-cli
用 Rust 打造的第一个命令行工具

# Rust 升級
```shell
rustc -V

rustup update
```

## 指令說明
```shell
cargo run -- -h
```

```shell
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
```shell
cargo run -- -V
```

```shell
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -V`
rust-cli 0.1.0
```

## 顯示檔案內容
```shell
cargo run -- -f Cargo.toml
```

```shell
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
```shell
cargo run -- -n 168
```

```shell
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -n 168`
你的幸運數字是 175.
```

# 全域安裝
```shell
cargo install --path .
```

# 添加環境變數
```shell
export PATH="$HOME/.cargo/bin:$PATH"

source ~/.bashrc  # 或者 source ~/.zshrc

在 Windows 上
添加 C:\Users\你的用戶名\.cargo\bin 到 Path 變數中
```

# 全域執行
```shell
rust-cli --help
```

# 編譯程式
```shell
cargo build

# 優化版本
cargo build --release
```

# 執行編譯程式
```shell
.\target\debug\rust-cli -help

# 優化版本
.\target\release\rust-cli -help
```

# git commit message
- 常用描述
```shell
✨ feat: 需求功能描述
🐛 fix: 修正 bug 的問題描述
💄 optimize: 最佳化程式碼或功能流程
🔧 chore: 雜事，例如: 調整設定檔案等等 
```

