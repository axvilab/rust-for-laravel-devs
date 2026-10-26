//! Type inference and explicit annotations

fn main() {
    let a = 5; // i32 (signed integer, 32 bits)
    let b = 5.0; // f64 (floating point, 64 bits)
    let c = true; // bool
    let d = 'x'; // char — one Unicode character, in single quotes
    let e = "text"; // &str — a string

    let port: u16 = 8080; // type given explicitly

    println!("{a} {b} {c} {d} {e} {port}");
}
