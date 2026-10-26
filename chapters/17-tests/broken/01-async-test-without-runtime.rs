//! `#[test]` над `async fn` — тест-раннер не вміє чекати на future
//!
//! НЕ КОМПІЛЮЄТЬСЯ (у тестовій збірці) — так задумано.
//! Очікувана помилка: error: async functions cannot be used for tests
//! Виправлення: `#[tokio::test]` — див. src/lib.rs, тест `fetches_user`
// rustc-flags: --test
// expected-error: async functions cannot be used for tests

#[test]
async fn fetches_user() {
    assert_eq!(1, 1);
}

fn main() {}
