//! A block `{ ... }` is an expression: the value of the last line without `;`

fn main() {
    let x = {
        let a = 10;
        let b = 20;
        a + b // no ; — this is the value of the block
    };
    println!("{x}"); // 30
}
