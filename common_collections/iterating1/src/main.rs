fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
    	println!("{}", i);
    	//*i = 0;
    }

    for i in &mut v {
    	println!("{}", i);
    	// Change every element of the vector:
    	*i += *i * 10;
    }

    println!("{:?}", v);
}
