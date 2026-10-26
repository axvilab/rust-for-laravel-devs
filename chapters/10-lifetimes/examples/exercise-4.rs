//! Exercise 4: `Excerpt` with a `String` — the lifetime disappears

struct Excerpt {
    part: String, // the struct owns its data
}

fn main() {
    let e;
    {
        let text = String::from("First sentence. Second.");
        // to_string() copies the piece — Excerpt no longer depends on text
        e = Excerpt {
            part: text.split('.').next().unwrap_or("").to_string(),
        };
    } // text is destroyed, e isn't
    println!("{}", e.part);
}
