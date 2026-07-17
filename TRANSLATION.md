# Rustlings 中文翻译与同步指南

## 项目关系

- 中文仓库：<https://github.com/AreChen/rustlings-cn>
- 官方上游：<https://github.com/rust-lang/rustlings>
- 本仓库的 `origin` 指向中文 fork，`upstream` 指向官方仓库。
- 当前同步基线是官方 Rustlings 6.5.0；更新版本号时以 `Cargo.toml` 和上游提交为准。

## 翻译原则

1. 先理解练习目标，再翻译说明；中文应自然、简洁，避免逐词直译。
2. Rust 代码、关键字、标识符、文件名、命令、API 名称和代码块保持原样。
3. `TODO`、`I AM NOT DONE` 等练习标记必须保留，不能因为翻译而让工具失去识别能力。
4. 测试依赖的字符串输出保持原样；只翻译不会影响测试语义的说明性文本。
5. Markdown 链接、锚点和 URL 必须保留并在本地验证。
6. 第一次出现的专业术语可以写成“中文（英文）”，之后优先使用术语表中的统一译法。

## 当前术语约定

| English | 中文建议 |
| --- | --- |
| variable | 变量 |
| immutable | 不可变 |
| mutable | 可变 |
| function | 函数 |
| expression | 表达式 |
| compiler | 编译器 |
| primitive type | 基本类型 |
| slice | 切片 |
| tuple | 元组 |
| array | 数组 |

## 当前状态

当前仓库以官方 Rustlings 6.5.0 为基线，已覆盖 `exercises/` 下全部练习 Rust 文件和 README，并完成 CLI 学习者可见文案的中文化。练习代码逻辑、练习名称、文件名、API、命令、测试契约字符串和生成模板中的技术字段按同步要求保留。

翻译检查器是状态的唯一可执行来源：

```bash
python scripts/test_check_translation.py
python scripts/check_translation.py --skip-upstream
```

检查器会统计中文、英文缺口和明确保留项，并审计上游文件集合是否发生变化。任何上游变更都必须重新确认对应中文文件，而不能直接覆盖本地翻译。

## 同步上游

翻译工作进行期间不要直接覆盖本地改动。建议先查看上游变化，再逐文件处理：

```bash
git fetch upstream
git log --oneline main..upstream/main
git diff --stat main...upstream/main
```

合并上游后，需要重新检查新增或修改的练习说明，并运行仓库验证命令。

## 本地验证

```bash
cargo test --all-targets
cargo fmt --all --check
cargo run -- --help
python scripts/test_check_translation.py
python scripts/check_translation.py --skip-upstream
git diff --check
```

提交前还应运行 `git fetch upstream`，确认 `main...upstream/main` 的差异，并在必要时更新翻译基线。GitHub Actions 会在 Ubuntu 上自动运行 Rust 测试、格式检查、翻译检查和空白检查。
