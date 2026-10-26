//! `'_` — анонімний лайфтайм: «тут позичено, деталі неважливі»

use std::fmt;

struct Money(i64);

impl fmt::Display for Money {
    // Formatter<'_> — у Formatter є лайфтайм, але називати його не треба
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} грн", self.0)
    }
}

struct Excerpt<'a> {
    part: &'a str,
}

// impl для структури з лайфтаймом: '_ замість вигаданого імені
impl Excerpt<'_> {
    fn len(&self) -> usize {
        self.part.chars().count()
    }
}

fn main() {
    println!("{}", Money(1500));
    let text = String::from("Розділ про лайфтайми");
    let e = Excerpt { part: &text };
    println!("{}", e.len());
}
