+++
title = "使用"
+++

<!-- toc -->

## 完成练习

练习按照主题分类，位于 `exercises/<topic>` 子目录中。
每个主题还包含一个 `README.md` 文件，其中有帮助你入门该主题的资料。
我们强烈建议在开始练习前先阅读这些资料 📚️

大多数练习都包含会阻止编译的错误，需要你亲手修复！
有些练习还包含测试，只有测试通过后才算完成 ✅

搜索 `TODO` 和 `todo!()`，找到需要修改的位置。
在_监视模式（Watch Mode）_中输入 `h` 获取提示 💡

## 监视模式（Watch Mode）

完成[初始化](@/setup/index.md)后，只需运行 `rustlings` 命令即可启动 Rustlings。

这会启动_监视模式_，按照预先设定的顺序带你完成练习（我们认为这个顺序最适合初学者）。
每当你修改 `exercises/` 目录中的练习文件时，它都会自动重新运行当前练习。

{% details(summary="如果无法检测 <code>exercises/</code> 目录中的文件变化……") %}

你可以添加 **`--manual-run`** 参数（`rustlings --manual-run`），然后在监视模式中输入 `r`，手动重新运行当前练习。

请在[官方 Rustlings 仓库](https://github.com/rust-lang/rustlings/issues/new)报告问题，并附上操作系统信息，以及你是否在容器或虚拟机（例如 WSL）中运行 Rustlings。

{% end %}

## 练习列表

启动 `rustlings` 后，在监视模式中输入 `l`，可以打开交互式练习列表。

在列表中，你可以：

- 查看所有练习的状态（已完成或待完成）；
- `c`：从另一个练习继续（临时跳过某些练习，或返回之前的练习）；
- `r`：重置选中练习的状态和文件（之后需要在编辑器中_重新加载或重新打开_该文件）。

所有可用按键都显示在列表底部。

## 有问题？

如果练习过程中需要帮助，而内置提示又无法解决问题，欢迎先查看或前往 [Q&A 讨论区](https://github.com/rust-lang/rustlings/discussions/categories/q-a?discussions_q=)提问 💡

## 继续学习

完成 Rustlings 后，把学到的知识运用起来吧！
你可以构建自己的项目、参与 Rustlings 贡献，或寻找其他开源项目继续练习 Rust 技能。

> 如果你想创建自己的 Rustlings 练习，请参阅[**社区练习**](@/community-exercises/index.md)页面 🏗️
