//! `Rc<RefCell<T>>`: перевірка позичань у рантаймі; порушення = паніка

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let cart = Rc::new(RefCell::new(vec!["ноутбук"]));
    let same_cart = Rc::clone(&cart);

    same_cart.borrow_mut().push("миша"); // &mut через спільне Rc
    println!("{:?}", cart.borrow()); // ["ноутбук", "миша"]

    let r = cart.borrow(); // читач тримається...
    // cart.borrow_mut();  // ...тут була б паніка: RefCell already borrowed
    println!(
        "borrow_mut зараз можливий: {}",
        cart.try_borrow_mut().is_ok()
    ); // false
    drop(r);
    println!("а після drop(r): {}", cart.try_borrow_mut().is_ok()); // true
}
