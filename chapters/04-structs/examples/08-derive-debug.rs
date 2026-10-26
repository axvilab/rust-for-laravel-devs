//! `#[derive(Debug)]` — компілятор пише реалізацію `Debug` за тебе

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Віталій".into(),
    };
    println!("{:?}", user); // User { id: 1, name: "Віталій" }
    println!("{:#?}", user); // те саме з переносами — як dd()
}
