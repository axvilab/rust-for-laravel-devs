//! Вправа 1: `http_class` через `match` з діапазонами

fn http_class(status: u16) -> &'static str {
    match status {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "invalid", // усе інше: 0..=99 і 600..
    }
}

fn main() {
    for s in [101, 200, 302, 404, 503, 42, 700] {
        println!("{s} → {}", http_class(s));
    }
}
