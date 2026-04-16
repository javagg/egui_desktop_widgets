# 技术规范

## 1. 项目定位

本项目是一个面向 egui 和 eframe 的桌面 UI 组件库与应用模版集合，服务对象是典型 CAD 建模类桌面应用。项目重点不是几何算法本身，而是承载 CAD 业务所需的桌面交互框架。

核心原则：

1. 以桌面端交互为中心，优先支持鼠标、键盘、右键菜单、快捷键和高密度信息展示。
2. 以组合优先于继承，所有 widget 应能在 egui immediate mode 模式下自然嵌入。
3. UI 层与领域层解耦，组件不直接绑定几何内核类型。
4. 先覆盖 80% 的典型 CAD 操作场景，再逐步扩展高级交互。

## 2. 目标用户与场景

目标用户：

1. 使用 Rust、egui、eframe 开发桌面 CAD、CAM、CAE、BIM、参数化建模或可视化编辑器的团队。
2. 希望快速搭建具有专业桌面软件形态的工程应用开发者。

典型场景：

1. 多文档或单文档建模工作区。
2. 左侧模型树、右侧属性面板、中间视图区、底部日志/状态区。
3. 命令驱动建模流程，如拉伸、倒角、阵列、草图编辑。
4. 高精度数值输入、约束状态反馈、选择集展示与编辑。
5. 中央区域同时打开 3D 建模、2D plot、数据表格等异构文档，并以 tab 页组织。

## 3. 范围定义

### 3.1 首期包含

1. 应用外壳模版。
2. 常见 CAD 高密度控件。
3. 命令系统 UI 封装。
4. 状态栏、消息栏、通知区。
5. 属性编辑器、树视图、面板标题栏。
6. 主题与尺寸规范。
7. 示例应用和文档。

### 3.2 首期不包含

1. 几何建模内核。
2. 文件格式解析器。
3. GPU 渲染引擎抽象。
4. 完整 docking 框架的自研实现。

说明：如果后续需要 docking，优先评估兼容 egui 生态的现有方案，并将适配层控制在本项目内。

## 4. 技术栈约束

1. Rust stable。
2. egui 作为核心 UI 框架。
3. eframe 作为桌面应用承载层。
4. 可选依赖应最小化，优先选择维护活跃、体量适中的 crate。
5. 默认支持 macOS、Windows、Linux 三端桌面运行。

建议依赖策略：

1. 核心 crate 只依赖 egui、eframe 和少量基础工具库。
2. 示例应用可按需引入更多生态依赖，但必须与核心库分层。
3. 高风险或 API 不稳定依赖需要单独隔离在 feature 或 adapter 模块中。

## 5. 设计原则

### 5.1 可预测性

1. 相同交互在不同 widget 中的行为应保持一致。
2. 输入提交、取消、聚焦、拖拽、悬停反馈应遵循统一约定。

### 5.2 高信息密度

1. 适配 CAD 类软件常见紧凑布局。
2. 允许较小字号和控件高度，但必须保证可读性和点击热区。

### 5.3 可扩展性

1. widget 应支持外部传入状态、样式和回调。
2. 尽量通过数据配置与 builder 模式拓展行为。

### 5.4 最小业务侵入

1. 组件 API 使用通用领域术语，如 Selection、PropertyItem、CommandAction。
2. 不直接依赖具体实体类型，如 Sketch、Solid、Edge。

## 6. 推荐仓库结构

```text
egui_desktop_widgets/
├── Cargo.toml
├── crates/
│   ├── cad_widgets/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── theme/
│   │   │   ├── shell/
│   │   │   ├── panels/
│   │   │   ├── inputs/
│   │   │   ├── trees/
│   │   │   ├── command/
│   │   │   ├── feedback/
│   │   │   └── utils/
│   └── cad_widgets_demo/
│       └── src/
├── docs/
└── examples/
```

推荐分层：

1. cad_widgets：组件库本体。
2. cad_widgets_demo：演示与验收载体。
3. docs：规范、决策记录、组件说明。
4. examples：独立示例与试验性场景。

## 7. 功能模块规范

### 7.1 应用壳层 Shell

应提供：

