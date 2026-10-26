//! A generic struct `Paginated<T>` and methods only for `T: Clone`

#[derive(Debug, Clone)]
struct User {
    name: String,
}

#[derive(Debug, Clone)]
struct Order {
    id: u64,
}

struct Paginated<T> {
    items: Vec<T>,
    page: u32,
    total: u64,
}

impl<T> Paginated<T> {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Clone> Paginated<T> {
    // the method exists only if T: Clone
    fn first_cloned(&self) -> Option<T> {
        self.items.first().cloned()
    }
}

// Instead of the book's `repo.list(1)` — a simple function that "slices" a page.
fn list<T: Clone>(all: &[T], page: u32) -> Paginated<T> {
    let per_page = 2;
    let start = ((page - 1) * per_page) as usize;
    Paginated {
        items: all
            .iter()
            .skip(start)
            .take(per_page as usize)
            .cloned()
            .collect(),
        page,
        total: all.len() as u64,
    }
}

fn main() {
    let all_users = [
        User {
            name: "Ivan".into(),
        },
        User {
            name: "Maria".into(),
        },
        User {
            name: "Oleh".into(),
        },
    ];
    let all_orders = [Order { id: 1 }];

    let users: Paginated<User> = list(&all_users, 1);
    let orders: Paginated<Order> = list(&all_orders, 2);

    println!(
        "page {} of {} users, first: {:?}",
        users.page,
        users.total,
        users.first_cloned()
    );
    println!(
        "orders, page {}: empty = {}",
        orders.page,
        orders.is_empty()
    );
}
