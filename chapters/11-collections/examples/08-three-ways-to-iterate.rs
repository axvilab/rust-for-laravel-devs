//! `for s in &v` (позичити), `&mut v` (змінювати), `v` (забрати)

fn main() {
    let mut v = vec![String::from("a"), String::from("b")];

    for s in &v {
        // s: &String — позичаємо, v живий після циклу
        print!("{s} ");
    }
    println!();
    for s in &mut v {
        // s: &mut String — можна міняти елементи
        s.push('!');
    }
    for s in v {
        // s: String — забираємо, v більше нема
        print!("{s} ");
    }
    println!();
    // println!("{v:?}"); // ПОМИЛКА E0382 (broken/09-for-moves-vec.rs)
}
