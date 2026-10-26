//! `Box` для рекурсивних типів; `Vec` усередині — без Box

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>), // (2 + 3) — вираз усередині виразу
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b), // &Box<Expr> поводиться як &Expr
    }
}

// Vec уже тримає елементи в купі — Box не потрібен
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
        "Електроніка".into(),
        vec![
            Category("Ноутбуки".into(), vec![]),
            Category("Телефони".into(), vec![]),
        ],
    );
    println!("{} категорій у «{}»", count(&tree), tree.0); // 3
}
