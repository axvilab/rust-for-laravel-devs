//! Замикання: `|x| x * 2` — це `fn($x) => $x * 2`

fn main() {
    let double = |x| x * 2;
    let greet = |name: &str| format!("Привіт, {name}");

    println!("{}", double(21)); // 42
    println!("{}", greet("Віталій"));

    let several_lines = |x: i32| {
        let y = x + 1;
        y * y
    };
    println!("{}", several_lines(3)); // 16
}
