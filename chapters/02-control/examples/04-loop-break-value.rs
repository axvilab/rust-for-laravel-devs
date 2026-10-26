//! `break value` — the result of the whole `loop`

// A stand-in "number source": 10, 11, 12, ...
fn next_number(counter: &mut i32) -> i32 {
    *counter += 1;
    9 + *counter
}

fn main() {
    let mut counter = 0;
    let result = loop {
        let n = next_number(&mut counter);
        if n % 7 == 0 {
            break n; // break with a value — the result of the whole loop
        }
    };
    println!("{result}"); // 14
}
