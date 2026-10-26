//! Result methods: unwrap_or, unwrap_or_else, map, map_err, ok, and_then, ok_or

#[derive(Debug)]
enum AppError {
    Validation(String),
    NotFound,
}

fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|_| format!("'{s}' is not a port"))
}

fn bind(port: u16) -> Result<String, String> {
    if port < 1024 {
        Err(format!("port {port} requires root"))
    } else {
        Ok(format!("0.0.0.0:{port}"))
    }
}

fn log(e: String) {
    eprintln!("log: {e}");
}

fn main() {
    let r: Result<u16, String> = parse_port("80");

    println!("{}", r.clone().unwrap()); // the value or a panic
    println!("{}", r.clone().expect("the port must be valid")); // the same with a message
    println!("{}", parse_port("x").unwrap_or(8080)); // the value or a default
    println!(
        "{}",
        parse_port("x").unwrap_or_else(|e| {
            log(e);
            8080
        })
    );
    println!("{:?}", r.clone().map(|p| p + 1)); // transform Ok, leave Err
    println!("{:?}", parse_port("x").map_err(AppError::Validation)); // transform Err
    println!("{:?}", r.clone().ok()); // Result → Option (drop the error)
    println!("{} {}", r.is_ok(), r.is_err());
    println!("{:?}", r.clone().and_then(bind)); // Err: 80 < 1024
    println!("{:?}", parse_port("8080").and_then(bind)); // Ok("0.0.0.0:8080")

    // Option → Result: findOrFail in one line
    let found: Option<u64> = None;
    println!("{:?}", found.ok_or(AppError::NotFound));
}
