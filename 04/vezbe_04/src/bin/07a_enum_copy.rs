// Derives the Copy, Clone, and Debug traits for the enum automatically.
// This works because:
// 1. `Direction` has no data that isn't `Copy` (e.g., references, `String`, or `Vec`).
// 2. All enum variants are simple and contain no additional fields.
#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;         // `direction` is created
    let copy_of_direction = direction;        // `direction` is copied to `copy_of_direction`

    println!("{:?}", direction);              // Outputs: North
    println!("{:?}", copy_of_direction);      // Outputs: North
}
