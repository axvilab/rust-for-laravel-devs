//! anyhow + `.context()`: людський ланцюжок причин у `main`
// exits-with-error: без файлу config.toml у поточній директорії програма покаже
// `Error: не вдалось прочитати config.toml` + `Caused by: No such file or directory`.
// Створи файл з `port = 8080` — і побачиш успішний запуск.

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
}

fn run(cfg: Config) -> Result<()> {
    println!("стартуємо на порту {}", cfg.port);
    Ok(())
}

fn main() -> Result<()> {
    let text =
        std::fs::read_to_string("config.toml").context("не вдалось прочитати config.toml")?;
    let cfg: Config = toml::from_str(&text).context("config.toml має неправильний формат")?;
    run(cfg)?;
    Ok(())
}
