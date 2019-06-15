use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 1);
    scores.insert(String::from("red"), 3);

    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores: Vec<i32> = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

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

}
