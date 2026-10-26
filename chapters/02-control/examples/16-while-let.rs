//! `while let` — loop while the pattern matches

fn process(job: &str) {
    println!("processing {job}");
}

fn main() {
    let mut queue = vec!["email", "invoice", "report"];
    while let Some(job) = queue.pop() {
        process(job);
    }
    // pop() takes from the end: report, invoice, email
}
