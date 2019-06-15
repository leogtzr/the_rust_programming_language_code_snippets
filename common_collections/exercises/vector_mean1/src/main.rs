fn mean(v: &Vec<i32>) -> f64 {
    let mut m = 0.0;
    for e in v.iter() {
        m += f64::from(*e);
    }

    m/f64::from(v.len() as i32)
}

fn main() {
    let nums = vec![4, 7, 3, 9];
    println!("Mean is: {}", mean(&nums));
    println!("{:?}", nums);
}
