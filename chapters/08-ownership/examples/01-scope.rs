//! Scope = the life of a value: the string is freed at the end of the block

fn main() {
    {
        let name = String::from("Vitalii"); // name is created, it owns the string
        println!("{name}");
    } // name went out of scope → the string is freed
    // println!("{name}");   // ERROR: name doesn't exist (broken/01-out-of-scope.rs)
}
