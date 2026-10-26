//! Destructuring a tuple right in a `match` arm

fn main() {
    for point in [(0, 0), (3, 0), (0, -2), (3, 4)] {
        match point {
            (0, 0) => println!("origin"),
            (x, 0) => println!("on the X axis, x={x}"),
            (0, y) => println!("on the Y axis, y={y}"),
            (x, y) => println!("({x}, {y})"),
        }
    }
}
