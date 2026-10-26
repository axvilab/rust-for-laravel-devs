//! `impl Drop` — a deterministic `__destruct()`; `drop(x)` — destroy earlier

struct Connection {
    id: u32,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("connection {} closed", self.id);
    }
}

fn main() {
    {
        let _c = Connection { id: 1 };
        println!("working with connection 1");
    } // "connection 1 closed" is printed here

    let c2 = Connection { id: 2 };
    drop(c2); // explicitly, before the end of the block
    println!("end of main");
}
