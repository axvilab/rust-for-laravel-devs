//! Exercise 5: how many doublings until 1 exceeds 1000 (`break value`)

fn main() {
    let mut value = 1;
    let mut doublings = 0;
    let count = loop {
        if value > 1000 {
            break doublings; // the loop returns the count
        }
        value *= 2;
        doublings += 1;
    };
    println!("{count} doublings, value {value}"); // 10 doublings, 1024
}
