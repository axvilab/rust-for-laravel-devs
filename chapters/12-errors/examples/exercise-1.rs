//! Вправа 1: `parse_age` — не число або > 150 → `Err`

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s.trim().parse().map_err(|_| format!("«{s}» — не вік"))?; // u8 сам відсікає від'ємні й > 255
    if n > 150 {
        return Err(format!("{n} — забагато"));
    }
    Ok(n)
}

fn main() {
    for s in ["35", "abc", "200", "-1"] {
        println!("{s:>4} → {:?}", parse_age(s));
    }
}
