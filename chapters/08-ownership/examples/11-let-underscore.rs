//! `let _ =` destroys immediately, `let _b =` — at the end of the block

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("released: {}", self.0);
    }
}

fn main() {
    let _ = Guard("A"); // destroyed IMMEDIATELY, on this very line
    let _b = Guard("B"); // lives to the end of the block, like a normal variable
    println!("end of main");
}
// Prints: released: A / end of main / released: B
