//! Exercise 4: `Rc<String>`, three clones, `strong_count` before and after `drop`

use std::rc::Rc;

fn main() {
    let s = Rc::new(String::from("a shared string"));
    let a = Rc::clone(&s);
    let b = Rc::clone(&s);
    let c = Rc::clone(&s);
    println!("{}", Rc::strong_count(&s)); // 4: the original + three clones
    drop(b);
    println!("{}", Rc::strong_count(&s)); // 3
    println!("{a} / {c}"); // one string for everyone
}
