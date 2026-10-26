//! Instead of a closure you can pass the name of a function or method

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    // Calling `.map` right on `Some(...)` makes clippy say
    // unnecessary_map_on_constructor ("just write Some(double(21))").
    // In real code the Option comes from outside — hence the variables.
    let num = Some(-5_i32);
    let answer = Some(21);
    let text = Some("text");

    let a = num.map(|n| n.abs()); // a closure
    let b = num.map(i32::abs); // the same: a method name
    let c = answer.map(double); // your own function
    let d = text.map(String::from);
    println!("{a:?} {b:?} {c:?} {d:?}"); // Some(5) Some(5) Some(42) Some("text")
}
