//! Exercise 1: `parse_age` — not a number or > 150 → `Err`

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s
        .trim()
        .parse()
        .map_err(|_| format!("'{s}' is not an age"))?; // u8 itself rejects negatives and > 255
    if n > 150 {
        return Err(format!("{n} is too much"));
    }
    Ok(n)
}

fn main() {
    for s in ["35", "abc", "200", "-1"] {
        println!("{s:>4} → {:?}", parse_age(s));
    }
}
