//! `collect()` into `Result<Vec<_>, _>` stops at the first error; `partition` collects everything

fn main() {
    let ok: Result<Vec<u32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<u32>()).collect();
    println!("{ok:?}"); // Ok([1, 2, 3])

    let bad: Result<Vec<u32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<u32>()).collect();
    println!("{bad:?}"); // Err(ParseIntError { kind: InvalidDigit })

    // An import report: good and bad separately
    let (good, failed): (Vec<_>, Vec<_>) = ["1", "x", "3", "y"]
        .iter()
        .map(|s| s.parse::<u32>())
        .partition(|r| r.is_ok());
    let good: Vec<u32> = good.into_iter().map(Result::unwrap).collect();
    println!("ok: {good:?}, errors: {}", failed.len());
}
