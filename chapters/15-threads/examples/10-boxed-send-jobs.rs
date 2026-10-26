//! `Vec<Box<dyn FnOnce() -> u64 + Send>>` — a list of jobs for threads

use std::thread;

// clippy (vec_init_then_push) suggests `vec![...]` instead of new + push. The push
// is on purpose: that's how jobs get added one by one in real code.
#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut jobs: Vec<Box<dyn FnOnce() -> u64 + Send>> = Vec::new();
    jobs.push(Box::new(|| 1 + 1));
    jobs.push(Box::new(move || 40 + 2));

    // Not `.map(|job| thread::spawn(job))`: clippy (redundant_closure) suggests
    // passing the function by name — as in the chapter 3 nuance.
    let handles: Vec<_> = jobs.into_iter().map(thread::spawn).collect();
    for h in handles {
        println!("{}", h.join().unwrap()); // 2, 42
    }
}
