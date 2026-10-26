//! Три рецепти для `'static` у `tokio::spawn`: move, clone, Arc

use std::sync::Arc;

#[derive(Clone)]
struct Mailer {
    from: Arc<str>, // сервіс із Arc усередині — дешевий Clone
}

impl Mailer {
    async fn send(&self, to: &str) -> String {
        format!("{} → {to}", self.from)
    }

    // &self у фоновій задачі позичено — тому в задачу йде self.clone()
    fn send_in_background(&self, to: String) -> tokio::task::JoinHandle<String> {
        let me = self.clone();
        tokio::spawn(async move { me.send(&to).await })
    }
}

#[tokio::main]
async fn main() {
    // 1. значення потрібне тільки задачі — async move
    let report = String::from("звіт за день");
    let a = tokio::spawn(async move { report.len() });

    // 2. потрібне і нам, і задачі, і воно дешеве — clone перед move
    let email = String::from("v@example.com");
    let email2 = email.clone();
    let b = tokio::spawn(async move { format!("лист на {email2}") });

    // 3. велике або спільне — Arc::clone перед move
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
