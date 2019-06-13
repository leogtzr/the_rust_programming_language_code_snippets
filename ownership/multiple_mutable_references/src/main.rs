fn main() {

	let mut s = String::from("hello");
	{
		let r1 = &mut s;
		r1.push_str(", world");
	}
	s.push_str("alv");

	println!("{}", s);

}
