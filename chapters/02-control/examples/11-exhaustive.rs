//! An exhaustive `match` on bool: both arms, no `_` needed

fn main() {
    let x: bool = true;
    match x {
        true => println!("yes"),
        false => println!("no"),
    }
}
