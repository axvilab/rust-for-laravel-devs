//! Four ways to make a `String` from a `&str`

struct User {
    name: String,
    email: String,
}

fn main() {
    let name = "Vitalii";
    let u = User {
        name: name.into(), // the field type is known → into() knows what to make
        email: String::from("v@example.com"),
    };
    let id = 42.to_string(); // to_string() works for numbers too
    // let s = "text".into();                // ERROR: type annotations needed
    let s: String = "text".into(); // ok: the type is given
    let t = "text".to_owned(); // also a String
    println!("{} {} {id} {s} {t}", u.name, u.email);
}
