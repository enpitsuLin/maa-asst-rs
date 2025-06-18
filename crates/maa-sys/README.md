# mma-sys

![MaaAssistantArknights](https://img.shields.io/github/v/tag/MaaAssistantArknights/MaaAssistantArknights?filter=v5.16.10&logo=github&label=MAA&link=https%3A%2F%2Fgithub.com%2FMaaAssistantArknights%2FMaaAssistantArknights)


全新的 [MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights) 的 Rust 绑定。


## 使用

> [!NOTE]
> 目前仍在开发中，使用时注意风险。


```toml
[dependencies]
maa-sys = { git = "https://github.com/enpitsuLin/maa-asst-rs.git" }
```

### 创建 MAA 助手实例

```rust
use maa_sys::Assistant;

/// 使用 MAA 助手资源目录作为 init 参数, 资源目录应该包括 `/resource` 目录和对应的运行库文件
let assistant = Assistant::init("MAA_RESOURCE_PATH").unwrap();

/// 同上但是通过 callback 参数传入回调函数, 回调函数需要实现 FnMut(Message, serde_json::Value) + Send + 'static 
let assistant = Assistant::init_with_callback("MAA_RESOURCE_PATH", |msg, details| {
    println!("msg: {:?}, details: {:?}", msg, details);
});

/// 通过 builder 模式分别设置 library 和 resource 路径以及回调函数, 最后调用 init 方法创建实例
let assistant = Assistant::registry()
    .with_library("MAA_LIB_PATH")
    .with_resource("MAA_RESOURCE_PATH")
    .with_callback(|msg, details| {
        println!("msg: {:?}, details: {:?}", msg, details);
    })
    .init()
    .unwrap();
```


## 构建 

1. 通过 [MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights) 获取 `AsstCaller.h` 文件，并设置 `MAA_HEADER_PATH` 为文件路径。
2. 执行 `cargo build` 构建。

## 运行测试
 
设置 `MAA_RESOURCE_PATH` 为 MAA 助手资源所在目录 （resource 文件夹所在的目录）。

执行 `cargo test` 运行测试。




