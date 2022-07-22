# MMA-Asst-rs

[MMA 助手](https://github.com/MaaAssistantArknights/MaaAssistantArknights)的 rust 绑定

## 注意事项

此库包编译需要自行构建 MeoAssistant 的链接库文件(\*.lib,\*.dll),并设置环境变量`MAA_LIB_PATH`为构建产物路径。

构建后产物需要与`MeoAssistant.dll`在同一目录,且其依赖的库文件路径应该均可以链接到。~~反正少东西会有报错提示~~
