// This structure represents a color in the RGB color space. 
struct Color(i32, i32, i32);
// This structure represents a point in the 3D space.
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}