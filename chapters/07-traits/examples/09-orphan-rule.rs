//! Working around the orphan rule: your own wrapper type around `Vec<String>`

use std::fmt;

struct Tags(Vec<String>);

impl fmt::Display for Tags {
    // ok, Tags is your type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0.join(" #"))
    }
}

fn main() {
    let tags = Tags(vec!["rust".into(), "laravel".into(), "php".into()]);
    println!("{tags}"); // #rust #laravel #php
}
