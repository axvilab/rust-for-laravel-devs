//! Вправа 3: `cargo add thiserror anyhow`, `cargo tree` — хто тягне `syn`
//!
//! Після `cargo add thiserror anyhow` у Cargo.toml з'являються два рядки в
//! [dependencies], а Cargo.lock фіксує ще й транзитивні: thiserror-impl,
//! proc-macro2, quote, syn, unicode-ident.
//!
//! У цьому репозиторії (крейт ch12-errors: anyhow, thiserror, serde, toml):
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
//! Відповідь: `syn` (парсер Rust-коду) тягнуть процедурні макроси — derive з
//! thiserror і serde. anyhow його не потребує: у нього немає макросів-derive.
//! Звідси повільна перша збірка: syn компілюється раніше за все інше.

fn main() {
    println!("див. коментар у файлі: cargo tree -p ch12-errors -i syn");
}
