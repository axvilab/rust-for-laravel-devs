//! `into_iter()` on `&Vec` borrows, on `Vec` it takes

fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in &v {
        // the same as v.iter() → s: &String
        print!("{s} ");
    }
    let r = &v;
    // clippy suggests `r.iter()` here (into_iter_on_ref) — and that's the point:
    // on a reference into_iter() does the same as iter().
    #[allow(clippy::into_iter_on_ref)]
    for s in r.into_iter() {
        // into_iter on &Vec — also &String!
        print!("{s} ");
    }
    for s in v {
        // the same as v.into_iter() → s: String
        print!("{s} ");
    }
    println!();
}
