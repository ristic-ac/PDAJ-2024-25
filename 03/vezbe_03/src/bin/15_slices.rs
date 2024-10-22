fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    analyze_slice(slice);
}

fn analyze_slice(slice: &[i32]) {
    println!("First element: {}", slice[0]);
}
