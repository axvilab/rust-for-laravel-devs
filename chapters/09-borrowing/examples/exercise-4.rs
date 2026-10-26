//! Вправа 4: два робочі варіанти — скопіювати значення або вивести до `push`

fn main() {
    // Варіант 1: i32 — Copy, тому беремо значення, а не посилання
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    println!("{first} {v:?}");

    // Варіант 2: посилання, але останнє використання — до push (NLL)
    let mut w = vec![1, 2, 3];
    let first = &w[0];
    println!("{first}");
    w.push(4);
    println!("{w:?}");
}
