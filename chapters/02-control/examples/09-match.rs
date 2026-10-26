//! `match` — a `switch` that grew up (and returns a value)

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
