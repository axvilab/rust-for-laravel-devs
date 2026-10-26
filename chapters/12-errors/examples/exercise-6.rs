//! Вправа 6: `parse_ids` — `split`, `map`, `collect()` у `Result<Vec<_>, _>`

fn parse_ids(raw: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    raw.split(',').map(|s| s.trim().parse::<u64>()).collect()
}

fn main() {
    println!("{:?}", parse_ids("1,2,3")); // Ok([1, 2, 3])
    println!("{:?}", parse_ids("1,x,3")); // Err(ParseIntError { kind: InvalidDigit })
}
