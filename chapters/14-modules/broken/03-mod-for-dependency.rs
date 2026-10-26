//! `mod serde;` для залежності з Cargo.toml — так не роблять
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0583]: file not found for module `serde`
//! Виправлення: прибрати `mod`; залежності доступні одразу: `use serde::Deserialize;`
// expected-error: E0583

mod serde;

fn main() {}
