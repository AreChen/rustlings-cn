# Rustlings 简体中文持续本地化设计

## 背景与目标

本仓库以 Rustlings 6.5.0 的 `upstream/main` 为同步基线，目标是维护一份可运行、可审校、可持续同步的简体中文版。范围覆盖 `exercises/` 的全部 94 个 Rust 练习、26 个 README，以及 `src/` 中面向学习者的 CLI 帮助、欢迎文本、提示、状态文本、错误和退出消息。

翻译必须不改变练习的教学契约：练习目录、文件名、exercise name、命令名、Rust API、标识符、代码块和测试依赖的字符串保持原样。可以翻译练习注释、说明、提示和不参与测试契约的用户输出。无法安全判断的字符串写入维护文档的保留清单，而不是伪造覆盖率。

## 方案

采用原地翻译加静态审计：

1. 逐文件将主题 README 和练习中的自然语言改为简体中文；Rust 字符串只有在确认不是测试期望值、标识符、协议/文件格式或诊断契约后才翻译。
2. 将 `src/cli.rs` 的 Clap 文档和 `src/` 中面向用户的输出翻译为中文；命令、参数、环境变量、路径和技术 API 用反引号保护并遵循术语表。
3. 添加 `scripts/check_translation.py`。它只使用 Python 标准库和本地 Git：扫描 Markdown、Rust 注释、Clap 文档及用户输出，跳过代码块和显式保留项，输出总文件数、中文单元数、英文候选、逐文件覆盖率；它还比较记录的上游 blob 基线，报告新增、删除或上游已变化的文件。
4. 添加 `scripts/translation-baseline.json`，记录当前上游基线引用和每个受管文件的上游 blob。翻译后的本地文件不与上游全文直接比较；同步风险通过上游 blob 是否变化来判断。
5. 添加 GitHub Actions，在 Ubuntu 和 Windows 上执行 `cargo test --all-targets`、`cargo fmt --check`、翻译覆盖检查及 `git diff --check`。检查脚本不联网，CI 的 checkout 只需提供上游基线所需的 Git 对象。

## 错误处理与保留规则

检查器将候选分为 `translated`、`english` 和 `preserved`。保留项必须有明确原因，例如测试断言中的精确字符串、Rust 标准库/编译器诊断示例、ASCII 艺术字、命令与 API、文件格式内容或用于展示语言特性的英文数据。检查报告直接列出这些项；任何未分类且含有英文自然语言的用户文案都会使检查失败。

源文件中使用 `// translation: preserve <reason>` 或 `// translation: preserve` 的显式标记时，检查器只跳过该行并把原因收集到报告；更大的保留集合放在 `TRANSLATION.md`，以便审校者能看到真实缺口。

## 验证

实现后按顺序运行：

```text
cargo test --all-targets
cargo fmt --check
cargo run -- --help
python scripts/check_translation.py
git diff --check
```

在 Windows 上额外确认检查器和 Cargo 使用仓库相对路径工作，且 `dev/Cargo.toml` 等生成文件没有被加入变更；Ubuntu CI 使用同一组命令，避免平台分叉。