1. 顶部菜单栏。
2. 顶部或侧边工具栏。
3. 四边 Docking Regions。
4. 中央多文档工作区。
5. 底部状态栏。
6. 底部消息/日志面板。

要求：

1. 布局支持固定面板与可折叠面板。
2. 中央工作区始终优先保证空间。
3. 面板标题栏具备折叠、关闭、更多操作入口。
4. 中央工作区应支持 tab 页切换，并能承载异构文档类型。
5. tab 至少支持激活态、脏标记和关闭入口。
6. 顶部、左侧、右侧、底部四边停靠 panel 都应支持展开和收起。
7. panel 收起后应保留轻量恢复入口，避免用户失去打开通路。

专业定义：

1. 应用壳层应采用四边停靠的 docking layout，即 top、left、right、bottom 四个停靠区围绕中央 document workspace 布局。
2. 每个停靠区不是单一 panel 槽位，而是一个可容纳多个 pane 的 dock host。
3. 同一停靠区内的多个 panel 应支持以 tab stack 方式组织，也应支持以 split container 方式并列排布。
4. split container 需要同时覆盖 horizontal split 和 vertical split，两者可递归组合形成嵌套 pane tree。
5. 从布局模型上看，每个停靠区本质上是一个 pane graph，其叶子节点为具体 panel，内部节点为 tab group 或 split group。
6. tab group 适合承载互斥浏览的工具面板，例如 Model、Layers、History；split group 适合承载需要同时可见的面板组合，例如 Object Tree 与 Selection Inspector。
7. 中央区域与四边停靠区应解耦。中央区域负责 document tabs 和 document views，停靠区负责工具 pane、属性 pane、日志 pane、分析 pane 等辅助工作区。

中央文档区要求：

1. 中央文档区不仅要支持传统 document tabs，还应支持基于 pane tree 的文档分屏能力。
2. 同一中央工作区内应允许 document tab group 与 split view 并存，例如左侧显示 3D 视图，右侧以 tab 组织 2D plot、表格和报告。
3. 中央文档区的 split 语义应面向 document views，而不是工具 pane，因此其节点类型应与四边 dock pane 分层建模。
4. 中央文档区关闭文档、切换文档和重排文档时，不能影响四边 dock region 的布局稳定性。
5. 中央文档区应支持由显式 layout spec 或 builder 初始化，而不是将示例数据硬编码在 widget 内部。
6. 中央文档区应提供快照与恢复 API，用于保存文档注册表、基线 pane tree 定义以及关闭状态集合。
7. 四边 dock pane 与中央 document pane 属于不同布局域。四个 dock host 之间允许跨区迁移与重组，但不允许与中央 document workspace 发生跨区拖拽、跨区停靠或跨区类型转换。

布局能力要求：

1. 每个停靠区至少应支持单 panel、tabbed panels、split panels 三种基本组织形式。
2. 同一停靠区内的 split 布局应允许二叉递归扩展，以适应复杂工程软件常见的多面板组合场景。
3. 每个 panel 必须具备唯一标识、标题、可见性状态、停靠归属和布局节点信息。
4. tab group 必须维护活动 tab、tab 顺序、关闭能力和脏标记状态。
5. split group 必须维护方向、分割比例、最小尺寸约束以及子节点集合。
6. 收起操作针对 panel 或 dock region 的可见性，不应破坏其在 pane tree 中的布局定义；恢复时应尽可能回到原布局位置。
7. 如果后续支持拖拽重排，drag target 应以 dock region、tab group 和 split edge 为基础语义单元。
8. 对于未组织为 tab 的独立 pane，系统仍应提供可见的 pane header 或 drag handle，以保证该 pane 在 split 布局中仍可被拖动和重排。
9. pane header 应支持最小动作集合：drag grip、pin or unpin、close、overflow menu。
10. tabbed pane 与 standalone pane 应采用不同的 header 策略：tabbed pane 使用紧凑型内部 header，避免与 tab bar 产生强视觉重复；standalone pane 使用完整 header band，以强化其作为独立工作面板的边界感和可操作性。
11. dock pane 与 document pane 的 close 语义都必须配套 restore affordance，避免关闭后丢失恢复通路。
12. restore affordance 应尽量采用统一模式，例如 restore last、restore all、按标题恢复，减少用户在 dock 区与中央区之间切换时的认知差异。
13. drag and drop 的作用域必须区分两类边界：dock pane 可以在 top、left、right、bottom 四个 dock host 之间迁移和重排；document pane 仅能在中央 document workspace 内重排。
14. dock host 与中央 document workspace 之间仍然保持隔离。如果后续确有业务需要支持两者之间的流转，也必须通过显式命令语义实现，例如 open as document 或 reveal in inspector，而不是直接复用通用拖拽手势。

