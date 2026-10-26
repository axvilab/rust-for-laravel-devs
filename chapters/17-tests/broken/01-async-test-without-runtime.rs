//! `#[test]` on an `async fn` — the test runner can't wait for a future
//!
//! DOES NOT COMPILE (in a test build) — on purpose.
//! Expected error: error: async functions cannot be used for tests
//! Fix: `#[tokio::test]` — see src/lib.rs, the test `fetches_user`
// rustc-flags: --test
// expected-error: async functions cannot be used for tests

#[test]
async fn fetches_user() {
    assert_eq!(1, 1);
}

fn main() {}
