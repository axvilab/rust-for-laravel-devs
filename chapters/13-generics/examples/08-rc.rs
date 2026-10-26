//! `Rc` — several owners in one thread; `Rc::clone` is just +1 to the counter

use std::rc::Rc;

#[derive(Debug)]
struct Config {
    db_url: String,
}

impl Config {
    fn load() -> Self {
        Config {
            db_url: "postgres://localhost/app".into(),
        }
    }
}

struct ServiceA {
    config: Rc<Config>,
}
struct ServiceB {
    config: Rc<Config>,
}

fn main() {
    let config = Rc::new(Config::load());
    let service_a = ServiceA {
        config: Rc::clone(&config),
    };
    let service_b = ServiceB {
        config: Rc::clone(&config),
    };
    println!("{}", Rc::strong_count(&config)); // 3
    println!("{} {}", service_a.config.db_url, service_b.config.db_url);
    drop(service_a);
    println!("{}", Rc::strong_count(&config)); // 2
}
