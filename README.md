# cargo_egui

## [官网](https://www.rust-lang.org/zh-CN/)

- [下载页面](https://www.rust-lang.org/zh-CN/learn/get-started)

- Linux 安装

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 或
# curl -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
# 或 vim ~/.bash_profile
# 添加如下内容
# export PATH="$HOME/.cargo/bin:$PATH" 
```

## 命令

- rust 版本

```shell
rustc --version
```

- rust 更新

```shell
rustup update
```

- rustc 编译

```shell
rustc main.rs
```

- cargo 版本

```shell
cargo --version
```

- cargo 创建项目

```shell
cargo new cargo_egui
```

- cargo 构建(编译)

```shell
cargo build
```

- cargo 在优化模式下构建(编译)并生成可执行文件

```shell
cargo build --release
```

- cargo 构建(编译)、运行

```shell
cargo run
```

- 快速检查当前的代码是否可以通过编译，而不需要花费额外的时间去真正生成可执行程序

```shell
cargo check
```

- 文档

```shell
rustup doc
```
