//! `Box` for recursive types; a `Vec` inside — no Box needed

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>), // (2 + 3) — an expression inside an expression
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b), // &Box<Expr> behaves like &Expr
    }
}

// Vec already keeps its elements on the heap — no Box needed
struct Category(String, Vec<Category>);

fn count(c: &Category) -> usize {
    1 + c.1.iter().map(count).sum::<usize>()
}

fn main() {
    let e = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Add(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
    );
    println!("{}", eval(&e)); // 9

    let tree = Category(
        "Electronics".into(),
        vec![
            Category("Laptops".into(), vec![]),
            Category("Phones".into(), vec![]),
        ],
    );
    println!("{} categories in '{}'", count(&tree), tree.0); // 3
}
