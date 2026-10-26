//! Exercise 3: why `push` is forbidden while `&v[0]` is alive
//!
//! Broken version — broken/exercise-3.rs (E0502). Here you can see the reason: when
//! `push` goes past the capacity, the vector asks the allocator for a bigger buffer.
//! The allocator sometimes grows the block in place, and sometimes moves the data
//! elsewhere and frees the old block. The compiler can't know what will happen at
//! runtime, so it forbids any reference that would outlive the `push`.

fn main() {
    let mut v = Vec::with_capacity(3);
    v.extend([1, 2, 3]);
    let (ptr_before, cap_before) = (v.as_ptr(), v.capacity());

    v.push(4); // capacity 3 exhausted → reallocation

    println!("capacity: {cap_before} → {}", v.capacity());
    println!(
        "buffer address changed: {} (depends on the allocator)",
        ptr_before != v.as_ptr()
    );
}
