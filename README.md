# egui_desktop_widgets

面向 egui 和 eframe 的桌面应用组件库与 UI 模版仓库，目标是为典型 CAD 建模类应用提供可复用的常用 widget、布局模式和应用框架。

当前阶段先定义技术规范与开发指导，作为后续 AI coding agent 和人工开发协作的统一依据。

## 文档入口

- [技术规范](./docs/technical-spec.md)
- [开发指导](./docs/development-guide.md)

## 当前代码状态

当前仓库已经包含第一版可运行骨架：

1. Rust workspace。
2. 核心组件库 crate：`cad_widgets`。
3. 演示应用 crate：`cad_widgets_demo`。
4. 默认浅色/深色主题 token 与 egui style 映射。
5. 基础 shell layout：菜单栏、工具栏、左右面板、中央视图区、底部消息区、状态栏。

## 快速开始

运行 demo：

```bash
cargo run -p cad_widgets_demo
```

建议首次运行前先执行：

```bash
cargo check
```

## 当前目录结构

```text
.
├── Cargo.toml
├── crates/
│   ├── cad_widgets/
│   └── cad_widgets_demo/
└── docs/
```

## 项目目标

1. 为 CAD 桌面应用提供一组高频、可组合、视觉一致的 egui widget。
2. 提供典型桌面应用壳层模版，包括多面板布局、命令系统、状态栏、属性面板等基础 UI。
3. 优先抽象 UI 与交互模式，不预设具体几何内核或建模算法实现。
4. 保持组件 API 稳定、易测、易扩展，方便后续接入业务模型与渲染逻辑。

## 当前范围

本仓库首期以以下内容为主：

1. 常用 CAD UI widget。
2. 应用模版与布局骨架。
3. 主题、交互约定、状态同步模式。
4. 示例与最小可运行 demo。

暂不包含：

1. 几何建模算法。
2. 网格布尔、参数化求解等内核能力。
3. 复杂三维渲染管线。

## 后续建议执行顺序

1. 先创建 crate 骨架与基础 demo。
2. 完成 shell layout、dock 区域和属性面板基础组件。
3. 完成命令栏、工具栏、树视图、数值输入等高频 widget。
4. 补充主题系统、示例页与交互测试。