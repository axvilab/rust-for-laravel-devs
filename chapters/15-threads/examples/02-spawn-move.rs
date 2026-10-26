//! `move` — віддати дані потоку у володіння

use std::thread;

fn main() {
    let name = String::from("Віталій");

    let h = thread::spawn(move || {
        println!("{name}"); // name тепер належить потоку
    });
    h.join().unwrap();
    // println!("{name}"); // ПОМИЛКА E0382: name переїхав у потік
}
