//! `mod serde;` for a dependency from Cargo.toml — that's not how it's done
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0583]: file not found for module `serde`
//! Fix: remove the `mod`; dependencies are available right away: `use serde::Deserialize;`
// expected-error: E0583

mod serde;

fn main() {}
