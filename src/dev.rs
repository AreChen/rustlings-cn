use anyhow::{Context, Result, bail};
use clap::Subcommand;
use std::path::PathBuf;

mod check;
mod new;
mod update;

#[derive(Subcommand)]
pub enum DevCommand {
    /// 为社区练习创建新项目
    New {
        /// 创建项目的路径
        path: PathBuf,
        /// 不要在项目目录中初始化 Git 仓库
        #[arg(long)]
        no_git: bool,
    },
    /// 检查练习
    Check {
        /// 要求每个练习都有解答
        #[arg(short, long)]
        require_solutions: bool,
    },
    /// 更新练习的 `Cargo.toml` 文件
    Update,
}

impl DevCommand {
    pub fn run(self) -> Result<()> {
        match self {
            Self::New { path, no_git } => {
                if cfg!(debug_assertions) {
                    bail!("调试构建中禁用此命令");
                }

                new::new(&path, no_git).context(INIT_ERR)
            }
            Self::Check { require_solutions } => check::check(require_solutions),
            Self::Update => update::update(),
        }
    }
}

const INIT_ERR: &str = "初始化失败。
解决问题后，请删除 `rustlings` 目录（如果已经创建），然后重试";
