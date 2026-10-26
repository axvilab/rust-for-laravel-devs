//! `[i]` панікує за межами, `.get(i)` повертає `Option<&T>`; `.copied()` знімає `&`
#![allow(clippy::useless_vec)] // показуємо саме Vec, хоча масив тут теж підійшов би

fn main() {
    let names = vec!["Іван", "Марія"];

    // У книзі `names.get(0)`; для індексу 0 clippy радить `.first()` (get_first),
    // тому тут другий елемент.
    let maybe: Option<&&str> = names.get(1); // посилання на елемент (&str) → &&str
    let name: Option<&str> = names.get(1).copied(); // для Copy-типів — copied()
    println!("{maybe:?} {name:?}");

    let idx = 10; // «прийшло ззовні»
    match names.get(idx) {
        Some(n) => println!("{n}"),
        None => println!("нема елемента {idx}"),
    }
    // println!("{}", names[idx]); // паніка: index out of bounds: the len is 2 but the index is 10

    let owned = vec![String::from("a")];
    let first: Option<String> = owned.first().cloned(); // для String — cloned()
    println!("{first:?}");
}
