//! A hand-written `impl Default` when "zero" values don't fit

struct Config {
    host: String,
    port: u16,
    debug: bool,
    active: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            debug: false,
            active: true, // here it is, our `$active = true`
        }
    }
}

fn main() {
    let cfg = Config {
        port: 3000,
        ..Default::default()
    };
    println!(
        "{}:{} debug={} active={}",
        cfg.host, cfg.port, cfg.debug, cfg.active
    );
}
