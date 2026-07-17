# Design: Clarify the Rustlings 6.x Initialization Flow

**Date:** 2026-07-18  
**Status:** Approved

## Context

Rustlings 6.x embeds the exercises in the installed CLI. `rustlings init` is therefore intended to be run from a separate learning directory; it creates a child directory named `rustlings/`. The CLI deliberately rejects execution from a checked-out Rustlings source repository when it finds `dev/rustlings-repo.txt`.

The Chinese README currently tells readers to clone `rustlings-cn`, install from that checkout, and run `rustlings init` inside the source repository. That produces the old-method error. The error then points readers to `cd rustlings`, even though initialization was rejected and no directory was created.

## Goals

- Give learners a copyable Chinese installation flow that uses the fork's CLI without requiring a source checkout.
- Explain the separate source-checkout flow for contributors.
- Make the old-method CLI error actionable and point to the Chinese instructions.
- Preserve the upstream 6.x behavior and exercise layout.

## Decision

Update `README.zh-CN.md` with two explicitly separated flows:

1. Learners install the fork with `cargo install --git ... --branch main --locked --force`, create a separate study directory, run `rustlings init`, then enter the generated `rustlings/` directory.
2. Contributors clone `rustlings-cn` for translation and development, but initialize learner exercises outside that checkout.

Update the CLI's old-method message to show the same commands and explain why the current directory is rejected. Add a short language/navigation note to the root README so the package's `README.md` link leads Chinese readers to the maintained instructions.

## Non-goals

- Do not change the Rustlings initialization algorithm.
- Do not duplicate or fork the embedded exercises at runtime.
- Do not change the upstream command names or exercise behavior.

## Verification

- Run Rust tests, formatting, and translation checks.
- Build the release binary and execute `rustlings init` from the source checkout to verify the new diagnostic.
- Use a temporary directory to run `rustlings init` successfully and confirm it creates the expected child directory.
- Check that the source checkout remains clean apart from intentional documentation and message changes.
