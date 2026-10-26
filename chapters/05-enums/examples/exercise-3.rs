//! Вправа 3: `find_discount` повертає `Option<f64>`

fn find_discount(code: &str) -> Option<f64> {
    match code {
        "WELCOME" => Some(0.1),
        "VIP" => Some(0.25),
        _ => None,
    }
}

fn main() {
    println!("{:?}", find_discount("WELCOME")); // Some(0.1)
    println!("{:?}", find_discount("VIP")); // Some(0.25)
    println!("{:?}", find_discount("FREE")); // None
}
