use std::{cmp::Ordering, io};

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

fn main() {
    let some_value: Option<u8> = Some(4u8);
    if let Some(3u8) = some_value {
    	println!("three!");
    } else {
    	println!("Other ... ");
    }

    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    // match coin {
    // 	Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    // 	_ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
    	println!("State is: {:?}", state);
    }
}
