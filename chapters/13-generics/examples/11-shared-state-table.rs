//! Що з чим поєднувати: Rc / Arc / Rc<RefCell> / Arc<Mutex> / Arc<RwLock> / Box

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

fn main() {
    // один потік, кілька власників, читання
    let a = Rc::new(String::from("конфіг"));
    // один потік, кілька власників, читання і запис
    let b = Rc::new(RefCell::new(0));
    *b.borrow_mut() += 1;
    // багато потоків, читання
    let c = Arc::new(String::from("конфіг"));
    // багато потоків, запис по черзі
    let d = Arc::new(Mutex::new(0));
    *d.lock().unwrap() += 1;
    // багато потоків: багато читачів АБО один писець
    let e = Arc::new(RwLock::new(String::from("v1")));
    e.write().unwrap().push_str("→v2");
    // один власник у купі
    let f = Box::new([0u8; 16]);

    println!(
        "{a} {} {c} {} {} {}",
        b.borrow(),
        d.lock().unwrap(),
        e.read().unwrap(),
        f.len()
    );
}
