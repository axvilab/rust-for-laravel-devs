//! `services::mail`

pub struct Mailer;

impl Mailer {
    pub fn new() -> Self {
        Mailer
    }

    pub fn send(&self, to: &str, subject: &str) {
        // super:: — батьківський модуль (services), як ../
        let _ = super::payment::is_payable(1);
        println!("лист на {to}: «{subject}»");
    }
}

impl Default for Mailer {
    fn default() -> Self {
        Self::new()
    }
}
