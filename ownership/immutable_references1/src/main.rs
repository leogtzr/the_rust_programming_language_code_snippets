fn main() {
    let mut s = String::from("hello");
    let r1 = &s;			// OK
    let r2 = &s;			// OK
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r1);
}
