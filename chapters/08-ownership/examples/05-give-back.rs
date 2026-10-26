//! Варіант А: віддати і отримати володіння назад (працює, але так не пишуть)

fn print_name(name: String) -> String {
    println!("{name}");
    name // повертаємо володіння
}

fn main() {
    let n = String::from("Віталій");
    let n = print_name(n);
    println!("знову моє: {n}");
}
