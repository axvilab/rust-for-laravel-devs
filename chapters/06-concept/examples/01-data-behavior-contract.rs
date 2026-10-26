//! Дані (`struct`), поведінка (`impl`) і контракт (`trait`) — три окремі блоки

// 1. Дані
struct Invoice {
    id: u64,
    total: f64,
    paid: bool,
}

// 2. Поведінка саме для Invoice
impl Invoice {
    fn mark_paid(&mut self) {
        self.paid = true;
    }
}

// 3. Контракт: «усе, що можна відправити email-ом»
trait Mailable {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

// 2 знову: Invoice виконує контракт
impl Mailable for Invoice {
    fn subject(&self) -> String {
        format!("Рахунок №{}", self.id)
    }
    fn body(&self) -> String {
        format!("До сплати: {} грн", self.total)
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
    println!("оплачено: {}", inv.paid);
}
