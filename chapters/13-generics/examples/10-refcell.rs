//! `Rc<RefCell<T>>`: borrow checks at runtime; a violation = a panic

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let cart = Rc::new(RefCell::new(vec!["laptop"]));
    let same_cart = Rc::clone(&cart);

    same_cart.borrow_mut().push("mouse"); // &mut through a shared Rc
    println!("{:?}", cart.borrow()); // ["laptop", "mouse"]

    let r = cart.borrow(); // a reader is holding on...
    // cart.borrow_mut();  // ...this would panic: RefCell already borrowed
    println!("borrow_mut possible now: {}", cart.try_borrow_mut().is_ok()); // false
    drop(r);
    println!("and after drop(r): {}", cart.try_borrow_mut().is_ok()); // true
}
