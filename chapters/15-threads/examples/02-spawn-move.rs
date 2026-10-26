//! `move` — give the data to the thread

use std::thread;

fn main() {
    let name = String::from("Vitalii");

    let h = thread::spawn(move || {
        println!("{name}"); // name now belongs to the thread
    });
    h.join().unwrap();
    // println!("{name}"); // ERROR E0382: name moved into the thread
}
