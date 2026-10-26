# Build from the REPOSITORY ROOT (the Cargo workspace and Cargo.lock live there):
#   docker build -f orders-api/Dockerfile -t orders-api .
# or simply: docker compose up --build   (from orders-api/)

# ---- chef: rust toolchain + cargo-chef --------------------------------------
FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef --version 0.1.78 --locked
WORKDIR /app

# ---- planner: a "recipe" = only the dependency graph, no source code ---------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- builder ----------------------------------------------------------------
FROM chef AS builder
# 1) dependencies only. This layer is cached until Cargo.toml/Cargo.lock change,
#    so editing src/ does not recompile the ~300 dependency crates.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# 2) our code. Migrations are embedded by sqlx::migrate!, queries are
#    runtime-checked (query_as, not query_as!) so no DATABASE_URL / .sqlx needed.
COPY . .
RUN cargo build --release -p orders-api

# ---- runtime: no shell, no package manager, just glibc + our binary ---------
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/orders-api /usr/local/bin/orders-api
ENV HOST=0.0.0.0 PORT=3000
EXPOSE 3000
USER nonroot
ENTRYPOINT ["/usr/local/bin/orders-api"]
