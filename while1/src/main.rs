fn main() {
    let mut number = 5;
    while number > 1 {
    	println!("{}", number);
    	number -= 1;
    }

    println!("................................");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
    	println!("The value is: {}", a[index]);
    	index += 1;
    }
}