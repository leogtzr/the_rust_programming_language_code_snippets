fn main() {
    let s1 = gives_ownership();	// gives_ownership moves its return value into s1
    let s2 = String::from("hello");			// s2 comes into scope

    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
	let some_string	= String::from("hello");
	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}