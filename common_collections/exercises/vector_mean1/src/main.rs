use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> f64 {
    let mut m = 0.0;
    for e in v.iter() {
        m += f64::from(*e);
    }

    m/f64::from(v.len() as i32)
}

fn median(v: &Vec<i32>) -> i32 {
    v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> (i32, i32) {
    let occurr_count = occurrence_count(&v);

    let mut k: i32 = -1;
    let mut v: i32 = -1;
    for (key, val) in &occurr_count {
        if *val > v {
            v = *val;
            k = *key;
        }
    }

    (k, v)
}
fn occurrence_count(v: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for e in v.iter() {
        let count = map.entry(*e).or_insert(0);
        *count += 1;
    }
    map
}

fn main() {
    let mut nums = vec![4, 7, 3, 9, 2, 3];
    nums.sort();
    println!("Mean is: {}", mean(&nums));
    println!("{:?}", nums);

    println!("{:?}", median(&nums));

    println!("{:?}", occurrence_count(&nums));

    println!("{:?}", mode(&nums));
}
