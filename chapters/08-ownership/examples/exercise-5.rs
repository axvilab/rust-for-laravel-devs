//! Вправа 5: зі `String` усередині — лише `Clone`, копія явна
//!
//! `#[derive(Copy)]` тут дає E0204 (див. broken/exercise-5.rs).

#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let a = User {
        id: 1,
        name: "Віталій".into(),
    };
    let b = a.clone(); // явна копія замість неявної
    println!("{a:?} {b:?}");
}
