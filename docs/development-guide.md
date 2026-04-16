# 开发指导

## 1. 文档目的

本文档面向后续 AI coding agent 与人工开发者，目的是让执行者在不了解历史上下文的情况下，也能按统一标准推进本项目。

## 2. 开发目标拆解

当前优先级不是一次性做完整产品，而是建立一个可持续扩展的 UI 基础仓库。执行时按以下顺序推进：

1. 建立 Rust workspace 与 demo 应用。
2. 建立主题系统与 shell layout。
3. 实现一组高频 CAD widget。
4. 为每个核心 widget 补示例页。
5. 补文档、验收清单与基础测试。

## 3. 推荐实施路径

### 第一步：初始化仓库骨架

建议产出：

1. 顶层 Cargo workspace。
2. 核心 crate：cad_widgets。
3. 演示 crate：cad_widgets_demo。
4. docs 目录。
5. examples 目录。

建议最低可运行目标：

1. 启动一个 eframe 窗口。
2. 显示顶部菜单、左面板、右面板、中间工作区、底部状态栏。
3. 所有区域使用占位 widget 即可。

### 第二步：建立主题系统

先做统一 token，再做具体 widget。避免先写控件后回头硬改样式。

最低要求：

1. 颜色 token。
2. 间距 token。
3. 文本层级。
4. 控件尺寸等级。
5. 面板与边框风格。

### 第三步：实现首批核心 widget

建议顺序：

1. PanelHeader。
2. Toolbar 与 ToolButton。
3. StatusBar。
4. PropertyGrid。
5. NumericField。
6. TreeView。
7. CommandPalette。

原因：这些组件能最快组成一个像样的 CAD 桌面应用外壳。

## 4. AI coding agent 执行规则

后续 agent 接手时，建议遵循以下规则：

1. 每次只实现一个明确模块或一个小批量强相关模块。
2. 先补 crate 和模块骨架，再补实现，最后补 demo 页面。
3. 写公共 widget 时优先考虑通用 API，不带业务特定语义。
4. 不把“示例逻辑”混入组件库本体。
5. 不引入大型依赖来解决简单 UI 问题。
6. 每完成一个核心组件，立刻补一个最小演示入口。

## 5. 模块设计建议

### 5.1 theme 模块

职责：

1. 统一 token。
2. 将 token 映射到 egui style。
3. 管理浅色和深色主题切换。

避免：

1. 在各 widget 内部重复定义颜色。
2. 把业务状态塞进 theme。

### 5.2 shell 模块

职责：

1. 提供应用壳层布局函数或组件。
2. 统一顶部栏、侧栏、状态栏的拼装方式。
3. 给业务层预留中央 viewport 槽位。
4. 管理顶部、左侧、右侧、底部停靠 panel 的展开/收起状态。
5. 为四边停靠区提供统一的 dock host、tab group 和 split group 布局抽象。

建议接口方向：

1. ShellFrameConfig。
2. ShellSlots。
3. show_shell(ui or ctx, config, slots)。
4. ShellLayoutState 中显式保存四边 panel 的 open 状态和尺寸。
5. DockRegionState、PaneNode、TabGroupState、SplitNodeState 等状态对象应与中央文档状态分离。

建议架构术语：

1. Dock Region：四边停靠区本身，是工具面板的承载边界。
2. Dock Host：某个停靠区的根容器，负责管理该区域的 pane tree。
3. Tab Group：多个 panel 共享同一区域，以 tab 页切换显示。
4. Split Group：多个 panel 以水平或垂直分割同时显示。
5. Pane Node：布局树中的节点，可表示 panel、tab group 或 split group。

推荐建模方式：

1. 将每个停靠区建模为一棵 pane tree，而不是一组平铺字段。
2. pane tree 的叶子节点表示实际 panel，非叶子节点表示 tab group 或 split group。
3. tab 和 split 都应作为布局节点能力，而不是仅作为某个具体 panel 的附属属性。
4. panel 的 open、pinned、active、closable 等状态应属于 panel state；split ratio、orientation 应属于 layout node state。
5. dock region 与中央 document workspace 应保持强边界，但 top、left、right、bottom 四个 dock host 之间应允许 pane 迁移。

实现建议：

1. 原型阶段优先保留现有 shell 外壳，自研范围只覆盖 shell 与业务适配层。
2. 每个 dock region 内部的 pane tree 可以优先交由 egui_tiles 管理，以快速获得 tabs、horizontal split、vertical split、resize 和 drag-and-drop 能力。
3. 如果后续需要更强的企业级定制，再在 egui_tiles 之上增加 panel metadata、pin/unpin 策略和命令绑定层。
4. drag handle 与迁移入口的交互提示应明确区分两类边界：dock pane 可在四个 dock host 之间迁移，但不能拖入中央文档区；document pane 也不能拖出到 dock host。
5. dock host 的状态建议和中央 workspace 一样提供 snapshot API，便于保存跨区迁移结果、关闭集合和额外注入 pane 的状态。

### 5.3 panels 模块

职责：

1. 面板标题栏。
2. 可折叠面板容器。
3. 属性面板基础布局。

### 5.4 workspace 模块

职责：

