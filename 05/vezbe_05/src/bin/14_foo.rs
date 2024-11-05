// Source: https://stackoverflow.com/questions/29861388/when-is-it-useful-to-define-multiple-lifetimes-in-a-struct
static ZERO: i32 = 0;

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        return x
    } else {
        return &ZERO
    }
}

fn main() {
    let x = 1;
    let v;
    {
        let y = 2;
        let f = Foo { x: &x, y: &y };
        v = get_x_or_zero_ref(&f.x, &f.y);
    }
    println!("{}", *v);
}