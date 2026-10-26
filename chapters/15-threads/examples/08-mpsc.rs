//! An `mpsc` channel: many senders, one receiver

use std::sync::mpsc;
use std::thread;

fn process_job(id: u32) -> String {
    format!("job {id}")
}

fn main() {
    let (tx, rx) = mpsc::channel(); // tx — the sender, rx — the receiver

    for id in 0..4 {
        let tx = tx.clone(); // every thread gets its own sender
        thread::spawn(move || {
            let result = process_job(id);
            tx.send(result).unwrap();
        });
    }
    drop(tx); // close the "original" sender

    for result in rx {
        // the loop ends when all tx are destroyed
        println!("done: {result}");
    }
}
