# Rustlings 交互式文案翻译补全实施计划

> **面向 Agent 型工作者：** 必需的子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 来逐项任务实施此计划。步骤使用复选框（`- [ ]`）语法进行跟踪。

**目标：** 将 Rustlings CLI 的欢迎语、完成提示和 94 条练习提示完整本地化为简体中文。

**架构：** 保持 `rustlings-macros/info.toml` 的上游 TOML 结构和嵌入路径不变，只替换学习者可见的自然语言。新增一个 Python 回归测试检查 metadata 文案，避免扩大现有上游同步基线的范围。

**技术栈：** Rust 2024、TOML、Rust proc-macro 嵌入、Python `tomllib`/`unittest`。

## 全局约束

- Rust 代码、关键字、标识符、文件名、命令、API 名称、URL 和测试契约字符串必须保持原样。
- 练习数量和 `info.toml` 中的练习顺序必须保持 94 项不变。
- 所有文案修改必须通过 `git diff --check`、Python 翻译测试和 Rust 测试。

---

### 任务 1: 为交互文案建立失败回归测试

#### 文件：

- 修改：`scripts/test_check_translation.py`
- 测试：同一测试文件中的 `InfoMetadataTranslationTests`

#### 步骤：

- [x] 使用 `tomllib` 读取 `rustlings-macros/info.toml`。
- [x] 检查欢迎语、完成提示和 94 条 hint 的数量。
- [x] 去除 URL 和反引号代码片段后，拒绝不含中文但包含英文单词的文案行。
- [x] 运行测试并确认它因当前英文 metadata 失败。

### 任务 2: 翻译嵌入式学习文案

#### 文件：

- 修改：`rustlings-macros/info.toml`
- 修改：必要时更新 `TRANSLATION.md` 的覆盖范围说明

#### 步骤：

- [x] 翻译 `welcome_message` 和 `final_message`。
- [x] 按现有练习顺序翻译全部 94 条 `hint`。
- [x] 保留命令、代码、链接、类型名和练习标识符。
- [x] 运行回归测试并确认通过。

### 任务 3: 验证嵌入后的 CLI

#### 步骤：

- [x] 运行 Python 翻译测试和覆盖检查。
- [x] 运行 `cargo fmt --all -- --check` 和 `cargo test --all-targets`。
- [x] 运行 `cargo run -- hint intro2`，确认输出中文提示。
- [x] 检查构建副产物 `dev-Cargo.toml` 未被纳入提交。

### 任务 4: 发布

#### 步骤：

- [ ] 审阅差异并提交翻译补全。
- [ ] 推送 `rustlings-cn` 的 `main` 分支。
- [ ] 验证远端提交、工作区状态和最终 CLI 文案。
