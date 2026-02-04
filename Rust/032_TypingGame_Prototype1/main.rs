use std::io;
use std::io::Write;
use std::time::Instant;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main(){
    println!("===  タイピングゲーム（プロトタイプ・コンソール版） ===");

    let content = fs::read_to_string("words.txt")
        .expect("エラー：words.txtが見つかりません");
    let mut words: Vec<&str> = content.lines().filter(|s| !s.trim().is_empty()).collect();

    println!("\n出題数を選んでください：");
    println!("１：すべて（１００％）");
    println!("２：半分（５０％）");
    println!("３：すこし（２５％）");
    print!("選択 >");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).ok();

    let now = Instant::now();
    words.sort_by_cached_key(|word| {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        now.elapsed().as_nanos().hash(&mut hasher);
        hasher.finish()
    });

    let limit = match choice.trim() {
        "2" => words.len() / 2,
        "3" => words.len() / 4,
        _ => words.len(),
    };

    let challenge_words = &words[..limit];

    println!("\n{}問用意しました", challenge_words.len());
    println!("エンターキーを押すとスタートします");
    let mut start_wait = String::new();
    io::stdin().read_line(&mut start_wait).ok();

    let start_time = Instant::now();
    let mut correct_count = 0;

    for (i, word) in challenge_words.iter().enumerate() {
        println!("\n\n第{}問 / {}問", i + 1, challenge_words.len());
        println!("===============================");
        println!("お題： {} ", word);
        println!("===============================");
        print!("入力 >");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        if input.trim() == *word {
            println!("✅正解");
            correct_count += 1;
        } else {
            println!("❌不正解（正解：{}）",word);
        }
    }

    let elapsed = start_time.elapsed();
    println!("================================");
    println!("正解発表");
    println!("正解数：{} / {} 問", correct_count, challenge_words.len());
    println!("かかった時間：{:.2} 秒", elapsed.as_secs_f64());
    println!("================================");

}