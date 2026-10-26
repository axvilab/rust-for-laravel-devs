//! `'a` describes a relationship: the result lives no longer than the shorter argument

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("Dnipro", "Kyiv")); // Dnipro

    let owned = String::from("Zaporizhzhia");
    let result;
    {
        let short = String::from("Sumy");
        result = longer(&owned, &short);
        println!("{result}"); // ok: short is still alive
    }
    // println!("{result}"); // ERROR E0597: `short` does not live long enough
}
