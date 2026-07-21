+++
title = "社区练习"
+++

## 社区练习列表

- 🇨🇳 [Rustlings 中文版（本项目）](@/_index.md)：面向简体中文用户的 Rustlings 翻译版。
- 🇯🇵 [Japanese Rustlings](https://github.com/sotanengel/rustlings-jp)：Rustlings 练习的日文翻译版。
- 🇨🇳 [Simplified Chinese Rustlings](https://github.com/SandmeyerX/rustlings-zh-cn)：Rustlings 练习的简体中文翻译版。
- 🇺🇦 [Rustlings in Ukrainian](https://github.com/noroutine/rustlings-ua)：Rustlings 练习的乌克兰语翻译版。
- 🇰🇷 [Korean Rustlings](https://github.com/eoncheole/rustlings-kr)：Rustlings 练习的韩文翻译版。

> 你可以使用通过 `cargo install` 安装的同一个 `rustlings` 程序运行社区练习。

## 创建社区练习

Rustlings 对社区练习的支持，让你可以创建专注于特定主题的练习。
你也可以将原版 Rustlings 练习翻译成其他语言，并作为社区练习发布。

### 开始创建

要创建社区练习，请安装 Rustlings，然后运行 `rustlings dev new PROJECT_NAME`。
这个命令类似于 `cargo new PROJECT_NAME`，会创建名为 `PROJECT_NAME` 的模板目录，其中包含开始开发所需的全部内容。

请阅读生成的 `info.toml` 文件中的_注释_，了解它的格式。
你可以在其中设置自定义的欢迎语和结束语，并指定每个练习的元数据。

### 创建练习

下面是一个练习元数据的示例：

```toml
[[exercises]]
name = "intro1"
hint = """
完成这个练习需要……
这些链接可能会对你有所帮助……"""
```

将这些内容写入 `info.toml` 后，在 `exercises/` 目录中创建 `intro1.rs` 文件。
练习必须包含一个 `main` 函数，但函数体可以为空。
建议为练习添加测试。
可以参考官方 Rustlings 练习获取灵感。

你也可以选择在 `solutions/` 目录中添加 `intro1.rs` 解答文件。

现在运行 `rustlings dev check`。
它会告诉你练习中存在的问题。
例如，它会提示你运行 `rustlings dev update`，更新 `Cargo.toml` 以包含新的练习 `intro1`。

如果有解答文件，`rustlings dev check` 还会运行它们，确保它们能够成功执行。
就是这样！你的第一个练习完成了 🎉

### Cargo.toml 配置

除了 `bin` 列表以外，你可以按照需要修改 `Cargo.toml` 文件。

> 运行 `rustlings dev update` 会自动更新 `bin` 列表。

- 你可以在 `[dependencies]` 表中添加依赖。
- 你可能希望为所有练习[配置 lint](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section)，可以在 `[lints.rust]` 和 `[lints.clippy]` 表中完成配置。

### 发布

现在，继续添加练习，并将其发布为 Git 仓库。

用户只需克隆该仓库并在其中运行 `rustlings`，就可以开始完成你的练习（与官方练习相同）。
与官方练习不同的是，在用户完成练习之前，解答文件不会被隐藏。
不过，你可以相信用户不会过早打开解答 😉

### 分享

发布社区练习后，可以在[官方 Rustlings 仓库](https://github.com/rust-lang/rustlings)中创建 Issue 或 Pull Request，将项目加入本页面的社区练习列表 😃
