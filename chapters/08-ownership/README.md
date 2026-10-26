# 8. Ownership — один власник на кожну річ

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch08-ownership --example 01-scope
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch08-ownership --example 01-scope
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-scope`](examples/01-scope.rs) | Область видимості = життя значення: рядок звільняється в кінці блоку | `cargo run -p ch08-ownership --example 01-scope` |
| [`02-move`](examples/02-move.rs) | Move: після `let b = a` власник рядка — `b` | `cargo run -p ch08-ownership --example 02-move` |
| [`03-move-into-function`](examples/03-move-into-function.rs) | Функція з параметром `String` стає власником і знищує рядок у кінці | `cargo run -p ch08-ownership --example 03-move-into-function` |
| [`04-move-in-loop-fixed`](examples/04-move-in-loop-fixed.rs) | Два виправлення E0382 у циклі: позичити (краще) або клонувати | `cargo run -p ch08-ownership --example 04-move-in-loop-fixed` |
| [`05-give-back`](examples/05-give-back.rs) | Варіант А: віддати і отримати володіння назад (працює, але так не пишуть) | `cargo run -p ch08-ownership --example 05-give-back` |
| [`06-copy-types`](examples/06-copy-types.rs) | Copy: числа копіюються, а не переїжджають | `cargo run -p ch08-ownership --example 06-copy-types` |
| [`07-copy-struct`](examples/07-copy-struct.rs) | `#[derive(Clone, Copy)]` для маленької «числової» структури | `cargo run -p ch08-ownership --example 07-copy-struct` |
| [`08-copy-is-a-copy`](examples/08-copy-is-a-copy.rs) | Copy-структура — окрема копія, а не «той самий об'єкт», як у PHP | `cargo run -p ch08-ownership --example 08-copy-is-a-copy` |
| [`09-clone`](examples/09-clone.rs) | Clone — явна (і глибока) копія | `cargo run -p ch08-ownership --example 09-clone` |
| [`10-drop`](examples/10-drop.rs) | `impl Drop` — детермінований `__destruct()`; `drop(x)` — знищити раніше | `cargo run -p ch08-ownership --example 10-drop` |
| [`11-let-underscore`](examples/11-let-underscore.rs) | `let _ =` знищує одразу, `let _b =` — у кінці блоку | `cargo run -p ch08-ownership --example 11-let-underscore` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `let t = s;` — і `s` більше недоступний | `cargo run -p ch08-ownership --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: з числом помилки немає — `i32` реалізує `Copy` | `cargo run -p ch08-ownership --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `shout(String)` забирає рядок — два способи виправити | `cargo run -p ch08-ownership --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: порядок знищення — зворотний до порядку оголошення | `cargo run -p ch08-ownership --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: зі `String` усередині — лише `Clone`, копія явна | `cargo run -p ch08-ownership --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: рядок у функцію в циклі — обидва виправлення по черзі | `cargo run -p ch08-ownership --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Вправа 7: одна змінна стала `_` — її значення знищується одразу | `cargo run -p ch08-ownership --example exercise-7` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 08
rustc --edition 2024 chapters/08-ownership/broken/01-out-of-scope.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`01-out-of-scope.rs`](broken/01-out-of-scope.rs) | Змінна поза своїм блоком не існує | error[E0425]: cannot find value `name` in this scope |
| [`02-move.rs`](broken/02-move.rs) | `let b = a` для String — переїзд: `a` більше не можна використовувати | error[E0382]: borrow of moved value: `a` |
| [`03-move-into-function.rs`](broken/03-move-into-function.rs) | Передав `String` у функцію — віддав | error[E0382]: borrow of moved value: `n` |
| [`04-move-in-loop.rs`](broken/04-move-in-loop.rs) | Move у циклі: «value moved here, in previous iteration of loop» | error[E0382]: use of moved value: `email` |
| [`exercise-1.rs`](broken/exercise-1.rs) | Вправа 1 (зламана версія): використання після move | error[E0382]: borrow of moved value: `s` |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3 (зламана версія): рядок використано після передачі у `shout` | error[E0382]: borrow of moved value: `s` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5: `Copy` для структури з полем `String` | error[E0204]: the trait `Copy` cannot be implemented for this type |
| [`exercise-6.rs`](broken/exercise-6.rs) | Вправа 6 (зламана версія): той самий String тричі в функцію, що приймає String | error[E0382]: use of moved value: `email` |
