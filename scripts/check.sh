#!/usr/bin/env bash
# Runs the same checks as CI, locally in Docker (no Rust needed on the host):
# fmt, clippy -D warnings, tests with Postgres, broken/, release build,
# cargo audit, docker build of orders-api.
#
#   ./scripts/check.sh
#
# The cargo registry is cached in the Docker volume `rfld-cargo`;
# remove it with `docker volume rm rfld-cargo`.
set -uo pipefail

cd "$(dirname "$0")/.."
repo="$PWD"
net=rfld-check
db=rfld-check-pg
image=rfld-check-rust

cleanup() {
    docker rm -f "$db" >/dev/null 2>&1
    docker network rm "$net" >/dev/null 2>&1
}
trap cleanup EXIT

echo "==> preparing Docker image and Postgres"
printf 'FROM rust:1-bookworm\nRUN rustup component add rustfmt clippy\n' \
    | docker build -q -t "$image" - >/dev/null || exit 1
docker network create "$net" >/dev/null 2>&1
docker run -d --name "$db" --network "$net" -e POSTGRES_PASSWORD=pg postgres:16 >/dev/null || exit 1
until docker exec "$db" pg_isready -U postgres >/dev/null 2>&1; do sleep 1; done

in_rust() {
    docker run --rm --network "$net" -u "$(id -u):$(id -g)" \
        -e CARGO_HOME=/cargo -e RUSTFLAGS="-D warnings" \
        -e DATABASE_URL="postgres://postgres:pg@$db:5432/postgres" \
        -v rfld-cargo:/cargo -v "$repo":/w -w /w "$image" bash -c "$1"
}

# The volume starts empty: copy the toolchain's cargo home into it once.
docker run --rm -v rfld-cargo:/cargo "$image" \
    bash -c "cp -rn /usr/local/cargo/. /cargo/ && chown -R $(id -u):$(id -g) /cargo"

failed=()
run() {
    local name="$1"; shift
    echo; echo "==> $name"
    if "$@"; then echo "OK    $name"; else echo "FAIL  $name"; failed+=("$name"); fi
}

run "fmt"           in_rust "cargo fmt --all --check"
run "clippy"        in_rust "cargo clippy --workspace --all-targets -- -D warnings"
run "test"          in_rust "cargo test --workspace"
run "broken"        in_rust "./scripts/check-broken.sh"
run "release build" in_rust "cargo build --workspace --release"
run "audit"         in_rust "cargo install cargo-audit --locked -q 2>/dev/null; cargo audit"
run "docker build"  docker build -q -f orders-api/Dockerfile -t orders-api:check .
docker rmi orders-api:check >/dev/null 2>&1

echo
if [ ${#failed[@]} -eq 0 ]; then
    echo "All checks passed."
else
    echo "Failed: ${failed[*]}"
    exit 1
fi
