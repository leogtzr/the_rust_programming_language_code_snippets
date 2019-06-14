fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area2(rect: (u32, u32)) -> u32 {
	rect.0 * rect.1
}

fn area3(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

// Associated functions
impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle {
			height: size, width: size
		}
	}
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area2(rect1));

    let rect2 = Rectangle  {
    	width: 30,
    	height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("{:#?}", rect2);

    println!("~~~~~~~~~~~~~~~~~~~~~~>");

    println!("The area is: {}", rect2.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let r4: Rectangle = Rectangle::square(56);
    println!("{:#?}", r4);

}
