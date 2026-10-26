//! A `&[T]` slice — a view of a Vec, a part of it or an array

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let middle: &[i32] = &v[1..4]; // [20, 30, 40]
    let all: &[i32] = &v; // the whole vector as a slice
    println!("{middle:?} {}", all.len());

    println!("{}", sum(&v)); // accepts a Vec
    println!("{}", sum(&v[..2])); // a piece
    println!("{}", sum(&[1, 2, 3])); // and an array
}
