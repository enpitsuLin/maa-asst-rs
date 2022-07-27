# mma-asst-rs

[![Latest Version](https://img.shields.io/crates/v/maa.svg)](https://crates.io/crates/maa)
[![dependency status](https://deps.rs/repo/github/enpitsuLin/maa-asst-rs/status.svg)](https://deps.rs/repo/github/enpitsuLin/maa-asst-rs)

[MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights)的 rust 绑定，提供一系列 rust 语法友好的封装。

## 用法

本库已发布到[`crates.io`](https://crates.io/crates/maa),请在 `Cargo.toml` 中添加

```toml
[dependencies]
maa = "*"
```

详细用法见`example/*`下的文件

## 构建

此库以及引用此库构建需要自行编译 MeoAssistant 的链接库文件(\*.lib,\*.dll),并设置环境变量`MAA_LIB_PATH`为其路径。

~~构建后产物需要与`MeoAssistant.dll`在同一目录,且其依赖的其他库文件应该均可以链接到。反正少东西会有报错提示，建议直接丢 MAA 助手文件夹底下就完事，以后可能自动将所需依赖添加到构建脚本里处理~~

现在已经会将链接库文件自动拷贝到 Cargo 构建目标路径

# License

MIT
