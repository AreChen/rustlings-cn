# Rustlings 简体中文持续本地化实施计划

> **面向 Agent 型工作者：** 本计划在当前用户授权的工作目录内执行；不提交、不推送，不修改 `book-cn`。步骤用复选框跟踪。

**目标：** 将 Rustlings 6.5.0 的全部练习和 CLI 用户文案维护为可运行、可审计的简体中文版本。

**架构：** 练习与 CLI 文案原地翻译，代码结构和测试契约保持不变；Python 标准库检查器负责候选文案、中文覆盖、保留原因和上游 blob 基线风险；GitHub Actions 在 Ubuntu/Windows 复用同一套检查。

**技术栈：** Rust 2024、Cargo、Python 3 标准库、GitHub Actions。

## 全局约束

- 保留 `exercises/` 下全部 Rust 代码逻辑、练习文件名、exercise name、命令名、API 名称和测试依赖字符串。
- 翻译所有主题 README、练习说明/提示/注释和适合本地化的终端用户消息；代码块、标识符、命令、API 和安全保留字符串不翻译。
- 不修改 `book-cn`，不 commit，不 push。
- 检查器不依赖网络，必须在 Windows 和 Ubuntu 运行。
- 完成前运行 `cargo test --all-targets`、`cargo fmt --check`、`cargo run -- --help`、`python scripts/check_translation.py`、`git diff --check`。

---

### 任务 1：建立覆盖检查器的失败测试

**文件：**

- 创建：`scripts/check_translation.py`
- 创建：`scripts/translation-baseline.json`
- 创建：`scripts/test_check_translation.py`

**接口：**

- `scripts/check_translation.py --root PATH --upstream-ref REF --json` 输出 JSON 报告并在发现未分类英文文案或同步风险时返回非零状态。
- `scripts/check_translation.py --root PATH --upstream-ref REF` 输出人类可读汇总。
- `scripts/test_check_translation.py` 使用 `unittest` 和临时目录覆盖解析、覆盖率、保留项、缺失文件和上游风险。

- [ ] **步骤 1：编写失败测试**：为 Markdown 代码块排除、中文/英文候选识别、`translation: preserve <reason>` 原因收集、缺少受管文件和上游 blob 变化分别写最小 `unittest`。
- [ ] **步骤 2：运行测试确认失败**：运行 `python scripts/test_check_translation.py`；预期因 `scripts.check_translation` 尚不存在而失败。
- [ ] **步骤 3：实现最小检查器**：只使用 `argparse`、`hashlib`、`json`、`pathlib`、`re`、`subprocess`、`sys`；解析受管文件，跳过 Markdown fenced code、Rust 代码结构及非用户字符串，输出 `translated`/`english`/`preserved` 记录和非零退出码。
- [ ] **步骤 4：运行单元测试确认通过**：再次运行 `python scripts/test_check_translation.py`；预期全部通过。

### 任务 2：更新术语表、全量状态和同步规则

**文件：**

- 修改：`TRANSLATION.md`
- 修改：`README.zh-CN.md`
- 修改：`scripts/translation-baseline.json`

- [ ] **步骤 1：列出 24 个主题/特殊目录、94 个 Rust 文件和 26 个 README 的真实状态**：将每个目录的 README、说明/注释、可翻译输出和审校状态分别记录为 `已完成`、`进行中` 或 `未开始`，不把检查器生成的报告当成翻译完成。
- [ ] **步骤 2：补充术语表与保留清单**：记录 Rust 术语、CLI 术语、命令/API/标识符/测试字符串/数据样例等不能翻译的原因，并写明如何处理上游新增或修改文件。
- [ ] **步骤 3：由检查器生成并固化基线**：以 `upstream/main` 当前解析出的 blob 为基线，记录 ref、commit 和全部受管路径；本地翻译内容不覆盖上游源 blob。
- [ ] **步骤 4：运行检查器**：在文案尚未翻译时确认它会真实列出英文缺口并失败，而不是报告虚假满覆盖。

