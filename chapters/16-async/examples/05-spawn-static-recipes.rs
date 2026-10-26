//! Three recipes for `'static` in `tokio::spawn`: move, clone, Arc

use std::sync::Arc;

#[derive(Clone)]
struct Mailer {
    from: Arc<str>, // a service with an Arc inside — a cheap Clone
}

impl Mailer {
    async fn send(&self, to: &str) -> String {
        format!("{} → {to}", self.from)
    }

    // &self is borrowed in a background task — so the task gets self.clone()
    fn send_in_background(&self, to: String) -> tokio::task::JoinHandle<String> {
        let me = self.clone();
        tokio::spawn(async move { me.send(&to).await })
    }
}

#[tokio::main]
async fn main() {
    // 1. only the task needs the value — async move
    let report = String::from("daily report");
    let a = tokio::spawn(async move { report.len() });

    // 2. both we and the task need it, and it's cheap — clone before move
    let email = String::from("v@example.com");
    let email2 = email.clone();
    let b = tokio::spawn(async move { format!("email to {email2}") });

    // 3. big or shared — Arc::clone before move
    let config = Arc::new(vec![0u8; 1024]);
    let cfg = Arc::clone(&config);
    let c = tokio::spawn(async move { cfg.len() });

    let mailer = Mailer {
        from: Arc::from("noreply@example.com"),
    };
    let d = mailer.send_in_background(email.clone());

    println!(
        "{} {} {} {} | {email} {}",
        a.await.unwrap(),
        b.await.unwrap(),
        c.await.unwrap(),
        d.await.unwrap(),
        config.len()
    );
}
