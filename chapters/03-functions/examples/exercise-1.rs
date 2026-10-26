//! Вправа 1: `stats` повертає (min, max, avg) кортежем

fn stats(values: &[f64]) -> (f64, f64, f64) {
    let mut min = values[0];
    let mut max = values[0];
    let mut sum = 0.0;
    for &v in values {
        min = min.min(v); // f64::min / f64::max — без if
        max = max.max(v);
        sum += v;
    }
    // len() — usize, тому явне `as f64` (розділ 1: неявних перетворень немає)
    (min, max, sum / values.len() as f64)
}

fn main() {
    let (min, max, avg) = stats(&[12.5, 3.0, 7.25, 20.0]);
    println!("min={min} max={max} avg={avg:.2}");
}
