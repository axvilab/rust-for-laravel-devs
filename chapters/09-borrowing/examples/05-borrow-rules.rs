//! Either many `&` or one `&mut`; a borrow ends at its last use

fn main() {
    let mut list = vec![1, 2, 3];

    let a = &list;
    let b = &list; // ok: two readers
    println!("{a:?} {b:?}");

    let c = &mut list; // ok: readers a and b aren't used any more
    c.push(4);
    c.push(5); // the last use of c

    let d = &list; // ok: editor c has already "given back" access
    println!("{d:?}"); // [1, 2, 3, 4, 5]
}
