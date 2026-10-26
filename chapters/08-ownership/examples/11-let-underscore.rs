//! `let _ =` знищує одразу, `let _b =` — у кінці блоку

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("звільнено: {}", self.0);
    }
}

fn main() {
    let _ = Guard("A"); // знищується ОДРАЗУ, у цьому ж рядку
    let _b = Guard("B"); // живе до кінця блоку, як звичайна змінна
    println!("кінець main");
}
// Виведе: звільнено: A / кінець main / звільнено: B
