//! Exercise 4: `let v2 = v;` vs `let v2 = v.clone();`
//!
//! After `let v2 = v;` the vector moved: `v` is no longer available (E0382,
//! see broken/exercise-4.rs). After `let v2 = v.clone();` there are two independent
//! copies, and `v` stays available.

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // a copy: both alive
    println!("{v:?} {v2:?}");

    let v3 = v; // move: v is unavailable now
    println!("{v3:?}");
}
