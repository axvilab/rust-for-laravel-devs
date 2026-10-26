# 9. Borrowing — позичити, не забираючи

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch09-borrowing --example 01-shared-borrow
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch09-borrowing --example 01-shared-borrow
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-shared-borrow`](examples/01-shared-borrow.rs) | `&T` — позичити подивитись: власник лишається власником | `cargo run -p ch09-borrowing --example 01-shared-borrow` |
| [`02-str-parameter`](examples/02-str-parameter.rs) | `&str` у параметрах приймає і `String`, і літерали | `cargo run -p ch09-borrowing --example 02-str-parameter` |
| [`03-mut-borrow`](examples/03-mut-borrow.rs) | `&mut T` — позичити з правом змін: три `mut`, і всі потрібні | `cargo run -p ch09-borrowing --example 03-mut-borrow` |
| [`04-mut-binding-vs-mut-ref`](examples/04-mut-binding-vs-mut-ref.rs) | `let mut r = &x` (можна перенаправити) проти `let r = &mut x` (можна змінити) | `cargo run -p ch09-borrowing --example 04-mut-binding-vs-mut-ref` |
| [`05-borrow-rules`](examples/05-borrow-rules.rs) | Або багато `&`, або один `&mut`; позичання закінчується на останньому використанні | `cargo run -p ch09-borrowing --example 05-borrow-rules` |
| [`06-collect-then-push`](examples/06-collect-then-push.rs) | Спочатку зібрати, що додати, потім додати | `cargo run -p ch09-borrowing --example 06-collect-then-push` |
| [`07-nll`](examples/07-nll.rs) | NLL: позичання живе до останнього використання, а не до `}` | `cargo run -p ch09-borrowing --example 07-nll` |
| [`08-two-phase-borrows`](examples/08-two-phase-borrows.rs) | `v.push(v.len())` працює (two-phase borrows), а `v.push(v.pop())` — ні | `cargo run -p ch09-borrowing --example 08-two-phase-borrows` |
| [`09-return-owned`](examples/09-return-owned.rs) | Повертаємо `String` (володіння), а не `&str` на локальну змінну | `cargo run -p ch09-borrowing --example 09-return-owned` |
| [`10-deref`](examples/10-deref.rs) | Розіменування `*`: значення за посиланням | `cargo run -p ch09-borrowing --example 10-deref` |
| [`11-match-on-reference`](examples/11-match-on-reference.rs) | `match &x` дивиться, `match x` забирає вміст | `cargo run -p ch09-borrowing --example 11-match-on-reference` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `count_vowels(&str)` працює і для String, і для літерала | `cargo run -p ch09-borrowing --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `append_exclamation(&mut String)` вимагає `let mut` у власника | `cargo run -p ch09-borrowing --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: чому `push` при живому `&v[0]` заборонений | `cargo run -p ch09-borrowing --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: два робочі варіанти — скопіювати значення або вивести до `push` | `cargo run -p ch09-borrowing --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `match &maybe` — один символ, і `maybe` лишається живим | `cargo run -p ch09-borrowing --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: `push(v.len())` працює, `push(v.pop())` — через тимчасову змінну | `cargo run -p ch09-borrowing --example exercise-6` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 09
rustc --edition 2024 chapters/09-borrowing/broken/04-assign-through-shared-ref.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`04-assign-through-shared-ref.rs`](broken/04-assign-through-shared-ref.rs) | Змінити значення через `&` не можна | error[E0594]: cannot assign to `*s`, which is behind a `&` reference |
| [`05-borrow-rules.rs`](broken/05-borrow-rules.rs) | Читач і редактор одночасно | error[E0502]: cannot borrow `list` as immutable because it is |
| [`06-mutate-while-iterating.rs`](broken/06-mutate-while-iterating.rs) | Змінювати колекцію, поки по ній ітеруєшся, не можна | error[E0502]: cannot borrow `list` as mutable because it is |
| [`07-nll-late-use.rs`](broken/07-nll-late-use.rs) | Те саме, але `first` використано ПІСЛЯ `push` | error[E0502]: cannot borrow `scores` as mutable because it is |
| [`08-two-mut-in-args.rs`](broken/08-two-mut-in-args.rs) | Два `&mut` одночасно: змінювати в аргументах того ж виклику не можна | error[E0499]: cannot borrow `v` as mutable more than once at a time |
| [`09-return-ref-to-local.rs`](broken/09-return-ref-to-local.rs) | Посилання на локальну змінну не можна повернути | error[E0106]: missing lifetime specifier |
| [`11-partially-moved.rs`](broken/11-partially-moved.rs) | `match` без `&` забрав рядок з Option — `name` більше не цілий | error[E0382]: borrow of partially moved value: `name` |
| [`exercise-2.rs`](broken/exercise-2.rs) | Вправа 2 (зламана версія): `&mut` на незмінну змінну | error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3: посилання на елемент, потім `push`, потім читання посилання | error[E0502]: cannot borrow `v` as mutable because it is |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5 (зламана версія): `match` без `&` забрав вміст Option | error[E0382]: borrow of partially moved value: `maybe` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Вправа 6 (зламана версія): `v.push(v.pop().unwrap())` | error[E0499]: cannot borrow `v` as mutable more than once at a time |
