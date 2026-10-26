//! Вправа 5: у що розгортається `#[derive(Debug, Clone)]` (як показав би `cargo expand`)
//!
//! `cargo install cargo-expand` і `cargo expand --example exercise-5` покажуть
//! майже дослівно реалізації нижче. Тут вони написані руками для `Point`,
//! а `DerivedPoint` отримує ті самі через derive — вивід однаковий.

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// ≈ що генерує #[derive(Debug)]
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// ≈ що генерує #[derive(Clone)]: клон кожного поля
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: Clone::clone(&self.x),
            y: Clone::clone(&self.y),
        }
    }
}

#[derive(Debug, Clone)]
struct DerivedPoint {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = DerivedPoint { x: 1, y: 2 };
    println!("{:?}", a.clone()); // Point { x: 1, y: 2 }
    println!("{:?}", b.clone()); // DerivedPoint { x: 1, y: 2 }
}
