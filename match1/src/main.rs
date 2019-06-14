#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from: {:?}!", state);
			25
		},
	}
}

fn main() {
    let c1: Coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(c1);

    let mut x: Option<i32> = Some(3);
    x = plus_one(x);

    println!("{:?}", x);

    let y: Option<char> = Some('@');

    match y {
    	None => {},
    	Some(c) => {
    		println!("Value is: {}", c);
    	}
    }

    match y {
    	None => {},
    	Some(_) => {
    		println!("It is not empty.");
    	}
    }

    let some_u8_value = 0u8;
    match some_u8_value {
    	1 => println!("one!"),
    	_ => println!("Other!"),
    }
}
