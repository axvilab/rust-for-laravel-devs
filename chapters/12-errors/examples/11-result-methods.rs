//! Методи Result: unwrap_or, unwrap_or_else, map, map_err, ok, and_then, ok_or

#[derive(Debug)]
enum AppError {
    Validation(String),
    NotFound,
}

fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|_| format!("«{s}» — не порт"))
}

fn bind(port: u16) -> Result<String, String> {
    if port < 1024 {
        Err(format!("порт {port} потребує root"))
    } else {
        Ok(format!("0.0.0.0:{port}"))
    }
}

fn log(e: String) {
    eprintln!("лог: {e}");
}

fn main() {
    let r: Result<u16, String> = parse_port("80");

    println!("{}", r.clone().unwrap()); // значення або паніка
    println!("{}", r.clone().expect("порт має бути валідним")); // те саме з повідомленням
    println!("{}", parse_port("x").unwrap_or(8080)); // значення або дефолт
    println!(
        "{}",
        parse_port("x").unwrap_or_else(|e| {
            log(e);
            8080
        })
    );
    println!("{:?}", r.clone().map(|p| p + 1)); // перетворити Ok, Err залишити
    println!("{:?}", parse_port("x").map_err(AppError::Validation)); // перетворити Err
    println!("{:?}", r.clone().ok()); // Result → Option (помилку відкинути)
    println!("{} {}", r.is_ok(), r.is_err());
    println!("{:?}", r.clone().and_then(bind)); // Err: 80 < 1024
    println!("{:?}", parse_port("8080").and_then(bind)); // Ok("0.0.0.0:8080")

    // Option → Result: findOrFail одним рядком
    let found: Option<u64> = None;
    println!("{:?}", found.ok_or(AppError::NotFound));
}
