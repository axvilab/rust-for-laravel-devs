//! Оператор `?`: якщо `Err` — повернути його з функції, якщо `Ok` — йти далі

use std::io;

#[derive(Debug)]
struct Config {
    port: u16,
}

// Мініпарсер рядка `port = 8080`. Повертає io::Error, щоб у load_config
// обидва `?` мали один тип помилки (як у книзі).
fn parse(text: &str) -> Result<Config, io::Error> {
    let value = text
        .trim()
        .strip_prefix("port")
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "очікую `port = N`"))?;
    let port = value
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Config { port })
}

fn load_config(path: &str) -> Result<Config, io::Error> {
    let text = std::fs::read_to_string(path)?; // якщо Err — повернути його з функції
    let cfg = parse(&text)?; // те саме
    Ok(cfg)
}

fn main() {
    let path = std::env::temp_dir().join("ch12-config.txt");
    std::fs::write(&path, "port = 8080").unwrap();
    println!("{:?}", load_config(path.to_str().unwrap())); // Ok(Config { port: 8080 })
    println!("{:?}", load_config("/немає/такого/файлу")); // Err(Os { code: 2, kind: NotFound, .. })
}
