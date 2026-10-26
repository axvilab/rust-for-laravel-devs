//! `Result<T, E>`: функція, яка може не впоратися, і `match` по результату

fn parse_port(s: &str) -> Result<u16, String> {
    match s.parse::<u16>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("порт не може бути 0".to_string()),
        Err(_) => Err(format!("«{s}» — не число")),
    }
}

fn main() {
    for input in ["8080", "0", "abc"] {
        match parse_port(input) {
            Ok(port) => println!("слухаємо {port}"),
            Err(msg) => eprintln!("помилка: {msg}"),
        }
    }
}
