//! Пастка: затінення всередині блоку не змінює зовнішню змінну

fn main() {
    let has_discount = true;
    let total = 100;
    if has_discount {
        let total = total + 50; // НОВА змінна, живе лише в цьому блоці
        println!("всередині: {total}"); // 150
    }
    println!("зовні: {total}"); // 100 — зовнішню ніхто не чіпав

    // Правильно: mut зовні і зміна БЕЗ let усередині.
    let mut total = 100;
    if has_discount {
        total += 50;
    }
    println!("виправлено: {total}"); // 150
}
