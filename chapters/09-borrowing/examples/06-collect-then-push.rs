//! First collect what to add, then add it

fn main() {
    let mut list = vec![1, 2, 3];

    let mut to_add = Vec::new();
    for item in &list {
        if *item == 2 {
            to_add.push(99);
        }
    }
    list.extend(to_add); // the loop's borrow is already over

    println!("{list:?}"); // [1, 2, 3, 99]
}