原型实现说明：

1. 当前项目原型可以采用 egui_tiles 作为 dock region 内部的 pane tree 引擎。
2. shell 负责四边停靠区边界、中央工作区、状态栏和整体应用框架；egui_tiles 负责单个停靠区内部的 tabs、horizontal split、vertical split 和拖拽重排。
3. 对于当前使用的 egui 0.33，建议选用兼容版本 egui_tiles 0.14.x。
4. 中央文档区同样可以建立独立的 egui_tiles document tree，用于承载 document tabs、document splits 与多视图联动场景。
5. 推荐在 egui_tiles 之上保留一层项目自有的 DocumentWorkspaceBuilder、DocumentWorkspaceSnapshot 等抽象，用于持久化、恢复、示例构造和后续布局迁移。
6. 四边 dock host 也应提供与中央工作区对称的快照与恢复能力，至少覆盖跨区迁移结果、已关闭 pane 集合和额外导入 pane 状态。

### 7.2 Widget 分类

高优先级 widget：

1. 紧凑工具按钮与分组工具栏。
2. 图标按钮、切换按钮、分裂按钮。
3. 属性网格 Property Grid。
4. 数值输入 Numeric Field，支持步进、拖拽调节、单位显示。
5. 向量输入 Vec2/Vec3/Vec4 Field。
6. 树视图与大纲视图。
7. 可搜索下拉框与过滤列表。
8. 标签页或工作区切换器。
9. 面包屑、选择集面板、对象信息卡片。
10. 状态条、进度条、轻量消息条。
11. 中央文档 tab 条与文档工作区容器。

中优先级 widget：

1. 命令面板 Command Palette。
2. 参数编辑对话框。
3. 快捷键提示浮层。
4. 表格或 inspector 风格列表。
5. 图层/显示状态切换面板。

### 7.3 命令系统 UI

应抽象以下能力：

1. 命令注册元数据：命令 id、标题、分组、提示、快捷键。
2. 命令触发入口：菜单、工具栏、命令面板、右键菜单。
3. 命令状态：可用、禁用、激活中、进行中。
4. 命令反馈：状态栏文本、toast、底部日志。

组件层只负责表达与交互，不负责编排具体业务命令执行。

### 7.4 属性编辑器 Property Grid

必须满足：

1. 支持按组展示属性。
2. 支持只读、可编辑、错误、警告状态。
3. 支持布尔、枚举、字符串、数值、颜色、向量等常见类型。
4. 支持延迟提交与实时提交两种模式。
5. 支持单位显示与数值格式化。

推荐 API 形态：

1. 数据驱动：PropertySection、PropertyRow、PropertyValue。
2. 渲染自定义：允许外部为某个属性类型提供 custom renderer。

### 7.5 树视图 Tree View

必须满足：

1. 支持展开/折叠。
2. 支持单选、多选和范围选择。
3. 支持右键菜单。
4. 支持图标、名称、辅助状态标识。
5. 支持懒加载和过滤。

可选能力：

1. 拖拽排序。
2. 拖拽父子重组。

## 8. 状态管理规范

建议采用三层状态：

1. AppState：应用级 UI 状态，如当前布局、主题、活动面板、通知队列。
2. ViewState：视图级状态，如树节点展开、滚动位置、筛选条件。
3. DomainState Adapter：业务层到 UI 层的数据映射，不直接把业务对象暴露给 widget。
4. DocumentWorkspaceState：中央 tab 工作区状态，如打开文档列表、活动 tab、脏标记、关闭请求。
5. DocumentWorkspaceSnapshot：中央工作区的持久化快照，至少包括 layout spec、文档元数据和已关闭文档集合。

