struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		username,
		email,
		active: true,
		sign_in_count: 1,
	}
}

fn main() {

	let u1 = build_user(String::from("leo@leo.com"), String::from("leogtzr"));
	// Struct update syntax:
	let u2 = User {
		email: String::from("leogutierrezramirez@hola.com"),
		// username: String::from("leogtzr"),
		..u1
	};

	println!("{}", u2.username);

}
