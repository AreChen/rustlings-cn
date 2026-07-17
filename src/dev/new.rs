use anyhow::{Context, Result, bail};
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    path::Path,
    process::Command,
};

use crate::{CURRENT_FORMAT_VERSION, init::RUST_ANALYZER_TOML};

// 创建相对于当前目录的目录，并打印其路径。
fn create_rel_dir(dir_name: &str, current_dir: &str) -> Result<()> {
    create_dir(dir_name).with_context(|| format!("创建目录 {current_dir}/{dir_name} 失败"))?;
    println!("已创建目录 {current_dir}/{dir_name}");
    Ok(())
}

// 写入相对于当前目录的文件，并打印其路径。
fn write_rel_file<C>(file_name: &str, current_dir: &str, content: C) -> Result<()>
where
    C: AsRef<[u8]>,
{
    fs::write(file_name, content)
        .with_context(|| format!("创建文件 {current_dir}/{file_name} 失败"))?;
    // 用空格与 `create_rel_dir` 的输出对齐。
    println!("已创建文件      {current_dir}/{file_name}");
    Ok(())
}

pub fn new(path: &Path, no_git: bool) -> Result<()> {
    let dir_path_str = path.to_string_lossy();

    create_dir(path).with_context(|| format!("创建目录 {dir_path_str} 失败"))?;
    println!("已创建目录 {dir_path_str}");

    set_current_dir(path).with_context(|| format!("将 {dir_path_str} 设置为当前目录失败"))?;

    if !no_git
        && !Command::new("git")
            .arg("init")
            .status()
            .context("运行 `git init` 失败")?
            .success()
    {
        bail!("`git init` 执行未成功。请查看上方可能的错误信息");
    }

    write_rel_file(".gitignore", &dir_path_str, GITIGNORE)?;

    create_rel_dir("exercises", &dir_path_str)?;
    create_rel_dir("solutions", &dir_path_str)?;

    write_rel_file(
        "info.toml",
        &dir_path_str,
        format!(
            "{INFO_FILE_BEFORE_FORMAT_VERSION}{CURRENT_FORMAT_VERSION}{INFO_FILE_AFTER_FORMAT_VERSION}"
        ),
    )?;

    write_rel_file("Cargo.toml", &dir_path_str, CARGO_TOML)?;

    write_rel_file("README.md", &dir_path_str, README)?;

    write_rel_file("rust-analyzer.toml", &dir_path_str, RUST_ANALYZER_TOML)?;

    create_rel_dir(".vscode", &dir_path_str)?;
    write_rel_file(
        ".vscode/extensions.json",
        &dir_path_str,
        crate::init::VS_CODE_EXTENSIONS_JSON,
    )?;

    println!("\n初始化完成 ✓");

    Ok(())
}

pub const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
!.vscode/extensions.json
";

const INFO_FILE_BEFORE_FORMAT_VERSION: &str =
    "# 格式版本用于表示社区练习与 Rustlings 程序之间的兼容性。
# 格式版本与 Rustlings 程序版本并不相同。
# 如果 Rustlings 对社区练习所需格式做出不可避免的不兼容变更，
# 你需要提升此版本并调整格式。
# 否则，最新版本的 Rustlings 程序将无法运行这些练习。
format_version = ";

const INFO_FILE_AFTER_FORMAT_VERSION: &str = r#"

# 可选的多行消息：学习者刚开始练习时显示。
welcome_message = """欢迎来到这些社区 Rustlings 练习。"""

# 可选的多行消息：完成所有练习后显示。
final_message = """希望这些练习对你有所帮助 :D"""

# 每个练习重复此部分。
[[exercises]]
# 练习名称，即去掉 `.rs` 扩展名后的练习文件名。
name = "???"

# 可选的目录名称，用于将练习组织到不同目录中。
# 如果指定 `dir`，练习路径为 `exercises/DIR/NAME.rs`
# 否则，路径为 `exercises/NAME.rs`
# dir = "???"

# Rustlings 要求练习包含测试并运行这些测试。
# 你可以将 `test` 设置为 `false` 来禁用测试（默认为 `true`）。
# 在这种情况下，练习只要成功编译就会被视为完成。
# test = true

# Rustlings 始终会对练习运行 Clippy。
# 你可以将 `strict_clippy` 设置为 `true`（默认为 `false`），
# 这样只有在没有任何警告时练习才会被视为完成。
# strict_clippy = false

# 可选的多行提示，学习者请求时显示。
hint = """???"""
"#;

const CARGO_TOML: &[u8] = r#"# 不要手动编辑 `bin` 列表！它会由 `rustlings dev update` 更新
bin = []

[package]
name = "exercises"
edition = "2024"
# 不要将练习发布到 crates.io！
publish = false

[dependencies]
"#
.as_bytes();

const README: &str = "# Rustlings 🦀 练习

欢迎来到这些社区 Rustlings 练习 😃

首先，[按照官方说明安装 Rustlings](https://github.com/rust-lang/rustlings) ✅

然后，克隆此仓库，在此目录中打开终端并运行 `rustlings`，开始练习 🚀
";
