#!/usr/bin/env python3
"""Generates chapters/<chapter>/README.md from the headers of examples/ and broken/ files.

The first `//! ...` line of each file is the "what it shows" description.
For broken/ files the `//! Expected error: ...` line is used as well.

    python3 scripts/gen-chapter-readmes.py
    docker run --rm -v "$PWD":/w -w /w python:3-slim python3 scripts/gen-chapter-readmes.py
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def sort_key(p: Path):
    m = re.match(r"(exercise-)?(\d+)", p.stem)
    return (1 if m and m.group(1) else 0, int(m.group(2)) if m else 0, p.stem)


def header(path: Path):
    desc, error = "", ""
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.startswith("//!"):
            break
        text = line[3:].strip()
        if not desc and text:
            desc = text
        if text.startswith("Expected error:"):
            error = text.removeprefix("Expected error:").strip()
    return desc, error


def md(text: str) -> str:
    return text.replace("|", "\\|")


def example_files(chapter: Path):
    """Single-file examples (`NN-name.rs`) and multi-file ones (`NN-name/main.rs`)."""
    files = list((chapter / "examples").glob("*.rs"))
    files += [d / "main.rs" for d in (chapter / "examples").iterdir() if (d / "main.rs").is_file()]
    return sorted(files, key=lambda p: sort_key(p.parent if p.name == "main.rs" else p))


def ex_name(p: Path) -> str:
    return p.parent.name if p.name == "main.rs" else p.stem


def ex_link(p: Path) -> str:
    return f"examples/{p.parent.name}/" if p.name == "main.rs" else f"examples/{p.name}"


def generate(chapter: Path):
    examples = example_files(chapter)
    if not examples:
        return False
    broken = sorted((chapter / "broken").glob("*.rs"), key=sort_key)
    pkg = "ch" + chapter.name
    lib = (chapter / "src" / "lib.rs").read_text(encoding="utf-8").splitlines()[0]
    title = lib.removeprefix("//! Chapter ").strip()

    samples = [p for p in examples if not ex_name(p).startswith("exercise-")]
    exercises = [p for p in examples if ex_name(p).startswith("exercise-")]

    out = [f"# {title}", ""]
    out += [
        "Code for this chapter of the book. Every example is a separate program in `examples/`.",
        "",
        "```bash",
        f"cargo run -p {pkg} --example {ex_name(samples[0])}",
        "# without Rust installed:",
        f'docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p {pkg} --example {ex_name(samples[0])}',
        "```",
        "",
        "Run the commands from the repository root.",
        "",
        "## Examples from the text",
        "",
        "| Example | What it shows | Run |",
        "|---|---|---|",
    ]
    for p in samples:
        desc, _ = header(p)
        out.append(f"| [`{ex_name(p)}`]({ex_link(p)}) | {md(desc)} | `cargo run -p {pkg} --example {ex_name(p)}` |")

    if exercises:
        out += ["", "## Solutions to the “Try it” exercises", "", "| Example | What it shows | Run |", "|---|---|---|"]
        for p in exercises:
            desc, _ = header(p)
            out.append(f"| [`{ex_name(p)}`]({ex_link(p)}) | {md(desc)} | `cargo run -p {pkg} --example {ex_name(p)}` |")

    lib = (chapter / "src" / "lib.rs").read_text(encoding="utf-8")
    bins = sorted((chapter / "src" / "bin").glob("*.rs")) if (chapter / "src" / "bin").is_dir() else []
    has_tests = (chapter / "tests").is_dir() or "#[test]" in lib or "```" in lib
    if (chapter / "src" / "main.rs").exists() or bins or has_tests:
        out += ["", "## The crate itself", "", "```bash"]
        if (chapter / "src" / "main.rs").exists():
            out.append(f"cargo run -p {pkg}                      # src/main.rs")
        for b in bins:
            out.append(f"cargo run -p {pkg} --bin {b.stem}         # src/bin/{b.name}")
        if has_tests:
            out.append(f"cargo test -p {pkg}                     # unit, integration and doc tests")
        out.append("```")

    if broken:
        out += [
            "",
            "## Intentionally broken examples",
            "",
            "`broken/` holds code that the book shows as a compile error. Cargo does not build these files.",
            "The header of each file states the expected error and the fix.",
            "To check that they really fail, and with exactly that error:",
            "",
            "```bash",
            f"./scripts/check-broken.sh {chapter.name[:2]}",
            f"rustc --edition 2024 chapters/{chapter.name}/broken/{broken[0].name}   # see the compiler message",
            "```",
            "",
            "| File | What it shows | Expected error |",
            "|---|---|---|",
        ]
        for p in broken:
            desc, error = header(p)
            out.append(f"| [`{p.name}`](broken/{p.name}) | {md(desc)} | {md(error)} |")

    (chapter / "README.md").write_text("\n".join(out) + "\n", encoding="utf-8")
    return True


if __name__ == "__main__":
    for ch in sorted((ROOT / "chapters").iterdir()):
        if ch.is_dir() and generate(ch):
            print(f"wrote {ch.relative_to(ROOT)}/README.md")
