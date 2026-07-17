#!/usr/bin/env python3
"""Audit Rustlings Chinese translation coverage without network access.

The checker deliberately uses heuristics only for learner-facing prose. Rust
identifiers, code blocks, test contracts, command names, and data examples are
reported as preserved instead of being counted as translated.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any, Iterable


CHINESE_RE = re.compile(r"[\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff]")
ENGLISH_WORD_RE = re.compile(r"[A-Za-z]{3,}")
MARKER_RE = re.compile(r"translation\s*:\s*preserve\b(?P<reason>.*)", re.IGNORECASE)
# Rust raw strings may be plain (`r"..."`), byte strings (`br"..."`), or
# use one or more hash delimiters.  Keep the prefix in the match so normal
# string extraction does not start inside a raw byte string.
RAW_LITERAL_RE = re.compile(
    r'(?<![A-Za-z0-9_])(?P<prefix>b?r)(?P<hashes>#{0,})"(?P<raw>.*?)"(?P=hashes)',
    re.DOTALL,
)
NORMAL_LITERAL_RE = re.compile(r'"(?:\\.|[^"\\])*"', re.DOTALL)

EXERCISE_USER_MACROS = re.compile(
    r"\b(?:print|println|eprint|eprintln|panic|todo|unimplemented)!\s*\(",
)
SOURCE_USER_CONTEXT = re.compile(
    r"(?:\bbail!\s*\(|\.context\s*\(|\.with_context\s*\(|"
    r"\b(?:print|println|eprint|eprintln)!\s*\(|"
    r"\bwrite(?:_ascii|_all)?\s*\(|\bwrite!\s*\(|"
    r"const\s+[A-Z0-9_]*(?:ERR|MSG)[A-Z0-9_]*\s*:\s*&str)",
)
TEST_CONTEXT = re.compile(
    r"\b(?:assert(?:_eq|_ne)?|debug_assert(?:_eq|_ne)?|matches|should_panic)\b"
    r"|\.expect\s*\(",
)
STRUCTURAL_CONTEXT = re.compile(
    r"\b(?:Command::new|include_bytes!|include_str!|Path::new)\b"
    r"|\.arg\s*\(|\b(?:push_str|push)\s*\(",
)
MESSAGE_OUTPUT_CONTEXT = re.compile(r"\b(?:self\.)?message\b")
PATH_OR_TOKEN_RE = re.compile(r"^[A-Za-z0-9_./\\-]+$")

TECHNICAL_ONLY = {
    "api",
    "cargo",
    "clippy",
    "code",
    "github",
    "readme",
    "rust",
    "rustlings",
    "url",
    "vs",
    "zellij",
}


def contains_chinese(value: str) -> bool:
    return bool(CHINESE_RE.search(value))


def has_english_words(value: str) -> bool:
    return bool(ENGLISH_WORD_RE.search(value))


def marker_reason(line: str) -> str | None:
    match = MARKER_RE.search(line)
    if not match:
        return None
    reason = match.group("reason").strip()
    reason = re.sub(r"(?:-->|\*/)$", "", reason).strip(" :-")
    return reason or "explicitly preserved"


def strip_inline_markup(value: str) -> str:
    value = re.sub(r"`[^`]*`", "", value)
    value = re.sub(r"!?\[([^\]]+)\]\([^)]*\)", r"\1", value)
    value = re.sub(r"https?://\S+", "", value)
    value = re.sub(r"<[^>]+>", "", value)
    value = re.sub(r"[*_~]", "", value)
    return value


def is_link_only(line: str) -> bool:
    return bool(
        re.match(r"^\s*[-*+]\s*\[[^\]]+\]\([^)]*\)\s*[.]?\s*$", line)
    )


def is_table_identifier_row(line: str) -> bool:
    if "|" not in line:
        return False
    cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
    if not cells or not any(cells):
        return False
    for cell in cells:
        if not cell:
            continue
        if re.fullmatch(r"[-:]+", cell):
            continue
        if re.search(r"[A-Za-z]{3,}", cell) and not re.fullmatch(
            r"[`A-Za-z0-9_./:#§+–—-]+", cell
        ):
            return False
    return True


def is_codeish_comment(comment: str) -> bool:
    value = comment.strip().strip("`")
    if not value:
        return True
    if "???" in value and not re.search(
        r"\b(?:fix|add|create|return|complete|replace|translate|make|define)\b",
        value,
        re.IGNORECASE,
    ):
        return True
    return bool(
        re.match(
            r"^(?:let|use|fn|pub|struct|enum|impl|match|if|else|for|while|loop)\b.*(?:;|=|\{)\s*$",
            value,
        )
    )


def is_comment_candidate(comment: str) -> bool:
    if comment.strip().upper().rstrip(":") == "TODO":
        return False
    if is_codeish_comment(comment):
        return False
    if not has_english_words(comment):
        return False
    words = ENGLISH_WORD_RE.findall(comment)
    return len(words) >= 2 or bool(
        re.search(r"\b(?:TODO|NOTE|WARNING|FIXME|Don't|You|Use|Fix|Return)\b", comment)
    )


def normal_literal_value(literal: str) -> str:
    if literal.startswith(("r", "br")):
        first_quote = literal.find('"')
        prefix_end = first_quote
        hashes = literal[2:prefix_end] if literal.startswith("br") else literal[1:prefix_end]
        return literal[first_quote + 1 : -(len(hashes) + 1)]
    value = literal[1:-1]
    value = value.replace(r"\n", "\n")
    value = value.replace(r"\r", "\r")
    value = value.replace(r"\t", "\t")
    value = value.replace(r'\"', '"')
    return value


def iter_rust_literals(text: str) -> Iterable[tuple[int, str, str, str]]:
    """Yield (line, value, surrounding source, source line) for literals."""

    matches: list[tuple[int, int, str]] = []
    raw_matches = list(RAW_LITERAL_RE.finditer(text))
    for match in raw_matches:
        matches.append((match.start(), match.end(), match.group(0)))

    # A malformed-looking normal literal can begin inside a raw string and
    # swallow the following source before the regex has a chance to find the
    # real literal.  Mask raw ranges before scanning normal literals while
    # preserving all offsets.
    masked = list(text)
    for match in raw_matches:
        masked[match.start() : match.end()] = " " * (match.end() - match.start())
    for match in NORMAL_LITERAL_RE.finditer("".join(masked)):
        matches.append((match.start(), match.end(), text[match.start() : match.end()]))
    for start, end, literal in sorted(matches):
        line = text.count("\n", 0, start) + 1
        context = text[max(0, start - 240) : min(len(text), end + 240)]
        line_start = text.rfind("\n", 0, start) + 1
        line_end = text.find("\n", start)
        if line_end < 0:
            line_end = len(text)
        source_line = text[line_start:line_end]
        yield line, normal_literal_value(literal), context, source_line


def event_status(
    value: str,
    preserve: str | None = None,
) -> str:
    if preserve:
        return "preserved"
    if not has_english_words(value):
        return "translated" if contains_chinese(value) else "ignored"
    return "translated" if contains_chinese(value) else "english"


def make_unit(
    path: Path,
    line: int,
    value: str,
    kind: str,
    preserve: str | None = None,
) -> dict[str, Any] | None:
    value = value.strip()
    if not value or (not has_english_words(value) and not contains_chinese(value)):
        return None
    if kind == "rust-string" and not has_english_words(
        re.sub(r"\{[^}]*\}", "", value)
    ) and not contains_chinese(value):
        return None
    status = event_status(value, preserve)
    if status == "ignored":
        return None
    return {
        "path": path.as_posix(),
        "line": line,
        "kind": kind,
        "text": value,
        "status": status,
        "reason": preserve,
    }


def extract_markdown_units(path: Path, text: str) -> list[dict[str, Any]]:
    units: list[dict[str, Any]] = []
    fenced = False
    pending: str | None = None
    for line_number, line in enumerate(text.splitlines(), 1):
        if line.strip().startswith("```"):
            fenced = not fenced
            continue
        reason = marker_reason(line)
        if reason:
            pending = reason
            continue
        if fenced or "<!--" in line:
            continue
        visible = strip_inline_markup(line)
        visible = re.sub(r"^\s{0,3}#{1,6}\s*", "", visible)
        visible = re.sub(r"^\s*[-*+]\s+", "", visible)
        visible = visible.strip(" |:.-")
        if not visible or not has_english_words(visible):
            continue
        auto_reason = None
        if is_link_only(line):
            auto_reason = "official/external resource title"
        elif is_table_identifier_row(line):
            auto_reason = "exercise identifier mapping"
        unit = make_unit(path, line_number, visible, "markdown", pending or auto_reason)
        if unit:
            units.append(unit)
            pending = None
    return units


def source_comments_enabled(path: Path) -> bool:
    return path.as_posix() in {"src/cli.rs", "src/dev.rs"}


def line_comment(line: str) -> str | None:
    index = line.find("//")
    if index < 0:
        return None
    comment = line[index + 2 :]
    if comment.startswith("/"):
        comment = comment[1:]
    return comment.strip()


def source_literal_is_user_facing(
    path: Path, value: str, context: str, source_line: str
) -> tuple[bool, str | None]:
    if TEST_CONTEXT.search(source_line):
        return True, "test contract or assertion string"
    if "\\x1b" in value or "\\x1B" in value:
        return False, "terminal control sequence"
    stripped_value = value.strip()
    if PATH_OR_TOKEN_RE.fullmatch(stripped_value) and (
        "/" in stripped_value
        or "\\" in stripped_value
        or stripped_value.startswith(".")
        or stripped_value
        in {
            "exercises",
            "solutions",
            "Cargo.toml",
            "README.md",
            "rust-analyzer.toml",
        }
    ):
        return False, "path or generated file name"
    if path.as_posix() == "src/dev/new.rs" and re.search(
        r"\b(?:GITIGNORE|INFO_FILE_|CARGO_TOML)\b", context
    ):
        return False, "generated community-project template"
    if MESSAGE_OUTPUT_CONTEXT.search(context):
        return True, None
    if STRUCTURAL_CONTEXT.search(source_line) and not SOURCE_USER_CONTEXT.search(source_line):
        return False, None
    if path.as_posix() == "src/dev/new.rs":
        if "\n" in value or len(value) > 80:
            return True, None
        return bool(SOURCE_USER_CONTEXT.search(source_line)), None
    if SOURCE_USER_CONTEXT.search(source_line):
        return True, None
    return False, None


def exercise_literal_is_user_facing(
    value: str, context: str, source_line: str
) -> tuple[bool, str | None]:
    if TEST_CONTEXT.search(source_line):
        return True, "test contract or assertion string"
    if EXERCISE_USER_MACROS.search(source_line):
        return True, None
    return False, None


def split_literal_lines(value: str, first_line: int) -> Iterable[tuple[int, str]]:
    for offset, part in enumerate(value.splitlines() or [value]):
        yield first_line + offset, part


def extract_rust_units(path: Path, text: str) -> list[dict[str, Any]]:
    is_exercise = path.as_posix().startswith("exercises/")
    is_source = path.as_posix().startswith("src/")
    units: list[dict[str, Any]] = []
    marker_by_line: dict[int, str] = {}
    for line_number, line in enumerate(text.splitlines(), 1):
        reason = marker_reason(line)
        if reason:
            marker_by_line[line_number] = reason

    for line_number, line in enumerate(text.splitlines(), 1):
        comment = line_comment(line)
        if comment and not marker_reason(line):
            should_scan = is_exercise or (is_source and source_comments_enabled(path))
            if should_scan and is_comment_candidate(comment):
                preserve = None
                for marker_line in range(line_number - 1, 0, -1):
                    if marker_line in marker_by_line:
                        preserve = marker_by_line[marker_line]
                        break
                    if text.splitlines()[marker_line - 1].strip():
                        break
                unit = make_unit(path, line_number, comment, "rust-comment", preserve)
                if unit:
                    units.append(unit)

    for line_number, value, context, source_line in iter_rust_literals(text):
        if is_exercise:
            user_facing, auto_reason = exercise_literal_is_user_facing(
                value, context, source_line
            )
        elif is_source:
            user_facing, auto_reason = source_literal_is_user_facing(
                path, value, context, source_line
            )
        else:
            user_facing, auto_reason = False, None
        if not user_facing:
            continue
        preserve = auto_reason
        prior_markers = [line for line in marker_by_line if line < line_number]
        if prior_markers:
            nearest_marker = max(prior_markers)
            lines_between = text.splitlines()[nearest_marker : line_number - 1]
            if all(not line.strip() or marker_reason(line) for line in lines_between):
                preserve = marker_by_line[nearest_marker]
        for value_line, part in split_literal_lines(value, line_number):
            if not part.strip() or (
                not has_english_words(part) and not contains_chinese(part)
            ):
                continue
            if is_codeish_comment(part):
                continue
            unit = make_unit(path, value_line, part, "rust-string", preserve)
            if unit:
                units.append(unit)
    return sorted(units, key=lambda unit: (unit["line"], unit["kind"]))


def extract_units(path: Path, text: str) -> list[dict[str, Any]]:
    if path.suffix.lower() == ".md":
        return extract_markdown_units(path, text)
    if path.suffix.lower() == ".rs":
        return extract_rust_units(path, text)
    return []


def managed_files(root: Path) -> list[Path]:
    paths = list((root / "exercises").rglob("*.rs"))
    paths.extend((root / "exercises").rglob("*.md"))
    paths.extend((root / "src").rglob("*.rs"))
    return sorted(path for path in paths if path.is_file())


def git_output(root: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args], cwd=root, text=True, capture_output=True, check=False
    )
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or f"git {' '.join(args)} failed")
    return result.stdout.strip()


def git_tree_files(root: Path, ref: str) -> set[str]:
    try:
        output = git_output(root, "ls-tree", "-r", "--name-only", ref, "--", "exercises", "src")
    except RuntimeError:
        return set()
    return {line.replace("\\", "/") for line in output.splitlines() if line}


def git_blob(root: Path, ref: str, path: str) -> str | None:
    try:
        return git_output(root, "rev-parse", f"{ref}:{path}")
    except RuntimeError:
        return None


def build_baseline(root: Path, upstream_ref: str) -> dict[str, Any]:
    commit = git_output(root, "rev-parse", f"{upstream_ref}^{{commit}}")
    files = git_tree_files(root, upstream_ref)
    blobs = {path: git_blob(root, upstream_ref, path) for path in sorted(files)}
    return {
        "schema": 1,
        "upstream_ref": upstream_ref,
        "upstream_commit": commit,
        "managed_roots": ["exercises", "src"],
        "files": {path: blob for path, blob in blobs.items() if blob},
    }


def check_upstream(root: Path, manifest_path: Path, upstream_ref: str | None = None) -> dict[str, Any]:
    if not manifest_path.exists():
        return {
            "risk": True,
            "unavailable": f"missing manifest: {manifest_path.as_posix()}",
            "changed": [],
            "added": [],
            "removed": [],
            "missing_local": [],
            "untracked_local": [],
        }
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    ref = upstream_ref or manifest.get("upstream_ref") or manifest.get("upstream_commit")
    baseline_files = set(manifest.get("files", {}))
    local_files = {
        path.relative_to(root).as_posix()
        for path in managed_files(root)
    }
    if not ref:
        return {
            "risk": True,
            "unavailable": "manifest does not specify upstream_ref or upstream_commit",
            "changed": [],
            "added": [],
            "removed": [],
            "missing_local": sorted(baseline_files - local_files),
            "untracked_local": sorted(local_files - baseline_files),
        }
    try:
        git_output(root, "rev-parse", f"{ref}^{{commit}}")
    except RuntimeError as error:
        return {
            "risk": True,
            "unavailable": str(error),
            "ref": ref,
            "changed": [],
            "added": [],
            "removed": [],
            "missing_local": sorted(baseline_files - local_files),
            "untracked_local": sorted(local_files - baseline_files),
        }
    current_files = git_tree_files(root, ref)
    changed = []
    for path in sorted(baseline_files & current_files):
        current_blob = git_blob(root, ref, path)
        if current_blob != manifest["files"].get(path):
            changed.append(path)
    added = sorted(current_files - baseline_files)
    removed = sorted(baseline_files - current_files)
    missing_local = sorted(baseline_files - local_files)
    untracked_local = sorted(local_files - baseline_files)
    return {
        "risk": bool(changed or added or removed or missing_local or untracked_local),
        "ref": ref,
        "changed": changed,
        "added": added,
        "removed": removed,
        "missing_local": missing_local,
        "untracked_local": untracked_local,
    }


def analyze(root: Path) -> dict[str, Any]:
    reports = []
    for path in managed_files(root):
        text = path.read_text(encoding="utf-8")
        units = extract_units(path.relative_to(root), text)
        counts = {status: sum(unit["status"] == status for unit in units) for status in ("translated", "english", "preserved")}
        total = sum(counts.values())
        translated_or_preserved = counts["translated"] + counts["preserved"]
        reports.append(
            {
                "path": path.relative_to(root).as_posix(),
                "units": total,
                **counts,
                "coverage": round((counts["translated"] / (counts["translated"] + counts["english"])) * 100, 2)
                if counts["translated"] + counts["english"]
                else 100.0,
                "review_coverage": round((translated_or_preserved / total) * 100, 2)
                if total
                else 100.0,
                "items": units,
            }
        )
    totals = {
        status: sum(report[status] for report in reports)
        for status in ("translated", "english", "preserved")
    }
    denominator = totals["translated"] + totals["english"]
    total_units = sum(totals.values())
    return {
        "files": reports,
        "summary": {
            "files": len(reports),
            "units": total_units,
            **totals,
            "translation_coverage": round((totals["translated"] / denominator) * 100, 2)
            if denominator
            else 100.0,
            "review_coverage": round(
                ((totals["translated"] + totals["preserved"]) / total_units) * 100, 2
            )
            if total_units
            else 100.0,
        },
    }


def write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def print_human(report: dict[str, Any]) -> None:
    summary = report["summary"]
    print(
        "翻译覆盖检查："
        f"文件 {summary['files']}，文案单元 {summary['units']}，"
        f"中文 {summary['translated']}，英文缺口 {summary['english']}，"
        f"保留 {summary['preserved']}，"
        f"中文覆盖率 {summary['translation_coverage']:.2f}%"
    )
    for file_report in report["files"]:
        if file_report["english"] or file_report["preserved"]:
            print(
                f"- {file_report['path']}: 中文 {file_report['translated']} / "
                f"英文 {file_report['english']} / 保留 {file_report['preserved']}"
            )
            for item in file_report["items"]:
                if item["status"] in {"english", "preserved"}:
                    suffix = f"（{item['reason']}）" if item["reason"] else ""
                    print(f"  {item['line']}: {item['status']} {item['text']}{suffix}")
    upstream = report.get("upstream")
    if upstream:
        if upstream.get("unavailable"):
            print(f"上游同步风险：{upstream['unavailable']}")
        else:
            print(
                "上游同步检查："
                f"变化 {len(upstream['changed'])}，新增 {len(upstream['added'])}，"
                f"删除 {len(upstream['removed'])}，本地缺失 {len(upstream['missing_local'])}，"
                f"本地未登记 {len(upstream['untracked_local'])}"
            )


def main(argv: list[str] | None = None) -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--manifest", type=Path, default=None)
    parser.add_argument("--upstream-ref", default=None)
    parser.add_argument("--write-baseline", action="store_true")
    parser.add_argument("--skip-upstream", action="store_true")
    parser.add_argument("--json", action="store_true", dest="as_json")
    args = parser.parse_args(argv)
    root = args.root.resolve()
    manifest = (args.manifest or root / "scripts" / "translation-baseline.json").resolve()

    if args.write_baseline:
        ref = args.upstream_ref or "upstream/main"
        try:
            baseline = build_baseline(root, ref)
        except RuntimeError as error:
            print(f"无法生成上游基线：{error}", file=sys.stderr)
            return 2
        write_json(manifest, baseline)
        print(f"已写入上游基线：{manifest}")
        return 0

    report = analyze(root)
    if not args.skip_upstream:
        report["upstream"] = check_upstream(root, manifest, args.upstream_ref)
    failed = report["summary"]["english"] > 0
    upstream = report.get("upstream")
    failed = failed or bool(upstream and upstream.get("risk"))
    if args.as_json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
    else:
        print_human(report)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
