//! Розділ 7. Трейти — інтерфейси, які вміють більше
//!
//! Увесь код розділу — у `examples/` (запуск: `cargo run -p ch07-traits --example <назва>`).
//! Приклади, які навмисно не компілюються, — у `broken/`.

use std::fmt;

/// Гроші в копійках — той самий `Money`, що в прикладах розділу.
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
        // знак окремо: `%` зберігає знак діленого, і без abs() -150 дало б "-1.-50"
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
