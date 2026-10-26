//! anyhow + `.context()`: a human-readable chain of causes in `main`
// exits-with-error: without a config.toml in the current directory the program shows
// `Error: failed to read config.toml` + `Caused by: No such file or directory`.
// Create the file with `port = 8080` and you'll see a successful run.

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
}

fn run(cfg: Config) -> Result<()> {
    println!("starting on port {}", cfg.port);
    Ok(())
}

fn main() -> Result<()> {
    let text = std::fs::read_to_string("config.toml").context("failed to read config.toml")?;
    let cfg: Config = toml::from_str(&text).context("config.toml has an invalid format")?;
    run(cfg)?;
    Ok(())
}
