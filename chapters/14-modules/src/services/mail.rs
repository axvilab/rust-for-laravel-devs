//! `services::mail`

pub struct Mailer;

impl Mailer {
    pub fn new() -> Self {
        Mailer
    }

    pub fn send(&self, to: &str, subject: &str) {
        // super:: — the parent module (services), like ../
        let _ = super::payment::is_payable(1);
        println!("email to {to}: '{subject}'");
    }
}

impl Default for Mailer {
    fn default() -> Self {
        Self::new()
    }
}
