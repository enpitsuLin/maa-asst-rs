# mma-sys

![MaaAssistantArknights](https://img.shields.io/github/v/tag/MaaAssistantArknights/MaaAssistantArknights?filter=v5.16.10&logo=github&label=MAA&link=https%3A%2F%2Fgithub.com%2FMaaAssistantArknights%2FMaaAssistantArknights)


全新的 [MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights) 的 Rust 绑定。


## 使用


```toml
[dependencies]
maa-sys = { git = "https://github.com/enpitsuLin/maa-asst-rs.git" }
```

> [!NOTE]
> 目前仍在开发中，使用时注意风险。

## 构建 

1. 首先需要先构建 [MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights) 获得一系列静态链接库文件，或者从对应平台已经编译好的产物获取。
2. 获取 `AsstCaller.h` 文件，并将其放置在 `crates/maa-sys/include` 目录下。
3. 然后将对应内容提供给 `MAA_LIB_PATH` 和 `MAA_HEADER_PATH` 环境变量，分别指向构建产物`MaaCore.dll`所在的目录和 [`AsstCaller.h`](https://github.com/MaaAssistantArknights/MaaAssistantArknights/blob/dev/include/AsstCaller.h) 的路径。
4. 执行 `cargo build` 构建。


