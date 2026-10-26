//! Two fixes for E0382 in a loop: borrow (better) or clone

// 1. The function only reads → a &str parameter, pass &email
fn send(email: &str) {
    println!("sent: {email}");
}

// 2. The function really wants ownership → call it with clone()
fn send_owned(email: String) {
    println!("sent (copy): {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        send(&email); // no copies
    }
    for _ in 0..3 {
        send_owned(email.clone()); // three copies of the string
    }
}
