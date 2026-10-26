//! Generic-функція `first<T>` — одна для будь-якого типу, без втрати типу

fn first<T>(items: &[T]) -> Option<&T> {
    items.first() // у книзі items.get(0); clippy радить first() — те саме
}

fn main() {
    let n = first(&[1, 2, 3]); // Option<&i32>
    let s = first(&["a", "b"]); // Option<&&str>
    let e: Option<&u8> = first(&[]);
    println!("{n:?} {s:?} {e:?}");
}
