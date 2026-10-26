//! Вправа 4: вивід знижки через `match`, потім через `.map()` + `.unwrap_or()`

fn find_discount(code: &str) -> Option<f64> {
    match code {
        "WELCOME" => Some(0.1),
        "VIP" => Some(0.25),
        _ => None,
    }
}

fn main() {
    let codes = ["WELCOME", "VIP", "FREE"];

    // через match
    for code in codes {
        match find_discount(code) {
            Some(d) => println!("знижка {}%", d * 100.0),
            None => println!("код не дійсний"),
        }
    }

    // через map + unwrap_or: Option<f64> → Option<String> → String
    for code in codes {
        let text = find_discount(code)
            .map(|d| format!("знижка {}%", d * 100.0))
            .unwrap_or("код не дійсний".to_string());
        println!("{text}");
    }
}
