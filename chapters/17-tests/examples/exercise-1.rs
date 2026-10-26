//! Exercise 1: a test that fails on purpose, with an explanation in `assert_eq!`
//!
//! The test is marked `#[ignore]` so that `cargo test` passes in CI. Run it with:
//!   cargo test -p ch17-tests --example exercise-1 -- --ignored
//! Output:
//!   ---- tests::broken stdout ----
//!   calculated: 20
//!   thread 'tests::broken' panicked at ...:
//!   assertion `left == right` failed: fee for 1000
//!     left: 20
//!    right: 25
//! `left` is what the code calculated, `right` is what you expected.

use ch17_tests::calc_fee;

fn main() {
    println!("calc_fee(1000) = {}", calc_fee(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "fails on purpose: cargo test --example exercise-1 -- --ignored"]
    fn broken() {
        let fee = calc_fee(1000);
        println!("calculated: {fee}");
        assert_eq!(fee, 25, "fee for 1000");
    }
}
