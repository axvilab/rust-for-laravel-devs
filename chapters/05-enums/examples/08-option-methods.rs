//! Everyday Option methods: is_some, map, unwrap_or, unwrap, expect, and_then

struct User {
    id: u64,
    name: String,
    email: String,
    company_id: u64,
}

struct Company {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    (id == 1).then(|| User {
        id: 1,
        name: "Vitalii".into(),
        email: "v@example.com".into(),
        company_id: 10,
    })
}

fn find_company(id: u64) -> Option<Company> {
    (id == 10).then(|| Company {
        name: "AxViLab".into(),
    })
}

fn main() {
    let maybe = find_user(5);

    // checks
    if maybe.is_some() {
        println!("present");
    }
    if maybe.is_none() {
        println!("absent");
    }

    // value or default; map "eats" maybe — it can't be used after this line
    let name = maybe.map(|u| u.name).unwrap_or("Guest".to_string());
    println!("{name}");

    // value or panic (an emergency stop). Only in prototypes and tests!
    let user = find_user(1).unwrap();
    println!("{}", user.id);

    // a panic with an explanation — better than unwrap if you panic anyway
    let user = find_user(1).expect("user 1 must exist after seeding");
    println!("{}", user.email);

    // transform the value inside without unwrapping
    let email_len: Option<usize> = find_user(1).map(|u| u.email.len());
    println!("{email_len:?}"); // Some(13)

    // a chain: if there's a user, find their company
    // (find_company also returns an Option)
    let company = find_user(1).and_then(|u| find_company(u.company_id));
    println!("{:?}", company.map(|c| c.name)); // Some("AxViLab")
}
