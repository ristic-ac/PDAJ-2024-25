#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Implement the Display trait for User
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {} {} {} {}", self.active, self.username, self.email, self.sign_in_count)
    }
}

// Debug trait for User has the default implementation (derive)
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // In this regular string:
//             // Each " inside the string must be escaped as \".
//             // Each \ must be escaped as \\. This is because \ is an escape character in Rust strings.
//         write!(f, r#"{{"active": {}, "username": "{}", "email": "{}", "sign_in_count": {}}}"#, self.active, self.username, self.email, self.sign_in_count)
//     }
// }

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    fn get_email(&self) -> &String {
        &self.email
    }
}


fn main() {
    // Slide 3
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User : {}", user1);
    println!("User : {:?}", user1);

    let user1 = User::build_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("User : {}", user1);
    println!("User : {:?}", user1);

    // Slide 4
    let user2 = User {
        email: user1.email,
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("User : {}", user2);
    // println!("User : {}", user1);

    let mut user3 = User::build_user(String::from("third@example.com"), String::from("thirdusername123"));
    user3.email = String::from("changed@now.com");
    println!("User : {:?}", user3);
}