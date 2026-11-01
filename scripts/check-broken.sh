#!/usr/bin/env bash
# Checks that every chapters/*/broken/*.rs really does NOT compile,
# and that the error is exactly the one promised by its `// expected-error: ...` line.
#
#   ./scripts/check-broken.sh            # all chapters
#   ./scripts/check-broken.sh 08         # only chapters/08-*
#
# Files with a `// check-with: cargo` line need the chapter crate's dependencies (axum, tower...):
# they are copied into that crate's examples/ for a moment and built with `cargo build --example`.
#
# Docker:  docker run --rm -v "$PWD":/w -w /w rust:1-bookworm ./scripts/check-broken.sh
set -uo pipefail

cd "$(dirname "$0")/.."
filter="${1:-}"
tmp="$(mktemp -d)"
probe=""   # a temporary examples/ file for `check-with: cargo`; removed on exit
trap 'rm -rf "$tmp"; [ -n "$probe" ] && rm -f "$probe"' EXIT

fail=0
count=0
for f in chapters/${filter}*/broken/*.rs; do
    [ -e "$f" ] || continue
    count=$((count + 1))
    expected="$(sed -n 's|^// expected-error: ||p' "$f" | head -1)"
    if [ -z "$expected" ]; then
        echo "FAIL  $f: no '// expected-error:' line"
        fail=1
        continue
    fi
    if grep -q '^// check-with: cargo' "$f"; then
        crate_dir="$(dirname "$(dirname "$f")")"
        pkg="$(sed -n 's|^name = "\(.*\)"|\1|p' "$crate_dir/Cargo.toml" | head -1)"
        probe="$crate_dir/examples/zz-broken-probe.rs"
        cp "$f" "$probe"
        out="$(cargo build --quiet -p "$pkg" --example zz-broken-probe 2>&1)"
        status=$?
        rm -f "$probe"
        probe=""
    else
        # optional extra flags, e.g. `// rustc-flags: --test` for errors only seen in test builds
        read -r -a flags <<<"$(sed -n 's|^// rustc-flags: ||p' "$f" | head -1)"
        out="$(rustc --edition 2024 --crate-type bin "${flags[@]}" -o "$tmp/out" "$f" 2>&1)"
        status=$?
    fi
    if [ "$status" -eq 0 ]; then
        echo "FAIL  $f: compiled, but must not"
        fail=1
    elif grep -qF -- "$expected" <<<"$out"; then
        echo "ok    $f  ($expected)"
    else
        echo "FAIL  $f: expected '$expected', got:"
        grep -E '^(error|warning)' <<<"$out" | sed 's/^/        /'
        fail=1
    fi
done

echo "checked $count broken examples"
exit $fail
