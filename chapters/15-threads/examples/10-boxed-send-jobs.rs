//! `Vec<Box<dyn FnOnce() -> u64 + Send>>` — список задач для потоків

use std::thread;

// clippy (vec_init_then_push) радить `vec![...]` замість new + push. Тут push
// навмисно: так задачі додають по одній у реальному коді.
#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut jobs: Vec<Box<dyn FnOnce() -> u64 + Send>> = Vec::new();
    jobs.push(Box::new(|| 1 + 1));
    jobs.push(Box::new(move || 40 + 2));

    // У книзі `.map(|job| thread::spawn(job))`; clippy (redundant_closure) радить
    // передати функцію за іменем — як у нюансі з розділу 3.
    let handles: Vec<_> = jobs.into_iter().map(thread::spawn).collect();
    for h in handles {
        println!("{}", h.join().unwrap()); // 2, 42
    }
}
