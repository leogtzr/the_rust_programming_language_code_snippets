#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {

		}

		fn seat_at_table() {

		}
	}

	mod serving {
		fn take_order() {

		}

		fn serve_order() {

		}

		fn take_payment() {

		}
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	// crate::front_of_house::hosting::add_to_waitlist();

	// // Relative path
	// front_of_house::hosting::add_to_waitlist();

	let mut meal = back_of_house::Breakfast::summer("rye");
	meal.toast = String::from("wheat");

	println!("I'd like {} toast please", meal.toast);

	// The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {

}

mod back_of_house {

	pub enum Appetizer {
		// Always public:
		Soup,
		Salad,
	}

	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,					// private
	}

	impl Breakfast {
		// Need it since Breakfast has a private field.
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}
}