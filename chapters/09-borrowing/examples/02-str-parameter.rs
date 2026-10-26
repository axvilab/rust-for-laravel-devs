//! `&str` у параметрах приймає і `String`, і літерали

fn print_name(name: &str) {
    println!("{name}");
}

fn main() {
    let owned = String::from("Віталій");
    print_name(&owned); // &String автоматично стає &str
    print_name("Літерал"); // літерал — уже &str
    print_name(&owned[0..6]); // і шматок рядка (перші 6 байт = 3 кириличні літери)
}
