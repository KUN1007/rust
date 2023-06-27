use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("azkhx"), 1007);
    scores.insert(String::from("kun"), 1007);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }
    println!("{:#?}", scores);

    let text = String::from("KUN KUN KUN IS CUTEST!");
    count_word(&text);

    println!("{:?}", text);
}

fn count_word(text: &String) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
