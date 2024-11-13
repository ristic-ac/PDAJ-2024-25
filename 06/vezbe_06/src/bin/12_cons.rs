#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    // Iterate over the list
    fn iterate_list(list: &List) {
        match list {
            List::Cons(value, next) => {
                println!("{value}");
                iterate_list(next);
            }
            List::Nil => (),
        }
    }
    iterate_list(&list);
    println!("{:?}",list)
}
