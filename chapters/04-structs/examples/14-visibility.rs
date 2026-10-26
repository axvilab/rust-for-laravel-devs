//! `pub` and private fields: from outside the module — only through the constructor

mod users {
    pub struct User {
        pub id: u64,
        pub name: String,
        password_hash: String, // no pub — invisible outside the module
    }

    impl User {
        pub fn new(id: u64, name: &str, password: &str) -> Self {
            // public
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
            // a private helper (NOT real hashing — just for the example!)
            password.chars().rev().collect()
        }
    }
}

fn main() {
    let u = users::User::new(1, "Vitalii", "secret");
    println!("{} {} {}", u.id, u.name, u.check("secret"));
    // println!("{}", u.password_hash);  // ERROR E0616: field is private
    // users::User { id: 2, name: "x".into(), password_hash: "y".into() };
    //                                       // ERROR E0451: private field
}
