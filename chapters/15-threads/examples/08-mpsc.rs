//! Канал `mpsc`: багато відправників, один отримувач

use std::sync::mpsc;
use std::thread;

fn process_job(id: u32) -> String {
    format!("задача {id}")
}

fn main() {
    let (tx, rx) = mpsc::channel(); // tx — відправник, rx — отримувач

    for id in 0..4 {
        let tx = tx.clone(); // кожен потік — свій відправник
        thread::spawn(move || {
            let result = process_job(id);
            tx.send(result).unwrap();
        });
    }
    drop(tx); // закрити «оригінальний» відправник

    for result in rx {
        // цикл закінчиться, коли всі tx знищені
        println!("готово: {result}");
    }
}
