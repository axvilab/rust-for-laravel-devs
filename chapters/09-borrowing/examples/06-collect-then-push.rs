//! Спочатку зібрати, що додати, потім додати

fn main() {
    let mut list = vec![1, 2, 3];

    let mut to_add = Vec::new();
    for item in &list {
        if *item == 2 {
            to_add.push(99);
        }
    }
    list.extend(to_add); // позичання циклом уже закінчилось

    println!("{list:?}"); // [1, 2, 3, 99]
}
