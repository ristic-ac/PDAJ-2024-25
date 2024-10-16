fn main() {
    let x = 5;
    println!("x: {}, address: {:p}", x, &x);

    let x = x + 1;
    println!("x: {}, address: {:p}", x, &x);

    {
        let x = 3;
        println!("x: {}, address: {:p}", x, &x);
        {
            let x = "hello";
            println!("x: {}, address: {:p}", x, &x);
        }
    }

    let x = x * 2;
    println!("x: {}, address: {:p}", x, &x);

    let x = "world";
    println!("x: {}, address: {:p}", x, &x);

    let mut x = 42;
    println!("x: {}, address: {:p}", x, &x);

    // x = "fail";
    // println!("x: {}, address: {:p}", x, &x);
}
