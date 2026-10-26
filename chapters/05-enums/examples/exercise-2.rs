//! Вправа 2: комісія за способом оплати через `match`

enum PaymentMethod {
    Cash,
    Card { last4: String },
    Crypto { wallet: String, network: String },
}

fn fee(method: &PaymentMethod, amount: f64) -> f64 {
    match method {
        PaymentMethod::Cash => 0.0,
        PaymentMethod::Card { .. } => amount * 0.02, // дані картки тут не потрібні
        PaymentMethod::Crypto { .. } => amount * 0.01 + 10.0,
    }
}

fn main() {
    let card = PaymentMethod::Card {
        last4: "4242".into(),
    };
    let crypto = PaymentMethod::Crypto {
        wallet: "0xabc".into(),
        network: "TRC20".into(),
    };
    println!("{}", fee(&PaymentMethod::Cash, 1000.0)); // 0
    println!("{}", fee(&card, 1000.0)); // 20
    println!("{}", fee(&crypto, 1000.0)); // 20
}
