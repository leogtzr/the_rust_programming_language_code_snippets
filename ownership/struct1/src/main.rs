struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}

fn main() {
    let mut user1 = User {
    	email: String::from("leo@leo.com"),
    	username: String::from("Leo"),
    	active: true,
    	sign_in_count: 1,
    };

    // println!("{}", user1);
    user1.username = String::from("leogtzr");

    println!("{}", user1.username);

    let mut user2 = build_user(String::from("hello"), String::from("leogtzr"));
    user2.active = false;
    println!("{}", user2.email);
    println!("{}", user2.active);
}
