//! Exercise 4: drop order is the reverse of declaration order

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    {
        let _a = Noisy("first");
        let _b = Noisy("second");
        let _c = Noisy("third");
        println!("end of block");
    }
    // drop third / drop second / drop first — like a stack: the last one created is destroyed first
}
