trait Summary {
    fn summarize(&self) -> String;

    // Default implementation:
    fn hello(&self) {

    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }
}

fn main() {
    let p = Point{x: 3, y: 7};
    println!("{}", p.summarize());
    p.hello();
}