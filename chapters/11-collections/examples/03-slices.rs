//! Зріз `&[T]` — погляд на Vec, його частину або масив

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let middle: &[i32] = &v[1..4]; // [20, 30, 40]
    let all: &[i32] = &v; // весь вектор як зріз
    println!("{middle:?} {}", all.len());

    println!("{}", sum(&v)); // приймає і Vec
    println!("{}", sum(&v[..2])); // і шматок
    println!("{}", sum(&[1, 2, 3])); // і масив
}
