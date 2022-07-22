# mma-asst-rs

[MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights)的 rust 绑定

## 安装

本库未发布到`crates.io`,请在 `Cargo.toml` 中添加

```toml
[dependencies]
maa = { git = "https://github.com/enpitsuLin/maa-asst-rs" }
```

## 用法

见`example/*`下的文件

## 注意事项

此库以及引用此库构建需要自行编译 MeoAssistant 的链接库文件(\*.lib,\*.dll),并设置环境变量`MAA_LIB_PATH`为其路径。

构建后产物需要与`MeoAssistant.dll`在同一目录,且其依赖的其他库文件应该均可以链接到。~~反正少东西会有报错提示，建议直接丢 MAA 助手文件夹底下就完事，以后可能自动将所需依赖添加到构建脚本里处理~~

# License

MIT
