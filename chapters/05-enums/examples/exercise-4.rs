//! Exercise 4: print the discount with `match`, then with `.map()` + `.unwrap_or()`

fn find_discount(code: &str) -> Option<f64> {
    match code {
        "WELCOME" => Some(0.1),
        "VIP" => Some(0.25),
        _ => None,
    }
}

fn main() {
    let codes = ["WELCOME", "VIP", "FREE"];

    // with match
    for code in codes {
        match find_discount(code) {
            Some(d) => println!("discount {}%", d * 100.0),
            None => println!("invalid code"),
        }
    }

    // with map + unwrap_or: Option<f64> → Option<String> → String
    for code in codes {
        let text = find_discount(code)
            .map(|d| format!("discount {}%", d * 100.0))
            .unwrap_or("invalid code".to_string());
        println!("{text}");
    }
}