### 任务 3：翻译全部 exercises 文案

**文件：**

- 修改：`exercises/README.md`、`exercises/quizzes/README.md`、`exercises/*/README.md`
- 修改：`exercises/**/*.rs`（94 个文件）

- [ ] **步骤 1：先保存每个受管文件的练习名、路径、API 和测试期望字符串快照**：检查变更只触及自然语言，不改变这些契约。
- [ ] **步骤 2：按主题批量翻译 26 个 README**：保留 Markdown 链接、锚点、URL、代码块和 Rust API；统一使用术语表。
- [ ] **步骤 3：按主题翻译 94 个 Rust 文件中的 TODO、说明、提示和普通用户输出**：测试断言和需要展示原始 Rust/英文数据的字符串保持原样，并用保留原因登记。
- [ ] **步骤 4：运行练习结构快照检查**：确认文件数量、文件名、exercise name、`info.toml` 条目、测试断言字符串未被意外改动。
- [ ] **步骤 5：运行 `python scripts/check_translation.py`**：预期剩余项目仅为有理由的 `preserved`，未经分类的英文用户文案为零。

### 任务 4：本地化 src CLI 用户文案

**文件：**

- 修改：`src/cli.rs`
- 修改：`src/main.rs`、`src/app_state.rs`、`src/exercise.rs`、`src/list.rs`、`src/list/state.rs`、`src/run.rs`
- 修改：`src/init.rs`、`src/editor.rs`、`src/editor/zellij.rs`、`src/dev.rs`、`src/dev/check.rs`、`src/dev/new.rs`、`src/dev/update.rs`
- 修改：`src/cmd.rs`、`src/embedded.rs`、`src/info_file.rs`、`src/watch.rs` 及其用户可见错误文本

- [ ] **步骤 1：翻译 Clap 帮助和命令说明**：保留 `init`、`run`、`check-all`、`reset`、`hint`、`dev`、参数名、`EDIT_CMD`、`VS Code`、`Zellij` 等字面量。
- [ ] **步骤 2：翻译欢迎、进度、状态、提示、错误和退出文本**：保留文件路径、exercise name、编译器/工具原始输出和测试依赖字符串；统一 `已完成`/`待完成` 等状态词。
- [ ] **步骤 3：运行 `cargo fmt --check` 和 CLI 单元测试**：检查帮助输出能渲染中文且命令解析不变。
- [ ] **步骤 4：运行 `cargo run -- --help`**：确认终端用户能看到中文帮助。

### 任务 5：加入跨平台 CI

**文件：**

- 创建：`.github/workflows/translation.yml`

- [ ] **步骤 1：配置 Ubuntu/Windows matrix**：安装稳定 Rust 和 Python，启用 `actions/checkout`，运行相同的 Cargo、fmt、翻译覆盖和 diff 检查。
- [ ] **步骤 2：确保 CI 不生成并提交 `dev/Cargo.toml`**：只在检查前运行必要的命令，使用工作区相对路径。
- [ ] **步骤 3：本地解析 YAML 逻辑并运行等价命令**：确认 Windows 使用 `python scripts/check_translation.py` 而非 Bash 专用语法。

### 任务 6：完整验证与缺口报告

- [ ] **步骤 1：运行 `cargo test --all-targets` 并记录退出码和失败数。**
- [ ] **步骤 2：运行 `cargo fmt --check` 并记录退出码。**
- [ ] **步骤 3：运行 `cargo run -- --help` 并核对中文帮助。**
- [ ] **步骤 4：运行 `python scripts/check_translation.py` 并记录覆盖率、保留项和同步风险。**
- [ ] **步骤 5：运行 `git diff --check`，并确认 `git status --short` 中没有 `book-cn`、生成的 `dev/Cargo.toml` 或目标目录变更。**
- [ ] **步骤 6：按验证结果更新 `TRANSLATION.md`，如有真实无法安全翻译的字符串，逐项列出原因。**
