//! Exercise 4: an `Rc<i32>` won't go into a thread — an `Arc<i32>` will
//!
//! broken/10-rc-is-not-send.rs: error[E0277]: `Rc<i32>` cannot be sent between
//! threads safely — because Rc's counter isn't atomic. Replacing Rc → Arc fixes it.

use std::sync::Arc;
use std::thread;

fn main() {
    let n = Arc::new(5);
    let n2 = Arc::clone(&n);
    let h = thread::spawn(move || println!("in the thread: {n2}"));
    h.join().unwrap();
    println!("in main: {n}, owners: {}", Arc::strong_count(&n));
}
