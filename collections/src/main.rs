use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count is now a pointer to the value stored by word
        *count += 1;
    }
    println!("{:?}", map);
}
