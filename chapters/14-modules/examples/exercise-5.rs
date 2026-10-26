//! Exercise 5: `calc_fee` in lib.rs with a doc example — and why it fails without `use`
//!
//! See the documentation of `calc_fee` in src/lib.rs: there are two doc tests.
//! 1) with `use ch14_modules::calc_fee;` — passes;
//! 2) without `use` — marked ```compile_fail,E0425```: rustdoc checks that it
//!    does NOT compile, with exactly E0425 (cannot find function `calc_fee` in this scope).
//!
//! Run only the doc tests: `cargo test -p ch14-modules --doc`

use ch14_modules::calc_fee;

fn main() {
    println!("{}", calc_fee(1000)); // 20
}
