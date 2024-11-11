fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    move ||  s_ref.to_string()
}
     
fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}

fn main() {
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    drop(s_own);
    cloner();
}