//! `impl Drop` — детермінований `__destruct()`; `drop(x)` — знищити раніше

struct Connection {
    id: u32,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("з'єднання {} закрито", self.id);
    }
}

fn main() {
    {
        let _c = Connection { id: 1 };
        println!("працюємо з з'єднанням 1");
    } // тут надрукується "з'єднання 1 закрито"

    let c2 = Connection { id: 2 };
    drop(c2); // явно, раніше кінця блоку
    println!("кінець main");
}
