//! Calculations. `pub(crate)` — visible in the crate, but not to users of the library.

use crate::models::Order;

pub fn subtotal(order: &Order) -> u64 {
    order
        .lines
        .iter()
        .map(|(p, qty)| p.price * u64::from(*qty))
        .sum()
}

pub fn total_with_fee(order: &Order) -> u64 {
    let sub = subtotal(order);
    sub + crate::calc_fee(sub)
}

pub fn format_uah(kopecks: u64) -> String {
    format!("{}.{:02} UAH", kopecks / 100, kopecks % 100)
}

/// An internal helper: `pub(crate)` is like `@internal`, but the compiler checks it.
pub(crate) fn round_to_hryvnia(kopecks: u64) -> u64 {
    (kopecks + 50) / 100 * 100
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Order, Product};

    #[test]
    fn total_includes_two_percent_fee() {
        let order = Order::new(1, vec![(Product::new(1, "Mouse", 1000), 2)]).unwrap();
        assert_eq!(subtotal(&order), 2000);
        assert_eq!(total_with_fee(&order), 2040);
    }

    #[test]
    fn rounding_is_crate_internal() {
        assert_eq!(round_to_hryvnia(12_349), 12_300);
        assert_eq!(round_to_hryvnia(12_350), 12_400);
    }
}
