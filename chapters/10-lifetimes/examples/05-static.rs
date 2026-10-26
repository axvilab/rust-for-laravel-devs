//! `'static` — посилання, валідне до кінця програми (рядкові літерали)

fn app_name() -> &'static str {
    "MyApp"
}

fn main() {
    let s: &'static str = "я живу вічно";
    println!("{s} — {}", app_name());
}
