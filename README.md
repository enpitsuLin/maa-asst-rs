# ZOOT MAA 助手

<small>ZEROTH ORDER OIL TANK</small>

## 简介

是一个一个 MAA 助手的 GUI 实现，使用 [GPUI](https://github.com/longbridge/gpui) 框架构建，支持跨平台。

> [!NOTE]
> 该项目处于实验阶段，开发尚处于非常早期的阶段。
> GPUI 仍然与 [Zed 编辑器](https://zed.dev/)关联，因此其文档记录不全，而且 API 中经常发生重大变更，可能导致行为发生变化。尽管如此，我还是选择 GPUI 作为 GUI 框架，以便学习 GPUI 并在 Zed 以外的代码库中进行测试。而且，这是一个非常棒的框架，一定要尝试一下！

## 目录结构

```text
├── app/
│   ├─── auto_update # 自动更新相关
│   ├─── global      # 全局常量和一些状态管理
│   ├─── settings    # 设置相关
│   ├─── zoot        # ZOOT MAA 助手主程序包
├─── crates/*        # MAA 的 rust 绑定相关库
├─── xtask           # 构建和发布相关的 xtask 任务
│
```

# License

MIT
