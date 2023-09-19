use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}", scores);

    let score = scores.get(&String::from("blue")).copied().unwrap_or_default();
    println!("score for blue: {score}");

    for (k, v) in &scores {
        println!("key: {k}, value: {v}");
    }

    scores.insert(String::from("blue"), 11);
    println!("scores: {:?}", scores);

    scores.entry(String::from("red")).or_insert(100);
    scores.entry(String::from("red")).or_insert(200);
    println!("scores: {:?}", scores);
}