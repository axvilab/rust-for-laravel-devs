//! Exercise 4: two working options — copy the value or print before `push`

fn main() {
    // Option 1: i32 is Copy, so we take the value, not a reference
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    println!("{first} {v:?}");

    // Option 2: a reference, but its last use is before push (NLL)
    let mut w = vec![1, 2, 3];
    let first = &w[0];
    println!("{first}");
    w.push(4);
    println!("{w:?}");
}
