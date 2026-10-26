//! What goes with what: Rc / Arc / Rc<RefCell> / Arc<Mutex> / Arc<RwLock> / Box

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

fn main() {
    // one thread, several owners, reading
    let a = Rc::new(String::from("config"));
    // one thread, several owners, reading and writing
    let b = Rc::new(RefCell::new(0));
    *b.borrow_mut() += 1;
    // many threads, reading
    let c = Arc::new(String::from("config"));
    // many threads, writing in turn
    let d = Arc::new(Mutex::new(0));
    *d.lock().unwrap() += 1;
    // many threads: many readers OR one writer
    let e = Arc::new(RwLock::new(String::from("v1")));
    e.write().unwrap().push_str("→v2");
    // one owner on the heap
    let f = Box::new([0u8; 16]);

    println!(
        "{a} {} {c} {} {} {}",
        b.borrow(),
        d.lock().unwrap(),
        e.read().unwrap(),
        f.len()
    );
}
