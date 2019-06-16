// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32>  {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2).sqrt())
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);

    //println!("{}", result);

    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};

    let ps: Point<i32> = Point{x: 5, y: 34};
    println!("{}", ps.x());

    let point1 = Point2{x: 5, y: 10.4};
    let point2 = Point2{x: "Hello", y: 'c'};

    let point3 = point1.mixup(point2);
    println!("p3.x = {}, p3.y = {}", point3.x, point3.y);

}
