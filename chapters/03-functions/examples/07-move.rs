//! `move` — замикання забирає захоплене у власність

fn main() {
    let name = String::from("Віталій");
    let greet = move || println!("Привіт, {name}");
    greet();
    // println!("{name}");   // ПОМИЛКА: name переїхав у замикання (broken/07-use-after-move.rs)

    // Якщо оригінал ще потрібен — віддати замиканню копію:
    let city = String::from("Дніпро");
    let city_for_closure = city.clone();
    let show = move || println!("Місто: {city_for_closure}");
    show();
    println!("{city}"); // ok
}
