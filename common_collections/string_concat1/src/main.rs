fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(", World!");
    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("{}", s2);

    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");

    //let val = a + "-" + &b + "-" + &c;
    //println!("{}", val);

    let x = format!("{}-{}-{}", a, b, c);
    println!("{}", x);

    println!("{}", a);
}
