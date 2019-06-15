fn main() {
	let len = String::from("Здравствуйте").len();
	// Number of bytes it takes to encode the given string.
	println!("{}", len);
	let hello = "Здравствуйте";

	//let s = &hello[0..1];
	//println!("{}", s);

	for c in "नमस्ते".chars() {
    	println!("{}", c);
	}

	for b in "नमस्ते".bytes() {
    	println!("{}", b);
	}
}
