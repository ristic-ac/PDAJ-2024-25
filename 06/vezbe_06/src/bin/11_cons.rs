enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
}
