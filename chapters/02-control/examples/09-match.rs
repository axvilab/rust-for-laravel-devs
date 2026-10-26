//! `match` — `switch`, який подорослішав (і повертає значення)

fn main() {
    let status = 404;
    let label = match status {
        200 | 201 => "ok",
        404 => "not found",
        500 => "server error",
        _ => "unknown",
    };
    println!("{label}");
}
