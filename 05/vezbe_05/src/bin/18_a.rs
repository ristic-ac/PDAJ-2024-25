use std::fs::File;

// The unwrap method is a shortcut method implemented just like the match expression. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}