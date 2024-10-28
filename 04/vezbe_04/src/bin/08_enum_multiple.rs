// Enums can have multiple types of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implement a method for the enum
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
        }
    }
}
fn main() {

    let m = Message::Write(String::from("hello"));
    m.call();
    m.call();

    let m = Message::Move { x: 1, y: 2 };
    m.call();
    m.call();

    let m = Message::ChangeColor(255, 0, 0);
    m.call();
    m.call();

    let m = Message::Quit;
    m.call();
    m.call();
}