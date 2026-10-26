//! Вичерпний `match` по bool: обидві гілки, `_` не потрібен

fn main() {
    let x: bool = true;
    match x {
        true => println!("так"),
        false => println!("ні"),
    }
}
