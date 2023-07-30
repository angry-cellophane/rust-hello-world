use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    value: String,
}

fn calculate_uniq_words(s: &str) -> usize {
    let mut m = HashMap::new();
    for w in s.split_whitespace() {
        let count = m.entry(w).or_insert(0);
        *count += 1;
    }
    m.len()
}

fn main() {
    let s = "hello world hello again no again";
    let words = calculate_uniq_words(s);
    println!("count = {}", words);
}
