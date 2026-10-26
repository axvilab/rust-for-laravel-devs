//! Data (`struct`), behavior (`impl`) and contract (`trait`) — three separate blocks

// 1. Data
struct Invoice {
    id: u64,
    total: f64,
    paid: bool,
}

// 2. Behavior specifically for Invoice
impl Invoice {
    fn mark_paid(&mut self) {
        self.paid = true;
    }
}

// 3. Contract: "anything that can be sent by email"
trait Mailable {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

// 2 again: Invoice fulfils the contract
impl Mailable for Invoice {
    fn subject(&self) -> String {
        format!("Invoice #{}", self.id)
    }
    fn body(&self) -> String {
        format!("Amount due: {} UAH", self.total)
    }
}

fn main() {
    let mut inv = Invoice {
        id: 7,
        total: 1500.0,
        paid: false,
    };
    println!("{} — {}", inv.subject(), inv.body());
    inv.mark_paid();
    println!("paid: {}", inv.paid);
}
