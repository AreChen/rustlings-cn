use anyhow::{Context, Result, bail};
use app_state::StateFileStatus;
use clap::Parser;
use std::{
    env,
    io::{self, IsTerminal, Write},
    path::Path,
    process::ExitCode,
};
use term::{clear_terminal, press_enter_prompt};

use crate::{
    app_state::AppState,
    cli::{Args, Command},
    editor::Editor,
    info_file::InfoFile,
};

mod app_state;
mod cargo_toml;
mod cli;
mod cmd;
mod dev;
mod editor;
mod embedded;
mod exercise;
mod info_file;
mod init;
mod list;
mod run;
mod term;
mod watch;

const CURRENT_FORMAT_VERSION: u8 = 1;

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    if cfg!(not(debug_assertions)) && Path::new("dev/rustlings-repo.txt").exists() {
        bail!("{OLD_METHOD_ERR}");
    }

    'priority_cmd: {
        match args.command {
            Some(Command::Init) => init::init().context("初始化失败")?,
            Some(Command::Dev(dev_command)) => dev_command.run()?,
            _ => break 'priority_cmd,
        }

        return Ok(ExitCode::SUCCESS);
    }

    if !Path::new("exercises").is_dir() {
        println!("{PRE_INIT_MSG}");
        return Ok(ExitCode::FAILURE);
    }

    let info_file = InfoFile::parse()?;

    if info_file.format_version > CURRENT_FORMAT_VERSION {
        bail!(FORMAT_VERSION_HIGHER_ERR);
    }

    let vs_code_term = env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode");
    let editor = if args.no_editor {
        None
    } else {
        Editor::new(args.edit_cmd, vs_code_term)?
    };

    let (mut app_state, state_file_status) = AppState::new(
        info_file.exercises,
        info_file.final_message.unwrap_or_default(),
        editor,
        vs_code_term,
    )?;

    // Show the welcome message if the state file doesn't exist yet.
    if let Some(welcome_message) = info_file.welcome_message {
        match state_file_status {
            StateFileStatus::NotRead => {
                let mut stdout = io::stdout().lock();
                clear_terminal(&mut stdout)?;

                let welcome_message = welcome_message.trim_ascii();
                write!(
                    stdout,
                    "{welcome_message}\n\n\
                     按 ENTER 继续 "
                )?;
                press_enter_prompt(&mut stdout)?;
                clear_terminal(&mut stdout)?;
                // Flush to be able to show errors occurring before printing a newline to stdout.
                stdout.flush()?;
            }
            StateFileStatus::Read => (),
        }
    }

    match args.command {
        None => {
            if !io::stdout().is_terminal() {
                bail!("不支持的终端，或缺少终端/TTY");
            }

            let notify_exercise_names = if args.manual_run {
                None
            } else {
                // For the notify event handler thread.
                // Leaking is fine since the slice is used until the end of the program.
                Some(
                    &*app_state
                        .exercises()
                        .iter()
                        .map(|exercise| exercise.name.as_bytes())
                        .collect::<Vec<_>>()
                        .leak(),
                )
            };

            watch::watch(&mut app_state, notify_exercise_names)?;
            app_state.close_editor()?;
        }
        Some(Command::Run { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }
            return run::run(&mut app_state);
        }
        Some(Command::CheckAll) => {
            let mut stdout = io::stdout().lock();
            if let Some(first_pending_exercise_ind) = app_state.check_all_exercises(&mut stdout)? {
                if app_state.current_exercise().done {
                    app_state.set_current_exercise_ind(first_pending_exercise_ind)?;
                }

                stdout.write_all(b"\n\n")?;
                let pending = app_state.n_pending();
                if pending == 1 {
                    stdout.write_all("有一个练习待完成：".as_bytes())?;
                } else {
                    write!(
                        stdout,
                        "{pending}/{} 个练习待完成。第一个：",
                        app_state.exercises().len(),
                    )?;
                }
                app_state
                    .current_exercise()
                    .terminal_file_link(&mut stdout, app_state.emit_file_links())?;
                stdout.write_all(b"\n")?;

                return Ok(ExitCode::FAILURE);
            }

            app_state.render_final_message(&mut stdout)?;
        }
        Some(Command::Reset { name }) => {
            app_state.set_current_exercise_by_name(&name)?;
            app_state.reset_current_exercise()?;

            let current_exercise = app_state.current_exercise();
            let mut stdout = io::stdout().lock();
            stdout.write_all("练习 ".as_bytes())?;
            current_exercise.terminal_file_link(&mut stdout, app_state.emit_file_links())?;
            stdout.write_all(" 已重置\n".as_bytes())?;
        }
        Some(Command::Hint { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }

            let current_exercise = app_state.current_exercise();
            let mut stdout = io::stdout().lock();
            stdout.write_all("当前练习：".as_bytes())?;
            current_exercise.terminal_file_link(&mut stdout, app_state.emit_file_links())?;

            stdout.write_all("\n\n提示：\n".as_bytes())?;
            stdout.write_all(current_exercise.hint.as_bytes())?;
            stdout.write_all(b"\n")?;
        }
        // Handled in an earlier match.
        Some(Command::Init | Command::Dev(_)) => (),
    }

    Ok(ExitCode::SUCCESS)
}

const OLD_METHOD_ERR: &str = concat!(
    "你正在 Rustlings 源码仓库中运行 `rustlings`。\n\
     Rustlings 6.0 及更高版本不需要克隆源码仓库来学习，也不能在源码仓库目录中运行 `rustlings init`。\n\
     \n\
     请在源码仓库之外创建练习目录，然后执行：\n\
     ",
    // translation: preserve CLI command examples
    "    cargo install --git https://github.com/AreChen/rustlings-cn.git --branch main --locked --force\n\
     mkdir rustlings-cn-study\n\
     cd rustlings-cn-study\n\
     rustlings init\n\
     cd rustlings\n\
     rustlings\n\
     \n\
     ",
    "上一步初始化没有执行，所以此时不要运行 `cd rustlings`。\n\
     中文说明：https://github.com/AreChen/rustlings-cn/blob/main/README.zh-CN.md\n\
     官方说明：https://github.com/rust-lang/rustlings#getting-started"
);

const FORMAT_VERSION_HIGHER_ERR: &str = "`info.toml` 文件中指定的格式版本高于当前支持的最高版本。
你使用的 Rustlings 可能已经过时。
请先安装最新版本的 Rustlings。";

const PRE_INIT_MSG: &str = r"
       欢迎来到……
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/

在当前目录中找不到 `exercises/` 目录。
如果你刚开始使用 Rustlings，请运行 `rustlings init` 命令进行初始化。";
