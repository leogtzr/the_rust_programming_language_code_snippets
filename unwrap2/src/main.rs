use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Vale shit la vida");
}