约束：

1. widget 尽量无状态或显式状态。
2. 避免在 widget 内部隐式持久化复杂业务状态。
3. 所有跨组件交互应通过明确事件或共享状态入口完成。
4. 关闭后的 dock pane 与 document pane 应以显式恢复状态暴露给上层，而不是仅靠内部隐藏标志维持。

## 9. 交互规范

### 9.1 输入行为

1. Enter 提交当前编辑。
2. Escape 取消当前编辑并恢复原值。
3. 数值输入支持鼠标滚轮或拖拽步进时，需提供视觉反馈。
4. 失焦是否提交必须在组件文档中明确定义。

### 9.2 选择行为

1. 点击主项为单选。
2. Command 或 Ctrl 用于追加选择。
3. Shift 用于范围选择。
4. 当前激活项与已选项应有不同视觉状态。

### 9.3 反馈行为

1. 短反馈进入状态栏。
2. 瞬时操作结果使用 toast 或消息条。
3. 长任务需要进度显示与取消入口。

## 10. 视觉与主题规范

### 10.1 视觉方向

1. 偏工程软件风格，强调清晰、克制、信息密度。
2. 默认浅色主题，后续补充深色主题。
3. 颜色不追求品牌感优先，先保证层级和可读性。

### 10.2 尺寸基线

建议基线：

1. 主字号 12 至 14 px。
2. 紧凑控件高度 22 至 26 px。
3. 常规控件高度 28 至 32 px。
4. 面板内边距 6 至 12 px。

### 10.3 主题抽象

应定义：

1. 颜色 token。
2. 间距 token。
3. 圆角 token。
4. 描边 token。
5. 字体与字号 token。

禁止在 widget 内散落硬编码颜色和尺寸常量。

## 11. API 设计规范

1. 公共组件优先使用 builder 模式或配置对象。
2. 入参命名保持领域通用与语义清晰。
3. 尽量返回 egui::Response 或包含 Response 的结果对象，便于外部联动。
4. 每个公共 widget 都需要最小示例。
5. 不暴露不稳定内部模块为 public API。

推荐模式：

1. Stateless widget + external state。
2. 简单配置 struct + show(ui, state) 调用方式。
3. 对复杂组件提供默认配置与高级配置两层 API。

## 12. 可测试性规范

1. 纯样式函数、布局计算、数据映射逻辑必须可单元测试。
2. 复杂交互流程至少提供 demo 场景用于人工验收。
3. 关键 widget 需要截图基线或交互清单。

建议验收维度：

1. 多平台显示一致性。
2. 高 DPI 下布局稳定性。
3. 键盘导航可用性。
4. 大量节点或属性项下的性能表现。

## 13. 文档规范

至少维护以下文档：

1. 组件总览。
2. 每个核心 widget 的用途、状态、输入输出说明。
3. 应用壳层搭建说明。
4. 主题定制说明。
5. 示例截图与典型用法。

## 14. 首期里程碑

### M1: 基础骨架

1. 建立 workspace 与 crate 结构。
2. 建立 demo 应用。
3. 完成主题 token 和 shell layout。

### M2: 高频组件

1. 工具栏。
2. 属性面板。
3. 树视图。
4. 状态栏。
5. 数值输入。

### M3: 命令与示例

1. 命令面板。
2. 右键菜单模式。
3. 示例场景页。
4. 文档补全。

## 15. 非功能要求

1. 首屏 demo 应在合理时间内启动，避免无必要初始化。
2. 组件库 API 变更要控制频率，优先稳定。
3. 对外暴露模块需具备基础 rustdoc。
4. 示例必须可运行，不能只保留伪代码。

## 16. 风险与边界

1. egui 属于 immediate mode UI，部分复杂虚拟化和 docking 交互需谨慎抽象。
2. CAD 应用通常有高频刷新和复杂画布交互，组件库不能阻碍后续渲染主循环。
3. 若未来接入 3D 视图，UI 层需要明确与 viewport 事件分发边界。

结论：本项目首期应先成为“专业桌面 CAD UI 基础设施”，而不是“完整 CAD 框架”。