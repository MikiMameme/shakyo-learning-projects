use std::collections::HashMap;
use std::fs;

fn main() {
    println!("単語カウンター");
    println!("count.txtを読み込みます");

    let text = fs::read_to_string("count.txt")
        .expect("count.txtを読み込むことができませんでした");

    let mut map: HashMap<String, i32> = HashMap::new();

    for word in text.split(|c: char| !c.is_alphabetic()) {
        if word.is_empty() {
            continue;
        }
        let key = word.to_lowercase();
        let count = map.entry(key).or_insert(0);
        *count += 1;
    }
    println!("===集計結果===");
    for (word, count) in & map {
        println!("{} : {}", word, count);
    }
}