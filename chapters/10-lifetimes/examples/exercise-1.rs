//! Exercise 1: `pick` with two literals, then with a literal and a `&String`

fn pick<'a>(cond: bool, a: &'a str, b: &'a str) -> &'a str {
    if cond { a } else { b }
}

fn main() {
    println!("{}", pick(true, "yes", "no")); // two &'static str

    let owned = String::from("from a String");
    // 'a becomes the shorter of the two: the lifetime of `owned`. The literal lives longer — ok.
    println!("{}", pick(false, "a literal", &owned));
}
