//! `#[derive(Debug)]` — the compiler writes the `Debug` implementation for you

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Vitalii".into(),
    };
    println!("{:?}", user); // User { id: 1, name: "Vitalii" }
    println!("{:#?}", user); // the same with line breaks — like dd()
}
