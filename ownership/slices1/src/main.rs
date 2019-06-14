fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item ==  b' ' {
			return i;
		}
	}
	s.len()
}

fn first_word_slice(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}

fn main() {
    let s = String::from("hello");
    let len = s.len();
    // let slice = &s[3..len];
    let slice = &s[3..];
    println!("{}", slice);
}
