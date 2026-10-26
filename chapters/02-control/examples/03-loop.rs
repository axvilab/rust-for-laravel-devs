//! `loop` — an infinite loop until `break` (in PHP: `while (true)`)

// A stand-in for "connecting": succeeds on the third attempt.
fn try_connect(attempt: u32) -> bool {
    println!("attempt {attempt}...");
    attempt == 3
}

fn main() {
    let mut attempts = 0;
    loop {
        attempts += 1;
        if try_connect(attempts) {
            break;
        }
        if attempts == 5 {
            panic!("failed to connect");
        }
    }
    println!("connected on attempt {attempts}");
}