1. 中央多文档 tab 条。
2. 活动文档状态管理。
3. 统一承载 3D、2D plot、表格等异构文档入口。
4. 支持中央文档区内部的 split view 与多视图组织。
5. 提供显式 builder、layout spec 和 snapshot API，便于初始化、持久化与恢复。

最低设计要求：

1. 支持 tab 激活、关闭和脏标记。
2. 不绑定具体文档渲染实现。
3. 能与 shell 中央区域自然组合。
4. 如需快速形成原型，可将中央工作区建模为独立的 egui_tiles document tree，而不是仅保留手写 tab 条。
5. builder 应允许调用方独立提供文档注册表和 pane tree 定义，避免 demo 数据渗入公共组件。
6. snapshot 应至少覆盖基线布局定义和关闭状态，并预留后续扩展到完整布局持久化的空间。

实现建议：

1. workspace 模块保留项目自有的 layout spec，而不是直接把 egui_tiles Tree 暴露为公共 API。
2. close/restore 入口要与四边 dock 区保持一致的交互语言，例如 restore last、restore all、按条目恢复。
3. demo 层应通过 builder 创建中央工作区，用快照按钮展示恢复能力，而不是继续依赖内部 demo 构造函数。
4. document pane 的拖拽和重排只能发生在中央 document workspace 内部，不能与四边 dock host 共享拖放目标。

### 5.5 inputs 模块

职责：

1. NumericField。
2. UnitField。
3. VecNField。
4. 搜索框与过滤输入。

最低设计要求：

1. 支持只读与禁用状态。
2. 支持错误提示。
3. 支持即时编辑与提交编辑区分。

### 5.6 trees 模块

职责：

1. TreeView。
2. 选择模型。
3. 节点渲染扩展点。

### 5.7 command 模块

职责：

1. 命令元数据结构。
2. 命令搜索面板。
3. 命令入口 widget。

## 6. 代码风格要求

1. 公开 API 命名直观，避免缩写堆叠。
2. 模块边界清晰，避免单文件过大。
3. 能用简单 struct 表达的，不要过度 trait 化。
4. 公共模块先追求可读性，再做泛型抽象。
5. 避免过早引入复杂宏。

## 7. Demo 策略

demo 不是附属品，而是主验收载体。

建议 demo 至少包含以下页面：

1. Shell Layout Demo。
2. Property Grid Demo。
3. Tree View Demo。
4. Inputs Demo。
5. Command Palette Demo。
6. Theme Demo。
7. Multi-Document Workspace Demo。

每个 demo 页面都应：

1. 展示正常态。
2. 展示禁用态。
3. 展示错误态或边界态。
4. 显示当前交互结果。

## 8. 验收标准

一个组件可以视为“完成”的最低标准：

1. 能在 demo 中独立展示。
2. API 能被外部最小成本调用。
3. 样式与主题系统一致。
4. 交互行为符合技术规范。
5. 没有明显布局抖动或状态错乱。

## 9. 建议任务拆分模板

后续可以按以下粒度给 agent 下发任务：

1. 初始化 workspace，并创建 cad_widgets 与 cad_widgets_demo 两个 crate。
2. 建立 theme 模块，并实现默认浅色主题映射到 egui style。
3. 实现 shell layout，支持顶部菜单、左右侧栏和底部状态栏。
4. 实现 PanelHeader 与可折叠面板容器。
5. 实现 NumericField，支持拖拽步进与单位显示。
6. 实现 PropertyGrid，并给出字符串、布尔、数值三种属性类型示例。
7. 实现 TreeView，支持展开、单选和右键菜单。
8. 实现 CommandPalette，并提供命令过滤示例。
9. 实现中央文档 tab workspace，支持 3D、2D plot、表格三类文档切换示例。

## 10. 建议的开发节奏

1. 每个阶段都保持可运行。
2. 不要先堆积多个未接线模块再统一整合。
3. 优先构建可见成果，便于持续人工评审。
4. 每完成一个里程碑，更新文档和截图。

## 11. 需要避免的问题

1. 过早绑定具体 CAD 业务模型。
2. 为了抽象而抽象，导致 API 难用。
3. 直接复制 demo 代码进入库代码。
4. 在没有主题 token 的情况下散落样式常量。
5. 把复杂状态机硬塞进单个 widget。

## 12. 后续第一批建议实施任务

如果下一步要直接开始编码，建议按下面顺序执行：

1. 初始化 Rust workspace 与基础 demo。
2. 实现 theme 模块和基础样式应用。
3. 实现 shell layout。
4. 实现 PanelHeader、Toolbar、StatusBar。
5. 实现 NumericField 与 PropertyGrid。
6. 实现 TreeView。
7. 实现中央 tab 工作区。
8. 补示例页面与基础说明。

## 13. 对后续 agent 的输入建议

为了让 AI coding agent 接手更高效，后续任务描述最好明确以下信息：

1. 本次只做哪个模块。
2. 目标 crate 和目标文件结构。
3. 是否需要 demo 页面。
4. 是否需要测试或仅需最小运行验证。
5. 是否允许引入新依赖。

推荐任务描述风格：

1. 范围小。
2. 可验收。
3. 有明确输出文件。
4. 有完成定义。

## 14. 完成定义

本阶段完成定义不是“组件全部写完”，而是：

1. 项目目标清晰。
2. 技术规范可执行。
3. 开发路径明确。
4. 后续 agent 能依据文档直接开始搭建代码骨架。