use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 1);
    scores.insert(String::from("red"), 3);

    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    match scores.get(&String::from("blue")) {
        None => {},
        Some(score) => {
            println!("Score: {}", score);
        }
    }

    // Iterating:
    for (key, val) in &scores {
        println!("{}:{}", key, val);
    }

    // Inserting a value if the key has no value:
    scores.entry(&String::from("Yellow")).or_insert(&1001);

    let mut names: HashMap<String, i32> = HashMap::new();
    names.entry(String::from("Leo")).or_insert(28);

    println!("{:?}", names);

}
