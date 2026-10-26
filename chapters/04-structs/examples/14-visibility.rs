//! `pub` і приватні поля: ззовні модуля — тільки через конструктор

mod users {
    pub struct User {
        pub id: u64,
        pub name: String,
        password_hash: String, // без pub — ззовні модуля не видно
    }

    impl User {
        pub fn new(id: u64, name: &str, password: &str) -> Self {
            // публічний
            Self {
                id,
                name: name.to_string(),
                password_hash: Self::hash_password(password),
            }
        }

        pub fn check(&self, password: &str) -> bool {
            self.password_hash == Self::hash_password(password)
        }

        fn hash_password(password: &str) -> String {
            // приватний helper (НЕ справжнє хешування — лише для прикладу!)
            password.chars().rev().collect()
        }
    }
}

fn main() {
    let u = users::User::new(1, "Віталій", "secret");
    println!("{} {} {}", u.id, u.name, u.check("secret"));
    // println!("{}", u.password_hash);  // ПОМИЛКА E0616: field is private
    // users::User { id: 2, name: "x".into(), password_hash: "y".into() };
    //                                       // ПОМИЛКА E0451: private field
}
