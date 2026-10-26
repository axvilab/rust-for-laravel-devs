//! Elision rules: one reference parameter or `&self` — no lifetimes needed

struct User {
    name: String,
}

// rule 2: one reference parameter → the result gets its lifetime
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

impl User {
    // rule 3: there's &self → the result gets the lifetime of self
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    println!("{}", first_word("Hello, world"));
    let u = User {
        name: "Vitalii".into(),
    };
    println!("{}", u.name());
}
