// We derive the Debug trait for User struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Implement the Display trait for User because it is not derivable
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

// Implementing methods for User struct
impl User {
    // Constructor
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Getter method
    fn get_email(&self) -> &String {
        &self.email
    }

    // Setter method
    fn set_email(&mut self, email: String) {
        self.email = email;
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

    // Display and Debug
    println!("User : {}", user1);
    println!("User : {:?}", user1);

    // Using the constructor
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

    // Display both user1 and user2
    // println!("User : {}", user1);  // This will not work as user1 has been moved to user2
    println!("User : {}", user2);

    let mut user3 = User::build_user(String::from("third@example.com"), String::from("thirdusername123"));
    user3.set_email(String::from("fourth@example.com"));
    println!("User email : {}", user3.get_email());
}