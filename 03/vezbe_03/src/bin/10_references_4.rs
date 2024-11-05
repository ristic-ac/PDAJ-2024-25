fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];


    let mut num: &mut i32 = &mut v[2];
    // let num: &mut i32 = &mut v[2];

    num = &mut v[1];

    *num += 1;

    num = &mut v[2];

    
    println!("Third element is {}", *num);




    println!("Vector is now {:?}", v);
}