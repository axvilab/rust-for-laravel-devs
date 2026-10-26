//! Вправа 6: `push(v.len())` працює, `push(v.pop())` — через тимчасову змінну

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.len()); // читання в аргументі — ok (two-phase borrow)
    println!("{v:?}"); // [1, 2, 3, 3]

    let last = v.pop().unwrap(); // перший &mut закінчився тут
    v.push(last); // другий &mut — уже без конфлікту
    println!("{v:?}"); // [1, 2, 3, 3]
}
