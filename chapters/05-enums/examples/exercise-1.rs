//! Вправа 1: enum `PaymentMethod` з даними у варіантах

#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Card { last4: String },
    Crypto { wallet: String, network: String },
}

fn main() {
    let methods = [
        PaymentMethod::Cash,
        PaymentMethod::Card {
            last4: "4242".into(),
        },
        PaymentMethod::Crypto {
            wallet: "0xabc".into(),
            network: "TRC20".into(),
        },
    ];
    for m in &methods {
        println!("{m:?}");
    }
}
