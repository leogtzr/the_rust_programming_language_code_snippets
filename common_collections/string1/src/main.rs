fn main() {
    let mut s = String::new();
    s.push_str("asd");

    // let s2: String = s.to_string();
    let data = "initial contents";
    let s2: String = data.to_string();

    let s3 = "Leonardo".to_string();

    let s4: String = String::from("Leo Gtz Ram");
    println!("{}", s4);

    let hello = String::from("Hello!");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);

    let mut name: String = String::from("Leo");
    // name.push_str("Gtz");

    let last_name: &str = "Gtz";
    name.push_str(last_name);

    println!("{}", name);
}