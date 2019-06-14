fn area(witdh: u32, height: u32) -> u32 {
	witdh * height
}

fn area2(rect: (u32, u32)) -> u32 {
	rect.0 * rect.1
}

fn main() {
    // let witdh1 = 30;
    // let height1 = 50;

    // println!("The area of the rectangle is {} square pixels.", area(witdh1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area2(rect1));

}