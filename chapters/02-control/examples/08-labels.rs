//! Loop labels: `break 'outer` is PHP's `break 2`

fn main() {
    'outer: for x in 0..10 {
        for y in 0..10 {
            if x * y > 50 {
                println!("stopped at x={x}, y={y}");
                break 'outer;
            }
        }
    }
}
