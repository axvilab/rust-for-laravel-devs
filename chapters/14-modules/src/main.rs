//! The binary: all the logic is in the library (so it can be tested), this just starts it.
//! `cargo run -p ch14-modules`

fn main() {
    if let Err(e) = ch14_modules::run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
