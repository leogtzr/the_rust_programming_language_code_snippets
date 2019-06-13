fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
    	println!("The value is: {}", element);
    }

    println!("a.len() = {}", a.len());

    for i in 0..a.len() {
    	println!("e = {}", a[i]);
    }

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>");

    for number in (1..8).rev() {
    	println!("{}!", number);
    }
}

