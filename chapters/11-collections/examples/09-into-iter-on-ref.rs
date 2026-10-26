//! `into_iter()` на `&Vec` позичає, на `Vec` — забирає

fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in &v {
        // те саме, що v.iter() → s: &String
        print!("{s} ");
    }
    let r = &v;
    // clippy радить тут `r.iter()` (into_iter_on_ref) — і саме в цьому суть:
    // на посиланні into_iter() робить те саме, що iter().
    #[allow(clippy::into_iter_on_ref)]
    for s in r.into_iter() {
        // into_iter на &Vec — теж &String!
        print!("{s} ");
    }
    for s in v {
        // те саме, що v.into_iter() → s: String
        print!("{s} ");
    }
    println!();
}
