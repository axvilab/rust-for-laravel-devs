//! Немає дефолтних аргументів: необов'язкове передається як `Option`

fn paginate(page: u32, per_page: Option<u32>) -> (u32, u32) {
    let per_page = per_page.unwrap_or(15); // None → 15
    (page, per_page)
}

fn main() {
    println!("{:?}", paginate(1, None)); // (1, 15)
    println!("{:?}", paginate(2, Some(50))); // (2, 50)
}
