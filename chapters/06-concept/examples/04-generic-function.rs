//! A generic function `send<T: Mailable>` — like `function send(Mailable $item)`

trait Mailable {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}

struct Invoice {
    id: u64,
    total: f64,
}

struct Newsletter {
    title: String,
}

impl Mailable for Invoice {
    fn subject(&self) -> String {
        format!("Invoice #{}", self.id)
    }
    fn body(&self) -> String {
        format!("Amount due: {} UAH", self.total)
    }
}

impl Mailable for Newsletter {
    fn subject(&self) -> String {
        self.title.clone()
    }
    fn body(&self) -> String {
        "News of the week".to_string()
    }
}

fn send<T: Mailable>(item: &T) {
    println!("{}: {}", item.subject(), item.body());
}

fn main() {
    let invoice = Invoice {
        id: 1,
        total: 1500.0,
    };
    let newsletter = Newsletter {
        title: "Digest".into(),
    };
    send(&invoice);
    send(&newsletter);
}
