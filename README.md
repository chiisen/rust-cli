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
cargo run -- -v
```

```
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rust-cli.exe -V`
rust-cli 0.1.0
```