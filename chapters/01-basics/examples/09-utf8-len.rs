//! Довжина рядка: байти (`len`) проти символів (`chars().count()`)

fn main() {
    let s = "Дніпро";
    println!("{}", s.len()); // 12 (байт)
    println!("{}", s.chars().count()); // 6 (символів)
}
