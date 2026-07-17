# Implementation Plan: Rustlings 6.x Chinese Initialization Guidance

## 1. Correct the learner documentation

- Replace the source-checkout quick start in `README.zh-CN.md` with the `cargo install --git` flow.
- Add a separate contributor section explaining that the source checkout is not the learner workspace.
- Keep the existing command reference and translation contribution checks.

## 2. Improve the CLI diagnostic

- Update `OLD_METHOD_ERR` in `src/main.rs` with the exact Chinese recovery commands.
- Link to `README.zh-CN.md` and the official Rustlings getting-started page.
- Add a root README language pointer if needed so the package's default README remains discoverable.

## 3. Verify and publish

- Run `cargo fmt --all -- --check`, `cargo test --all-targets`, and both translation checks.
- Build and run the release binary in the source checkout to verify the diagnostic.
- Run initialization in an isolated temporary directory and inspect the generated structure.
- Commit and publish the change to `AreChen/rustlings-cn`.
