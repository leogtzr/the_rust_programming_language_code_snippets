fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let total: i32 = v2.iter().sum();

    println!("{}\n\n", total);

    let v3: Vec<i32> = vec![5, 4, 3, 2, 1];
    // for value in v3.iter().map(|x| x + 1) {
    //     println!("{}", value);
    // }
    let v4: Vec<i32> = v3.iter().map(|x| x + 1).collect();
    println!("{:?}", v4);

    let v5: Vec<bool> = vec![1, 2, 3, 4, 5].iter().map(|x| x % 2 == 0).collect();
    println!("{:?}", v5);

    // let v6: Vec<_> = vec![23, 54, 56, 23].iter().filter(|x: i32| {x % 2 == 0}).collect();
    // println!("{:?}", v6);
    let v6: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let v7: Vec<i32> = v6.iter().filter(|x| *x % 2 == 0).map(|x| *x).collect();

    println!("{:?}", v7);
}
