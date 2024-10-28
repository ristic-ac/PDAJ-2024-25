// This does not compile, because the fields of the struct are references.
// Resolve this by using the `String` type instead of `&str`
// Or by using the `lifetime` syntax.
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}