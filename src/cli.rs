use clap::{ArgAction, Parser, Subcommand};

use crate::dev::DevCommand;

/// Rustlings 是一组帮助你熟悉 Rust 代码编写和阅读的小练习。
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// 禁用在 VS Code 或 Zellij 中自动打开当前文件。
    /// 此选项会忽略 `--edit-cmd`。
    #[arg(long)]
    pub no_editor: bool,
    /// 运行 `EDIT_CMD EXERCISE_PATH` 打开当前练习。
    /// 此命令不能阻塞（例如 `vim`）。
    /// 它应与另一个进程中的编辑器通信。
    /// `EDIT_CMD` 可以包含参数，例如 `--edit-cmd "PROGRAM -x --arg1"`。
    /// Rustlings 会将当前练习的路径作为最后一个参数添加到命令中。
    /// 在 VS Code 中会忽略 `--edit-cmd`。
    ///
    /// 示例：`--edit-cmd "code"`（在 VS Code 终端中运行时的默认行为）
    #[arg(long)]
    pub edit_cmd: Option<String>,
    /// 在监视模式下使用 `r` 手动运行当前练习。
    /// 仅当 Rustlings 无法检测练习文件变化时使用。
    #[arg(long)]
    pub manual_run: bool,
    #[arg(short = 'h', long = "help", action = ArgAction::Help, help = "显示帮助")]
    pub help: Option<bool>,
    #[arg(short = 'V', long = "version", action = ArgAction::Version, help = "显示版本")]
    pub version: Option<bool>,
}

#[derive(Subcommand)]
pub enum Command {
    /// 初始化官方 Rustlings 练习
    Init,
    /// 运行单个练习。
    /// 如果未指定练习名称，则运行下一个待完成的练习。
    Run {
        /// 练习名称
        name: Option<String>,
    },
    /// 检查所有练习，并相应地标记为已完成或待完成
    CheckAll,
    /// 重置单个练习
    Reset {
        /// 练习名称
        name: String,
    },
    /// 显示提示。
    /// 如果未指定练习名称，则显示下一个待完成练习的提示。
    Hint {
        /// 练习名称
        name: Option<String>,
    },
    /// 用于开发（社区）Rustlings 练习的命令
    #[command(subcommand)]
    Dev(DevCommand),
}
