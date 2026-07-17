use anyhow::{Context, Error, Result, anyhow, bail};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::{self, OpenOptions, read_dir},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use crate::{
    CURRENT_FORMAT_VERSION,
    cargo_toml::{BINS_BUFFER_CAPACITY, append_bins, bins_start_end_ind},
    cmd::CmdRunner,
    exercise::{OUTPUT_CAPACITY, RunnableExercise},
    info_file::{ExerciseInfo, InfoFile},
    term::ProgressCounter,
};

const MAX_N_EXERCISES: usize = 999;
const MAX_EXERCISE_NAME_LEN: usize = 32;

// Find a char that isn't allowed in the exercise's `name` or `dir`.
fn forbidden_char(input: &str) -> Option<char> {
    input.chars().find(|c| !c.is_alphanumeric() && *c != '_')
}

// Check that the `Cargo.toml` file is up-to-date.
fn check_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let current_cargo_toml = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("读取文件 `{cargo_toml_path}` 失败"))?;

    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(&current_cargo_toml)?;

    let old_bins = &current_cargo_toml.as_bytes()[bins_start_ind..bins_end_ind];
    let mut new_bins = Vec::with_capacity(BINS_BUFFER_CAPACITY);
    append_bins(&mut new_bins, exercise_infos, exercise_path_prefix);

    if old_bins != new_bins {
        if cfg!(debug_assertions) {
            bail!(
                "文件 `dev/Cargo.toml` 已过期。请运行 `cargo dev update` 更新，然后再次运行 `cargo run -- dev check`"
            );
        }

        bail!(
            "文件 `Cargo.toml` 已过期。请运行 `rustlings dev update` 更新，然后再次运行 `rustlings dev check`"
        );
    }

    Ok(())
}

// Check the info of all exercises and return their paths in a set.
fn check_info_file_exercises(info_file: &InfoFile) -> Result<HashSet<PathBuf>> {
    let mut names = HashSet::with_capacity(info_file.exercises.len());
    let mut paths = HashSet::with_capacity(info_file.exercises.len());

    let mut file_buf = String::with_capacity(1 << 14);
    for exercise_info in &info_file.exercises {
        let name = exercise_info.name;
        if name.is_empty() {
            bail!("`info.toml` 中存在空的练习名称");
        }
        if name.len() > MAX_EXERCISE_NAME_LEN {
            bail!("练习名称 `{name}` 的长度超过上限 {MAX_EXERCISE_NAME_LEN}");
        }
        if let Some(c) = forbidden_char(name) {
            bail!("练习名称 `{name}` 中的字符 `{c}` 不被允许");
        }

        if let Some(dir) = exercise_info.dir {
            if dir.is_empty() {
                bail!("练习 `{name}` 在 `info.toml` 中的目录名称为空");
            }
            if let Some(c) = forbidden_char(dir) {
                bail!("练习目录 `{dir}` 中的字符 `{c}` 不被允许");
            }
        }

        if exercise_info.hint.trim_ascii().is_empty() {
            bail!("练习 `{name}` 的提示为空。请提供提示，或至少告诉学习者为什么这个练习不需要提示");
        }

        if !names.insert(name) {
            bail!("练习名称 `{name}` 重复。所有练习名称必须唯一");
        }

        let path = exercise_info.path();

        OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("打开文件 {path} 失败"))?
            .read_to_string(&mut file_buf)
            .with_context(|| format!("读取文件 {path} 失败"))?;

        if !file_buf.contains("fn main()") {
            bail!(
                "文件 `{path}` 缺少 `main` 函数。\n\
                 请至少创建一个空的 `main` 函数，以避免语言服务器报错"
            );
        }

        if !file_buf.contains("// TODO") {
            bail!(
                "在文件 `{path}` 中没有找到 `// TODO` 注释。\n\
                 至少需要一条这样的注释来引导学习者。"
            );
        }

        let contains_tests = file_buf.contains("#[test]\n");
        if exercise_info.test {
            if !contains_tests {
                bail!(
                    "文件 `{path}` 不包含任何测试。如果你不想为此练习添加测试，请在 `info.toml` 中将该练习设置为 `test = false`"
                );
            }
        } else if contains_tests {
            bail!(
                "文件 `{path}` 包含使用 `#[test]` 标注的测试，但练习 `{name}` 在 `info.toml` 中设置了 `test = false`"
            );
        }

        file_buf.clear();

        paths.insert(PathBuf::from(path));
    }

    Ok(paths)
}

// Check `dir` for unexpected files.
// Only Rust files in `allowed_rust_files` and `README.md` files are allowed.
// Only one level of directory nesting is allowed.
fn check_unexpected_files(dir: &str, allowed_rust_files: &HashSet<PathBuf>) -> Result<()> {
    let unexpected_file = |path: &Path| {
        anyhow!(
            "发现文件 `{}`。`{dir}` 目录中只允许存在 `README.md` 和 `info.toml` 中练习对应的 Rust 文件",
            path.display()
        )
    };

    for entry in read_dir(dir).with_context(|| format!("打开 `{dir}` 目录失败"))? {
        let entry = entry.with_context(|| format!("读取 `{dir}` 目录失败"))?;

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !allowed_rust_files.contains(&path) {
                return Err(unexpected_file(&path));
            }

            continue;
        }

        let dir_path = entry.path();
        for entry in
            read_dir(&dir_path).with_context(|| format!("打开目录 {} 失败", dir_path.display()))?
        {
            let entry = entry.with_context(|| format!("读取目录 {} 失败", dir_path.display()))?;
            let path = entry.path();

            if !entry.file_type().unwrap().is_file() {
                bail!(
                    "发现 `{}`，但此处应只有文件。练习目录最多只能嵌套一层",
                    path.display()
                );
            }

            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !allowed_rust_files.contains(&path) {
                return Err(unexpected_file(&path));
            }
        }
    }

    Ok(())
}

