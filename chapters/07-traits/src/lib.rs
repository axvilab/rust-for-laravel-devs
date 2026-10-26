//! Chapter 7. Traits — interfaces that can do more
//!
//! All code for the chapter is in `examples/` (run: `cargo run -p ch07-traits --example <name>`).
//! Examples that intentionally do not compile are in `broken/`.

use std::fmt;

/// Money in kopecks — the same `Money` as in the chapter examples.
///
/// ```
/// use ch07_traits::Money;
///
/// assert_eq!(Money::uah(150050).to_string(), "1500.50 UAH");
/// assert_eq!(Money::uah(-150).to_string(), "-1.50 UAH");
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

impl Money {
    pub fn uah(amount: i64) -> Self {
        Money {
            amount,
            currency: "UAH".to_string(),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // the sign goes separately: `%` keeps the dividend's sign, and without abs() -150 would give "-1.-50"
        let sign = if self.amount < 0 { "-" } else { "" };
        let abs = self.amount.abs();
        write!(f, "{sign}{}.{:02} {}", abs / 100, abs % 100, self.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_positive_zero_and_negative_amounts() {
        assert_eq!(Money::uah(150050).to_string(), "1500.50 UAH");
        assert_eq!(Money::uah(7).to_string(), "0.07 UAH");
        assert_eq!(Money::uah(0).to_string(), "0.00 UAH");
        assert_eq!(Money::uah(-150).to_string(), "-1.50 UAH");
        assert_eq!(Money::uah(-5).to_string(), "-0.05 UAH");
    }
}
