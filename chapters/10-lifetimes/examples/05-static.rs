//! `'static` — a reference valid until the end of the program (string literals)

fn app_name() -> &'static str {
    "MyApp"
}

fn main() {
    let s: &'static str = "I live forever";
    println!("{s} — {}", app_name());
}
