//! Exercise 7: one variable became `_` — its value is destroyed immediately

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    {
        let _a = Noisy("first");
        let _ = Noisy("second"); // `_` isn't a variable: a value without an owner, dropped at once
        let _c = Noisy("third");
        println!("end of block");
    }
    // drop second / end of block / drop third / drop first
}
