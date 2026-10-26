//! Чотири способи зробити `String` з `&str`

struct User {
    name: String,
    email: String,
}

fn main() {
    let name = "Віталій";
    let u = User {
        name: name.into(), // тип поля відомий → into() знає, у що
        email: String::from("v@example.com"),
    };
    let id = 42.to_string(); // to_string() вміє і числа
    // let s = "текст".into();                // ПОМИЛКА: type annotations needed
    let s: String = "текст".into(); // ok: тип вказали
    let t = "текст".to_owned(); // теж String
    println!("{} {} {id} {s} {t}", u.name, u.email);
}
