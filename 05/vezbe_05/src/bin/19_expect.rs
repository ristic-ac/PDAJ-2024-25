use std::fs::File;

// The expect method is similar to the unwrap method. The difference is that expect allows us to specify the panic! message.
fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}