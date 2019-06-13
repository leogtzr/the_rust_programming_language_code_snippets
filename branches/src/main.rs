fn main() {
	let number = 3;
	let result = if number < 5 {
		"ok"
	} else {
		":("
	};

	for x in 1..10 {
		if x % 2 == 0 {
			println!("{}", x);
		}
	}

	println!("{}", result);
	println!("\n\n");

	let mut y = 0;
	loop {
		if y > 10 {
			break;
		}
		println!("{}", y);
		y += 1;
	}
}
