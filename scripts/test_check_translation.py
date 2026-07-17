import json
import sys
import subprocess
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from scripts import check_translation


class TranslationExtractionTests(unittest.TestCase):
    def test_markdown_code_blocks_are_not_translation_units(self):
        text = """# 中文主题

This paragraph must be translated.

```rust
fn main() { println!(\"English code output\"); }
```

<!-- translation: preserve official resource title -->
- [The Rust Book](https://doc.rust-lang.org/book/)
"""

        units = check_translation.extract_units(Path("exercises/demo/README.md"), text)

        self.assertEqual([unit["status"] for unit in units], ["english", "preserved"])
        self.assertEqual(units[1]["reason"], "official resource title")

    def test_rust_comments_and_user_output_are_classified(self):
        text = """// TODO: Translate this learner hint.
// translation: preserve test contract: exact assertion output
assert_eq!(value, \"Exact English\");
println!(\"User-facing English\");
"""

        units = check_translation.extract_units(Path("exercises/demo/demo.rs"), text)

        self.assertEqual([unit["status"] for unit in units], ["english", "preserved", "english"])
        self.assertEqual(units[1]["reason"], "test contract: exact assertion output")

    def test_chinese_text_is_translated(self):
        text = "// TODO: 请修复这个练习。\nprintln!(\"你好，学习者！\");\n"

        units = check_translation.extract_units(Path("exercises/demo/demo.rs"), text)

        self.assertEqual(len(units), 2)
        self.assertTrue(all(unit["status"] == "translated" for unit in units))

    def test_placeholder_output_and_standalone_todo_are_ignored(self):
        text = """// TODO
println!(\"{value:?}\");
println!(\"输出={value:?}\");
"""

        units = check_translation.extract_units(Path("exercises/demo/demo.rs"), text)

        self.assertEqual([unit["text"] for unit in units], ["输出={value:?}"])

    def test_raw_byte_strings_do_not_swallow_following_source(self):
        text = 'let path = br"\\\\?\\";\nprintln!("用户提示");\n'

        units = check_translation.extract_units(Path("src/term.rs"), text)

        self.assertEqual([unit["text"] for unit in units], ["用户提示"])

    def test_list_messages_are_user_facing_but_paths_and_ansi_are_preserved(self):
        text = (
            'self.message.push_str("Nothing selected");\n'
            'fs::write("rust-analyzer.toml", contents);\n'
            'writer.write_all(b"\\x1b]8;;file://");\n'
        )

        units = check_translation.extract_units(Path("src/list/state.rs"), text)

        self.assertEqual([unit["status"] for unit in units], ["english"])
        self.assertEqual(units[0]["text"], "Nothing selected")


class UpstreamRiskTests(unittest.TestCase):
    def test_upstream_blob_change_is_reported(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "exercises/demo").mkdir(parents=True)
            (root / "src").mkdir()
            (root / "exercises/demo/README.md").write_text("# 中文\n", encoding="utf-8")
            (root / "exercises/demo/demo.rs").write_text("// 中文\n", encoding="utf-8")
            (root / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            (root / "upstream.txt").write_text("source v1\n", encoding="utf-8")
            subprocess.run(["git", "add", "."], cwd=root, check=True)
            subprocess.run(
                ["git", "-c", "user.name=Test", "-c", "user.email=test@example.com", "commit", "-qm", "baseline"],
                cwd=root,
                check=True,
            )
            commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()

            manifest = {
                "schema": 1,
                "upstream_commit": commit,
                "files": {
                    "exercises/demo/README.md": "0" * 40,
                    "exercises/demo/demo.rs": "0" * 40,
                    "src/main.rs": "0" * 40,
                },
            }
            (root / "manifest.json").write_text(json.dumps(manifest), encoding="utf-8")
            report = check_translation.check_upstream(root, root / "manifest.json", commit)

            self.assertTrue(report["risk"])
            self.assertIn("exercises/demo/README.md", report["changed"])


if __name__ == "__main__":
    unittest.main()
