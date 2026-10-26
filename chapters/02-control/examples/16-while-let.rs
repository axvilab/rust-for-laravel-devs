//! `while let` — цикл, поки шаблон збігається

fn process(job: &str) {
    println!("обробляю {job}");
}

fn main() {
    let mut queue = vec!["email", "invoice", "report"];
    while let Some(job) = queue.pop() {
        process(job);
    }
    // pop() знімає з кінця: report, invoice, email
}
