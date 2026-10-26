//! Вправа 5: скільки подвоєнь, щоб 1 перевищило 1000 (`break значення`)

fn main() {
    let mut value = 1;
    let mut doublings = 0;
    let count = loop {
        if value > 1000 {
            break doublings; // loop повертає кількість
        }
        value *= 2;
        doublings += 1;
    };
    println!("{count} подвоєнь, значення {value}"); // 10 подвоєнь, 1024
}
