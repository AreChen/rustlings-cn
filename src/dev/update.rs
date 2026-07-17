use anyhow::{Context, Result};
use std::fs;

use crate::{
    cargo_toml::updated_cargo_toml,
    info_file::{ExerciseInfo, InfoFile},
};

// Update the `Cargo.toml` file.
fn update_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let current_cargo_toml = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("读取文件 `{cargo_toml_path}` 失败"))?;

    let updated_cargo_toml =
        updated_cargo_toml(exercise_infos, &current_cargo_toml, exercise_path_prefix)?;

    fs::write(cargo_toml_path, updated_cargo_toml).context("写入 `Cargo.toml` 文件失败")?;

    Ok(())
}

pub fn update() -> Result<()> {
    let info_file = InfoFile::parse()?;

    if cfg!(debug_assertions) {
        // A hack to make `cargo dev update` work when developing Rustlings.
        update_cargo_toml(&info_file.exercises, "dev/Cargo.toml", b"../")
            .context("更新文件 `dev/Cargo.toml` 失败")?;

        println!("已更新 `dev/Cargo.toml`");
    } else {
        update_cargo_toml(&info_file.exercises, "Cargo.toml", &[])
            .context("更新文件 `Cargo.toml` 失败")?;

        println!("已更新 `Cargo.toml`");
    }

    Ok(())
}
