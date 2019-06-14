struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn print_color(p: Point) {
	println!("({}, {}, {})", p.0, p.1, p.2);
}

fn main() {
    let black: Color = Color(1, 2, 3);
    println!("{}", black.1);

    let origin: Point = Point(0, 0, 0);
    let (x, y, z) = (origin.0, origin.1, origin.2);

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

}
