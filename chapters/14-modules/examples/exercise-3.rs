//! Exercise 3: `cargo add thiserror anyhow`, `cargo tree` — who pulls in `syn`
//!
//! After `cargo add thiserror anyhow`, Cargo.toml gets two lines in
//! [dependencies], and Cargo.lock also pins the transitive ones: thiserror-impl,
//! proc-macro2, quote, syn, unicode-ident.
//!
//! In this repository (the ch12-errors crate: anyhow, thiserror, serde, toml):
//!
//! ```text
//! $ cargo tree -p ch12-errors -i syn -e normal
//! syn v3.0.9
//! ├── serde_derive v1.0.229 (proc-macro)
//! │   └── serde v1.0.229
//! └── thiserror-impl v2.0.21 (proc-macro)
//!     └── thiserror v2.0.21
//! ```
//!
//! The answer: `syn` (a Rust code parser) is pulled in by procedural macros — the derives
//! of thiserror and serde. anyhow doesn't need it: it has no derive macros.
//! Hence the slow first build: syn compiles before everything else.

fn main() {
    println!("see the comment in this file: cargo tree -p ch12-errors -i syn");
}
