//! Вправа 4: `Rc<String>`, три клони, `strong_count` до і після `drop`

use std::rc::Rc;

fn main() {
    let s = Rc::new(String::from("спільний рядок"));
    let a = Rc::clone(&s);
    let b = Rc::clone(&s);
    let c = Rc::clone(&s);
    println!("{}", Rc::strong_count(&s)); // 4: оригінал + три клони
    drop(b);
    println!("{}", Rc::strong_count(&s)); // 3
    println!("{a} / {c}"); // рядок один на всіх
}
