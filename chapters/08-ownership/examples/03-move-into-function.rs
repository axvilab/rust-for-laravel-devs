//! Функція з параметром `String` стає власником і знищує рядок у кінці

fn print_name(name: String) {
    println!("{name}");
} // name тут знищується

fn main() {
    let n = String::from("Віталій");
    print_name(n); // n переїхав у функцію
    // println!("{n}");                // ПОМИЛКА E0382 (broken/03-move-into-function.rs)
}
