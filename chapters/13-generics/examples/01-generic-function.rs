//! A generic function `first<T>` — one for any type, without losing the type

fn first<T>(items: &[T]) -> Option<&T> {
    items.first() // the same as items.get(0); clippy prefers first()
}

fn main() {
    let n = first(&[1, 2, 3]); // Option<&i32>
    let s = first(&["a", "b"]); // Option<&&str>
    let e: Option<&u8> = first(&[]);
    println!("{n:?} {s:?} {e:?}");
}
