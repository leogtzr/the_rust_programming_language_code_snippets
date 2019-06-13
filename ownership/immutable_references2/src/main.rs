fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point, so it is safe to do this:

    let r3 = &mut s;
    r3.push_str("Ok ... ");

    println!("{}", r3);
}