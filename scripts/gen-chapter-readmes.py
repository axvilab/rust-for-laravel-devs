#!/usr/bin/env python3
"""Генерує chapters/<розділ>/README.md з шапок файлів examples/ і broken/.

Перший рядок `//! ...` кожного файлу — опис «що показує».
У broken/ ще береться рядок `//! Очікувана помилка: ...`.

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
        if text.startswith("Очікувана помилка:"):
            error = text.removeprefix("Очікувана помилка:").strip()
    return desc, error


def md(text: str) -> str:
    return text.replace("|", "\\|")


def generate(chapter: Path):
    examples = sorted((chapter / "examples").glob("*.rs"), key=sort_key)
    if not examples:
        return False
    broken = sorted((chapter / "broken").glob("*.rs"), key=sort_key)
    pkg = "ch" + chapter.name
    lib = (chapter / "src" / "lib.rs").read_text(encoding="utf-8").splitlines()[0]
    title = lib.removeprefix("//! Розділ ").strip()

    samples = [p for p in examples if not p.stem.startswith("exercise-")]
    exercises = [p for p in examples if p.stem.startswith("exercise-")]

    out = [f"# {title}", ""]
    out += [
        "Код до розділу книги. Кожен приклад — окрема програма в `examples/`.",
        "",
        "```bash",
        f"cargo run -p {pkg} --example {samples[0].stem}",
        "# без Rust на машині:",
        f'docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p {pkg} --example {samples[0].stem}',
        "```",
        "",
        "Команди запускаються з кореня репозиторію.",
        "",
        "## Приклади з тексту",
        "",
        "| Приклад | Що показує | Команда запуску |",
        "|---|---|---|",
    ]
    for p in samples:
        desc, _ = header(p)
        out.append(f"| [`{p.stem}`](examples/{p.name}) | {md(desc)} | `cargo run -p {pkg} --example {p.stem}` |")

    if exercises:
        out += ["", "## Розв'язки вправ «Спробуй»", "", "| Приклад | Що показує | Команда запуску |", "|---|---|---|"]
        for p in exercises:
            desc, _ = header(p)
            out.append(f"| [`{p.stem}`](examples/{p.name}) | {md(desc)} | `cargo run -p {pkg} --example {p.stem}` |")

    if broken:
        out += [
            "",
            "## Навмисно зламані приклади",
            "",
            "У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.",
            "У шапці кожного файлу записано очікувану помилку і виправлення.",
            "Перевірити, що вони справді не компілюються і з тією самою помилкою:",
            "",
            "```bash",
            f"./scripts/check-broken.sh {chapter.name[:2]}",
            f"rustc --edition 2024 chapters/{chapter.name}/broken/{broken[0].name}   # побачити повідомлення компілятора",
            "```",
            "",
            "| Файл | Що показує | Очікувана помилка |",
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
