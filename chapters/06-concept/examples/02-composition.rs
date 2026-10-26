//! Composition instead of inheritance: the shared part is a separate struct

#[derive(Debug)]
struct Timestamps {
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    timestamps: Timestamps,
}

#[derive(Debug)]
struct Order {
    id: u64,
    total: f64,
    timestamps: Timestamps,
}

fn main() {
    let u = User {
        id: 1,
        name: "Vitalii".into(),
        timestamps: Timestamps {
            created_at: 1_700_000_000,
            updated_at: 1_700_000_500,
        },
    };
    let o = Order {
        id: 10,
        total: 1500.0,
        timestamps: Timestamps {
            created_at: 1_700_001_000,
            updated_at: 1_700_001_000,
        },
    };
    println!("{u:#?}\n{o:#?}");
}
