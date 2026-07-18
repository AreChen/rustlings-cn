# Design: Translate Rustlings Interactive Learning Messages

**Date:** 2026-07-18
**Status:** Approved

## Context

The Rustlings CLI loads its official exercise metadata from `rustlings-macros/info.toml` when no local `info.toml` exists. That metadata contains the first-time welcome message, the final message, and one hint for each of the 94 exercises. The Chinese fork translated the exercise source, README files, and Rust CLI strings, but left this embedded learner-facing metadata in English.

## Goals

- Translate the welcome message, final message, and all 94 exercise hints into natural Simplified Chinese.
- Preserve Rust identifiers, commands, exercise names, code fragments, URLs, and exact test-contract text.
- Add a regression check so future upstream syncs cannot silently reintroduce English-only interactive hints.
- Verify the translated metadata is embedded in the installed/debug CLI, not only present in the source file.

## Decision

Keep the upstream TOML schema and exercise ordering unchanged. Translate only the prose inside `welcome_message`, `final_message`, and each `hint` field. The existing `info.toml` is embedded by `rustlings-macros::include_files!()` and therefore remains the single runtime source of official hints.

Extend `scripts/test_check_translation.py` with a focused metadata test. It will parse `rustlings-macros/info.toml`, assert the expected 94 exercises, and reject any learner-facing line that contains English prose without Chinese after code spans and URLs are removed. This test complements the existing source/README coverage checker without changing upstream blob-baseline semantics.

## Non-goals

- Do not translate Rust syntax, commands, identifiers, URLs, exercise names, or exact output strings used by tests.
- Do not change the exercise logic, ordering, format version, or CLI behavior.
- Do not change community-exercise `info.toml` handling.

## Verification

- Observe the new metadata regression test fail before translation.
- Run the Python translation tests and coverage checker.
- Run Rust formatting, tests, and build checks.
- Run `cargo run -- hint intro2` and verify the hint is Chinese.
- Build/run the CLI from a clean generated exercise directory so the embedded metadata path is exercised.

