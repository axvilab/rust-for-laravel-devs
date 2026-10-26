# 15. Потоки — паралельність, якої в PHP не було

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch15-threads --example 01-spawn-join
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch15-threads --example 01-spawn-join
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-spawn-join`](examples/01-spawn-join.rs) | `thread::spawn` — запустити потік; `join` — дочекатися і забрати результат | `cargo run -p ch15-threads --example 01-spawn-join` |
| [`02-spawn-move`](examples/02-spawn-move.rs) | `move` — віддати дані потоку у володіння | `cargo run -p ch15-threads --example 02-spawn-move` |
| [`03-arc-mutex`](examples/03-arc-mutex.rs) | `Arc<Mutex<T>>`: кілька власників + змінювати по черзі | `cargo run -p ch15-threads --example 03-arc-mutex` |
| [`05-short-lock`](examples/05-short-lock.rs) | Тримай замок коротко: повільну роботу — до `lock()` | `cargo run -p ch15-threads --example 05-short-lock` |
| [`06-match-on-lock`](examples/06-match-on-lock.rs) | Тимчасовий guard у `match` живе до кінця всього `match` | `cargo run -p ch15-threads --example 06-match-on-lock` |
| [`07-rwlock`](examples/07-rwlock.rs) | `RwLock` — багато читачів або один писець | `cargo run -p ch15-threads --example 07-rwlock` |
| [`08-mpsc`](examples/08-mpsc.rs) | Канал `mpsc`: багато відправників, один отримувач | `cargo run -p ch15-threads --example 08-mpsc` |
| [`09-scoped-threads`](examples/09-scoped-threads.rs) | `thread::scope` — позичити локальні дані потокам без `Arc` і `move` | `cargo run -p ch15-threads --example 09-scoped-threads` |
| [`10-boxed-send-jobs`](examples/10-boxed-send-jobs.rs) | `Vec<Box<dyn FnOnce() -> u64 + Send>>` — список задач для потоків | `cargo run -p ch15-threads --example 10-boxed-send-jobs` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: 4 потоки рахують суми частин `0..1_000_000`, результати — через `join` | `cargo run -p ch15-threads --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: те саме через спільний `Arc<Mutex<u64>>` | `cargo run -p ch15-threads --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: те саме через канал `mpsc` | `cargo run -p ch15-threads --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `Rc<i32>` у потік не пройде — `Arc<i32>` пройде | `cargo run -p ch15-threads --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `match` по `lock()` тримає замок до кінця match — `try_lock` це показує | `cargo run -p ch15-threads --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: дедлок через різний порядок замків — і виправлення одним порядком | `cargo run -p ch15-threads --example exercise-6` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 15
rustc --edition 2024 chapters/15-threads/broken/02-spawn-without-move.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-spawn-without-move.rs`](broken/02-spawn-without-move.rs) | Потік позичає локальну змінну — вона може не дожити | error[E0373]: closure may outlive the current function, but it |
| [`04-arc-not-cloned.rs`](broken/04-arc-not-cloned.rs) | Забули `Arc::clone` перед `move` у циклі | error[E0382]: the type `Arc` does not implement `Copy` |
| [`10-rc-is-not-send.rs`](broken/10-rc-is-not-send.rs) | `Rc` не можна передати в інший потік | error[E0277]: `Rc<i32>` cannot be sent between threads safely |
