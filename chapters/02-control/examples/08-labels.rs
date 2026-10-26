//! Мітки циклів: `break 'outer` — це `break 2` у PHP

fn main() {
    'outer: for x in 0..10 {
        for y in 0..10 {
            if x * y > 50 {
                println!("зупинились на x={x}, y={y}");
                break 'outer;
            }
        }
    }
}
