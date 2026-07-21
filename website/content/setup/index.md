+++
title = "安装"
+++

<!-- toc -->

## 安装 Rust

安装 Rustlings 之前，必须先安装**最新版本的 Rust**。
请参阅 [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 了解安装方法。
该过程也会安装 Rust 的包和项目管理器 _Cargo_。

> 🐧 如果你使用 **Linux**，请确保已安装 `gcc`（用作链接器）。
>
> Debian：`sudo apt install gcc`\
> Fedora：`sudo dnf install gcc`
>
> 🍎 如果你使用 **macOS**，请确保已安装 _Xcode 和开发者工具_：`xcode-select --install`

## 安装 Rustlings

以下命令会从我们的 fork 下载并编译 Rustlings 中文版：

```bash
cargo install --git https://github.com/AreChen/rustlings-cn.git --branch main --locked --force
```

{% details(summary="如果安装失败……") %}

- 运行 `rustup update`，确保使用最新版本的 Rust。
- 确认命令中包含 `--locked` 参数。
- 如果仍然失败，请在[官方 Rustlings 仓库](https://github.com/rust-lang/rustlings/issues/new)报告问题。

{% end %}

## 初始化练习目录

安装 Rustlings 后，运行以下命令初始化 `rustlings/` 目录：

```bash
rustlings init
```

{% details(summary="如果找不到 <code>rustlings</code> 命令……") %}

你可能使用的是 Linux，并且通过系统包管理器安装了 Rust。

Cargo 会将二进制文件安装到 `~/.cargo/bin` 目录。
遗憾的是，包管理器通常不会自动把 `~/.cargo/bin` 加入 `PATH` 环境变量。

- 手动将 `~/.cargo/bin` 加入 `PATH`；
- 或者卸载包管理器安装的 Rust，再按照官方方式使用 [`rustup`](https://www.rust-lang.org/tools/install) 安装。

{% end %}

现在进入刚刚初始化的目录并启动 Rustlings，查看练习入门说明：

```bash
cd rustlings/
rustlings
```

## 工作环境

### 编辑器

我们通常推荐使用 [VS Code](https://code.visualstudio.com/) 和 [rust-analyzer 插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。
不过，只要编辑器支持 [rust-analyzer](https://rust-analyzer.github.io/)，就足以完成这些练习。

### 终端

使用 Rustlings 时，请选择现代终端，以获得更好的体验。
Linux 和 macOS 的默认终端通常已经足够。
Windows 用户推荐使用 [Windows Terminal](https://aka.ms/terminal)。

### 离线文档

离线使用 Rustlings 时，可以运行 `rustup doc --book` 或 `rustup doc --std`，访问本地的 Rust 语言书或标准库文档。

## 开始使用

完成安装后，请前往[**使用**](@/usage/index.md)页面，了解 Rustlings 的使用方法 🚀
