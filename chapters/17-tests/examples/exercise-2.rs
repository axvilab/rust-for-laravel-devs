//! Exercise 2: `println!` in a passing test — visible only with `--nocapture`
//!
//!   cargo test -p ch17-tests --example exercise-2                  # no "calculating..." line
//!   cargo test -p ch17-tests --example exercise-2 -- --nocapture   # there it is
//! cargo hides the output of tests that PASSED; the output of failed tests is always shown.

use ch17_tests::calc_fee;

fn main() {
    println!("{}", calc_fee(5000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_for_5000() {
        println!("calculating the fee for 5000...");
        assert_eq!(calc_fee(5000), 100);
    }
}
