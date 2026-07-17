# Rustlings 中文版 🦀

这是基于 [Rustlings 官方仓库](https://github.com/rust-lang/rustlings) 的非官方中文翻译工作仓库。Rustlings 通过一组小练习帮助你熟悉阅读和编写 Rust 代码，也帮助你学会阅读编译器信息。

本仓库当前以官方 Rustlings 6.5.0 的代码和练习结构为同步基线。上游仓库仍然是 Rustlings 的事实来源；中文翻译、术语和贡献流程由本仓库维护。

## 快速开始

先安装 Rust 和 Cargo，然后克隆本仓库并安装本仓库构建出的 Rustlings：

```bash
git clone https://github.com/AreChen/rustlings-cn.git
cd rustlings-cn
cargo install --path . --force
rustlings init
cd rustlings
rustlings
```

最后一条命令会进入交互式练习模式。你也可以使用以下命令：

```bash
rustlings check-all       # 检查所有练习
rustlings run              # 运行下一个未完成的练习
rustlings run variables1  # 运行指定练习
rustlings hint             # 查看下一个练习的提示
```

练习本身故意包含待修复的错误，因此刚初始化时不应期待 `check-all` 全部通过。按照终端提示编辑 `exercises/` 下的文件，直到练习完成。

## 中文翻译范围

当前优先翻译入门、变量、函数、条件判断和基本类型主题的说明、提示和注释。Rust 代码、命令、文件名、API 名称和测试行为保持不变，便于与官方 Rustlings 同步，也便于学习者搜索英文资料。

翻译状态和贡献规则见 [`TRANSLATION.md`](TRANSLATION.md)。

## 参与贡献

请先阅读 [`TRANSLATION.md`](TRANSLATION.md)，选择一个尚未完成的文件或主题。修改后运行：

```bash
cargo test --all-targets
cargo run -- --help
git diff --check
```

如果你发现翻译错误、术语不一致或上游内容已经变化，欢迎提交 Issue 或 Pull Request。

## 上游与许可证

- 上游：<https://github.com/rust-lang/rustlings>
- 当前 fork：<https://github.com/AreChen/rustlings-cn>
- Rustlings 使用 MIT 许可证，详见 [`LICENSE`](LICENSE)。
