//! Деструктуризація кортежу прямо в гілці `match`

fn main() {
    for point in [(0, 0), (3, 0), (0, -2), (3, 4)] {
        match point {
            (0, 0) => println!("початок координат"),
            (x, 0) => println!("на осі X, x={x}"),
            (0, y) => println!("на осі Y, y={y}"),
            (x, y) => println!("({x}, {y})"),
        }
    }
}