fn check_exercises_unsolved(
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all("正在运行所有练习，以检查它们是否已经解出……\n".as_bytes())?;

    let handles = info_file
        .exercises
        .iter()
        .filter_map(|exercise_info| {
            if exercise_info.skip_check_unsolved {
                return None;
            }

            Some(
                thread::Builder::new()
                    .spawn(|| exercise_info.run_exercise(None, cmd_runner))
                    .map(|handle| (exercise_info.name, handle)),
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .context("创建用于检查练习是否已经解出的线程失败")?;

    let mut progress_counter = ProgressCounter::new(&mut stdout, handles.len())?;

    for (exercise_name, handle) in handles {
        let Ok(result) = handle.join() else {
            bail!("运行练习 {exercise_name} 时发生 panic");
        };

        match result {
            Ok(true) => {
                bail!(
                    "练习 {exercise_name} 已经解出。\n\
                     {SKIP_CHECK_UNSOLVED_HINT}",
                )
            }
            Ok(false) => (),
            Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    Ok(())
}

fn check_exercises(info_file: &'static InfoFile, cmd_runner: &'static CmdRunner) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!(
            "`format_version` < {CURRENT_FORMAT_VERSION}（支持的版本）\n\
             请迁移到最新格式版本"
        ),
        Ordering::Greater => bail!(
            "`format_version` > {CURRENT_FORMAT_VERSION}（支持的版本）\n\
             请尝试更新 Rustlings 程序"
        ),
        Ordering::Equal => (),
    }

    let handle = thread::Builder::new()
        .spawn(move || check_exercises_unsolved(info_file, cmd_runner))
        .context("创建用于检查练习是否已经解出的线程失败")?;

    let info_file_paths = check_info_file_exercises(info_file)?;
    check_unexpected_files("exercises", &info_file_paths)?;

    handle.join().unwrap()
}

enum SolutionCheck {
    Success { sol_path: String },
    MissingOptional,
    RunFailure { output: Vec<u8> },
    Err(Error),
}

fn check_solutions(
    require_solutions: bool,
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all("正在运行所有解答……\n".as_bytes())?;

    let handles = info_file
        .exercises
        .iter()
        .map(|exercise_info| {
            thread::Builder::new().spawn(move || {
                let sol_path = exercise_info.sol_path();
                if !Path::new(&sol_path).exists() {
                    if require_solutions {
                        return SolutionCheck::Err(anyhow!(
                            "练习 {} 的解答缺失",
                            exercise_info.name,
                        ));
                    }

                    return SolutionCheck::MissingOptional;
                }

                let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
                match exercise_info.run_solution(Some(&mut output), cmd_runner) {
                    Ok(true) => SolutionCheck::Success { sol_path },
                    Ok(false) => SolutionCheck::RunFailure { output },
                    Err(e) => SolutionCheck::Err(e),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("创建用于检查解答的线程失败")?;

    let mut sol_paths = HashSet::with_capacity(info_file.exercises.len());
    let mut fmt_cmd = Command::new("rustfmt");
    fmt_cmd
        .arg("--check")
        .arg("--edition")
        .arg("2024")
        .arg("--color")
        .arg("always")
        .stdin(Stdio::null());

    let mut progress_counter = ProgressCounter::new(&mut stdout, handles.len())?;

    for (exercise_info, handle) in info_file.exercises.iter().zip(handles) {
        let Ok(check_result) = handle.join() else {
            bail!("运行练习 {} 的解答时发生 panic", exercise_info.name,);
        };

        match check_result {
            SolutionCheck::Success { sol_path } => {
                fmt_cmd.arg(&sol_path);
                sol_paths.insert(PathBuf::from(sol_path));
            }
            SolutionCheck::MissingOptional => (),
            SolutionCheck::RunFailure { output } => {
                drop(progress_counter);
                stdout.write_all(&output)?;
                bail!("运行练习 {} 的解答失败，错误信息见上方", exercise_info.name,);
            }
            SolutionCheck::Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    let n_solutions = sol_paths.len();
    let handle = thread::Builder::new()
        .spawn(move || check_unexpected_files("solutions", &sol_paths))
        .context("创建用于检查 solutions 目录中异常文件的线程失败")?;

    if n_solutions > 0
        && !fmt_cmd
            .status()
            .context("对所有解答文件运行 `rustfmt` 失败")?
            .success()
    {
        bail!("部分解答未格式化。请对它们运行 `rustfmt`");
    }

    handle.join().unwrap()
}

pub fn check(require_solutions: bool) -> Result<()> {
    let info_file = InfoFile::parse()?;

    if info_file.exercises.len() > MAX_N_EXERCISES {
        bail!("练习数量上限为 {MAX_N_EXERCISES}");
    }

    if cfg!(debug_assertions) {
        // A hack to make `cargo dev check` work when developing Rustlings.
        check_cargo_toml(&info_file.exercises, "dev/Cargo.toml", b"../")?;
    } else {
        check_cargo_toml(&info_file.exercises, "Cargo.toml", b"")?;
    }

    // Leaking is fine since they are used until the end of the program.
    let cmd_runner = Box::leak(Box::new(CmdRunner::build()?));
    let info_file = Box::leak(Box::new(info_file));

    check_exercises(info_file, cmd_runner)?;
    check_solutions(require_solutions, info_file, cmd_runner)?;

    println!("检查通过！");

    Ok(())
}

const SKIP_CHECK_UNSOLVED_HINT: &str = "如果这是一个设计为初始即已解出的入门练习，请在 `info.toml` 的练习元数据中添加 `skip_check_unsolved = true`";
