//! Exercise 1: `stats` returns (min, max, avg) as a tuple

fn stats(values: &[f64]) -> (f64, f64, f64) {
    let mut min = values[0];
    let mut max = values[0];
    let mut sum = 0.0;
    for &v in values {
        min = min.min(v); // f64::min / f64::max — no if needed
        max = max.max(v);
        sum += v;
    }
    // len() is usize, hence the explicit `as f64` (chapter 1: no implicit conversions)
    (min, max, sum / values.len() as f64)
}

fn main() {
    let (min, max, avg) = stats(&[12.5, 3.0, 7.25, 20.0]);
    println!("min={min} max={max} avg={avg:.2}");
}
