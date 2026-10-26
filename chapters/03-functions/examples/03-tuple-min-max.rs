//! Кілька значень з функції — кортеж; зріз `&[i32]` як параметр

fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &v in values {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }
    (min, max)
}

fn main() {
    let (lo, hi) = min_max(&[3, 9, 1, 7]);
    println!("{lo}..{hi}"); // 1..9

    let pair = min_max(&[5, -2]);
    println!("{} {}", pair.0, pair.1); // доступ за номером: -2 5
}
