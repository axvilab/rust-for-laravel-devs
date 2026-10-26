//! Вправа 4: `Excerpt` з `String` — лайфтайм зникає

struct Excerpt {
    part: String, // структура володіє даними
}

fn main() {
    let e;
    {
        let text = String::from("Перше речення. Друге.");
        // to_string() копіює шматок — Excerpt більше не залежить від text
        e = Excerpt {
            part: text.split('.').next().unwrap_or("").to_string(),
        };
    } // text знищено, а e — ні
    println!("{}", e.part);
}
