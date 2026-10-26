//! Щоденні методи Option: is_some, map, unwrap_or, unwrap, expect, and_then

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
        name: "Віталій".into(),
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

    // перевірки
    if maybe.is_some() {
        println!("є");
    }
    if maybe.is_none() {
        println!("нема");
    }

    // значення або дефолт; map «з'їдає» maybe — після цього рядка його не використати
    let name = maybe.map(|u| u.name).unwrap_or("Гість".to_string());
    println!("{name}");

    // значення або паніка (аварійна зупинка). Тільки в прототипах і тестах!
    let user = find_user(1).unwrap();
    println!("{}", user.id);

    // паніка з поясненням — краще за unwrap, якщо вже панікуєш
    let user = find_user(1).expect("користувач 1 має існувати після сідера");
    println!("{}", user.email);

    // перетворити значення всередині, не розпаковуючи
    let email_len: Option<usize> = find_user(1).map(|u| u.email.len());
    println!("{email_len:?}"); // Some(13)

    // ланцюжок: якщо є користувач — знайти його компанію
    // (find_company теж повертає Option)
    let company = find_user(1).and_then(|u| find_company(u.company_id));
    println!("{:?}", company.map(|c| c.name)); // Some("AxViLab")
}
