//! Вправа 5: `calc_fee` у lib.rs з doc-прикладом — і чому без `use` він падає
//!
//! Див. документацію `calc_fee` у src/lib.rs: там два doc-тести.
//! 1) з `use ch14_modules::calc_fee;` — проходить;
//! 2) без `use` — позначений ```compile_fail,E0425```: rustdoc перевіряє, що він
//!    НЕ компілюється саме з E0425 (cannot find function `calc_fee` in this scope).
//!
//! Запуск лише doc-тестів: `cargo test -p ch14-modules --doc`

use ch14_modules::calc_fee;

fn main() {
    println!("{}", calc_fee(1000)); // 20
}
