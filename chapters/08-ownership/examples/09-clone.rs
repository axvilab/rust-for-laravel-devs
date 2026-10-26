//! Clone — явна (і глибока) копія

fn main() {
    let a = String::from("привіт");
    let b = a.clone(); // виділили нову пам'ять, скопіювали байти
    println!("{a} {b}"); // ok, обидва живі

    let tags = vec!["rust".to_string(), "php".to_string()];
    let copy = tags.clone(); // копія вектора І кожного рядка в ньому
    println!("{tags:?} {copy:?}");
}
