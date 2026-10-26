# 10. Лайфтайми — коротко і без паніки

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch10-lifetimes --example 01-longer
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch10-lifetimes --example 01-longer
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-longer`](examples/01-longer.rs) | `'a` описує зв'язок: результат живе не довше за коротший з аргументів | `cargo run -p ch10-lifetimes --example 01-longer` |
| [`02-elision`](examples/02-elision.rs) | Правила виведення: один параметр-посилання або `&self` — лайфтайми не потрібні | `cargo run -p ch10-lifetimes --example 02-elision` |
| [`03-return-owned`](examples/03-return-owned.rs) | Створив рядок у функції — повертай `String`, не `&str` | `cargo run -p ch10-lifetimes --example 03-return-owned` |
| [`04-struct-with-ref`](examples/04-struct-with-ref.rs) | Структура з посиланням потребує лайфтайму і не переживе джерело | `cargo run -p ch10-lifetimes --example 04-struct-with-ref` |
| [`05-static`](examples/05-static.rs) | `'static` — посилання, валідне до кінця програми (рядкові літерали) | `cargo run -p ch10-lifetimes --example 05-static` |
| [`06-static-bound`](examples/06-static-bound.rs) | `T: 'static` виконують власні типи: String, Vec, числа, структури з String | `cargo run -p ch10-lifetimes --example 06-static-bound` |
| [`07-anonymous-lifetime`](examples/07-anonymous-lifetime.rs) | `'_` — анонімний лайфтайм: «тут позичено, деталі неважливі» | `cargo run -p ch10-lifetimes --example 07-anonymous-lifetime` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `pick` з двома літералами, потім з літералом і `&String` | `cargo run -p ch10-lifetimes --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: що пропонує компілятор, якщо прибрати лайфтайми | `cargo run -p ch10-lifetimes --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `Excerpt<'a>` працює, поки джерело живе довше | `cargo run -p ch10-lifetimes --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `Excerpt` з `String` — лайфтайм зникає | `cargo run -p ch10-lifetimes --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `make_greeting` повертає `String` | `cargo run -p ch10-lifetimes --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 10
rustc --edition 2024 chapters/10-lifetimes/broken/01-missing-lifetime.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`01-missing-lifetime.rs`](broken/01-missing-lifetime.rs) | Два посилання на вході, посилання на виході — компілятор просить лайфтайм | error[E0106]: missing lifetime specifier |
| [`03-return-ref-to-local.rs`](broken/03-return-ref-to-local.rs) | Лайфтайм не врятує посилання на локальну змінну | error[E0515]: cannot return reference to local variable `s` |
| [`06-static-bound.rs`](broken/06-static-bound.rs) | `T: 'static` — «не містить позичених посилань», а не «живе вічно» | error[E0597]: `local` does not live long enough |
| [`exercise-2.rs`](broken/exercise-2.rs) | Вправа 2: `pick` без лайфтаймів | error[E0106]: missing lifetime specifier |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3: `Excerpt` з посиланням на String, що знищився в кінці блоку | error[E0597]: `text` does not live long enough |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5: повернути `&format!(...)` з функції | error[E0515]: cannot return reference to temporary value |
