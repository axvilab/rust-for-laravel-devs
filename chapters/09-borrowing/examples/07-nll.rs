//! NLL: позичання живе до останнього використання, а не до `}`

fn main() {
    let mut scores = vec![10, 20];
    let first = &scores[0]; // позичання почалося
    println!("{first}"); // ...і закінчилося тут: останнє використання
    scores.push(30); // ok, хоча first ще «в області видимості»
    println!("{scores:?}");
}
