//! Вправа 1: `pick` з двома літералами, потім з літералом і `&String`

fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str {
    if cond { a } else { b }
}

fn main() {
    println!("{}", pick(true, "так", "ні")); // два &'static str

    let owned = String::from("з String");
    // 'a стає коротшим із двох: лайфтаймом `owned`. Літерал живе довше — ок.
    println!("{}", pick(false, "літерал", &owned));
}
