use anyhow::{Context, Result, bail};
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};
use serde::Deserialize;
use std::{
    env::{current_dir, set_current_dir},
    fs::{self, create_dir},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    cargo_toml::updated_cargo_toml, embedded::EMBEDDED_FILES, exercise::RunnableExercise,
    info_file::InfoFile, term::press_enter_prompt,
};

#[derive(Deserialize)]
struct CargoLocateProject<'a> {
    #[serde(borrow)]
    root: &'a Path,
}

pub fn init() -> Result<()> {
    let rustlings_dir = Path::new("rustlings");
    if rustlings_dir.exists() {
        bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
    }

    let locate_project_output = Command::new("cargo")
        .arg("locate-project")
        .arg("-q")
        .arg("--workspace")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context(
            "运行命令 `cargo locate-project …` 失败\n\
             是否已经安装 Rust？\n\
             请运行 `cargo --version` 来诊断问题。",
        )?;

    if !Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("运行命令 `cargo clippy --version` 失败")?
        .success()
    {
        bail!(
            "缺少官方 Rust lint 工具 Clippy。\n\
             请先安装 Clippy，再初始化 Rustlings。"
        )
    }

    let mut stdout = io::stdout().lock();
    let mut init_git = true;

    if locate_project_output.status.success() {
        if Path::new("exercises").exists() && Path::new("solutions").exists() {
            bail!(IN_INITIALIZED_DIR_ERR);
        }

        let workspace_manifest =
            serde_json::de::from_slice::<CargoLocateProject>(&locate_project_output.stdout)
                .context("无法从 `cargo locate-project …` 的输出中读取字段 `root`")?
                .root;

        let workspace_manifest_content = fs::read_to_string(workspace_manifest)
            .with_context(|| format!("无法读取文件 {}", workspace_manifest.display()))?;
        if !workspace_manifest_content.contains("[workspace]")
            && !workspace_manifest_content.contains("workspace.")
        {
            bail!(
                "当前目录已经属于一个 Cargo 项目。\n\
                 请在其他目录中初始化 Rustlings。"
            );
        }

        stdout.write_all(
            "此命令会在当前 Cargo 工作区中创建 `rustlings/` 目录，并将其作为工作区成员。\n\
                           按 ENTER 继续 "
                .as_bytes(),
        )?;
        press_enter_prompt(&mut stdout)?;

        // Make sure "rustlings" is added to `workspace.members` by making
        // Cargo initialize a new project.
        let status = Command::new("cargo")
            .arg("new")
            .arg("-q")
            .arg("--vcs")
            .arg("none")
            .arg("rustlings")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        if !status.success() {
            bail!(
                "初始化新的 Cargo 工作区成员失败。\n\
                 请在其他目录中初始化 Rustlings。"
            );
        }

        stdout.write_all("目录 `rustlings` 已添加到当前 Cargo 工作区的 `Cargo.toml` 文件的 `workspace.members` 中。\n".as_bytes())?;
        fs::remove_dir_all(rustlings_dir).context("删除临时目录 `rustlings/` 失败")?;
        init_git = false;
    } else {
        stdout.write_all(
            "此命令会创建包含练习的 `rustlings/` 目录。\n\
                           按 ENTER 继续 "
                .as_bytes(),
        )?;
        press_enter_prompt(&mut stdout)?;
    }

    create_dir(rustlings_dir).context("创建 `rustlings/` 目录失败")?;
    set_current_dir(rustlings_dir).context("切换当前目录到 `rustlings/` 失败")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("初始化 `rustlings/exercises` 目录失败")?;

    create_dir("solutions").context("创建 `solutions/` 目录失败")?;
    fs::write(
        "solutions/README.md",
        include_bytes!("../solutions/README.md"),
    )
    .context("创建文件 rustlings/solutions/README.md 失败")?;
    for dir in EMBEDDED_FILES.exercise_dirs {
        let mut dir_path = String::with_capacity(10 + dir.name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir(&dir_path).with_context(|| format!("创建目录 {dir_path} 失败"))?;
    }
    for exercise_info in &info_file.exercises {
        let solution_path = exercise_info.sol_path();
        fs::write(&solution_path, INIT_SOLUTION_FILE)
            .with_context(|| format!("创建文件 {solution_path} 失败"))?;
    }

    let current_cargo_toml = include_str!("../dev-Cargo.toml");
    // Skip the first line (comment).
    let newline_ind = current_cargo_toml
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("嵌入的 `Cargo.toml` 为空或只包含一行")?;
    let current_cargo_toml = current_cargo_toml
        .get(newline_ind + 1..)
        .context("嵌入的 `Cargo.toml` 只包含一行")?;
    let updated_cargo_toml = updated_cargo_toml(&info_file.exercises, current_cargo_toml, b"")
        .context("生成 `Cargo.toml` 失败")?;
    fs::write("Cargo.toml", updated_cargo_toml).context("创建文件 `rustlings/Cargo.toml` 失败")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("创建文件 `rustlings/rust-analyzer.toml` 失败")?;

    fs::write(".gitignore", GITIGNORE).context("创建文件 `rustlings/.gitignore` 失败")?;

    create_dir(".vscode").context("创建目录 `rustlings/.vscode` 失败")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("创建文件 `rustlings/.vscode/extensions.json` 失败")?;

    if init_git && let Ok(dir) = current_dir() {
        let mut dir = dir.as_path();

        loop {
            if dir.join(".git").exists() || dir.join(".jj").exists() {
                break;
            }

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                // Ignore any Git error because Git initialization is not required.
                let _ = Command::new("git")
                    .arg("init")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                break;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("初始化完成 ✓".as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n\n")?;

    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(POST_INIT_MSG)?;
    stdout.queue(ResetColor)?;

    Ok(())
}

const INIT_SOLUTION_FILE: &[u8] = r#"fn main() {
    // 不要编辑此解答文件！
    // 完成练习后，系统会自动填充它。
}
"#
.as_bytes();

pub const RUST_ANALYZER_TOML: &[u8] = br#"check.command = "clippy"
check.extraArgs = ["--profile", "test"]
cargo.targetDir = true
"#;

const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const IN_INITIALIZED_DIR_ERR: &str = "Rustlings 似乎已经在此目录中初始化。

如果已经初始化 Rustlings，请运行命令 `rustlings` 查看开始练习的说明。
否则，请在其他目录中再次运行 `rustlings init`。";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str = "当前目录中已经存在名为 `rustlings` 的目录。
你可能已经初始化过 Rustlings。
请运行 `cd rustlings`
然后再次运行 `rustlings`。";

const POST_INIT_MSG: &[u8] = r#"运行 `cd rustlings` 进入生成的目录。
然后运行 `rustlings` 开始练习。
"#
.as_bytes();
