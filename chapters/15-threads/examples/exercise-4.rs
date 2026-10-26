//! Вправа 4: `Rc<i32>` у потік не пройде — `Arc<i32>` пройде
//!
//! broken/10-rc-is-not-send.rs: error[E0277]: `Rc<i32>` cannot be sent between
//! threads safely — бо лічильник Rc не атомарний. Заміна Rc → Arc виправляє.

use std::sync::Arc;
use std::thread;

fn main() {
    let n = Arc::new(5);
    let n2 = Arc::clone(&n);
    let h = thread::spawn(move || println!("у потоці: {n2}"));
    h.join().unwrap();
    println!("у main: {n}, власників: {}", Arc::strong_count(&n));
}
