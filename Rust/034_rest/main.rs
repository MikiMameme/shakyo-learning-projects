use std::thread;
use std::time::Duration;

fn main(){
    println!("MacBook「ふう、熱いよ🥵」");

    for i in 1..=10 {
        println!("{}回目の深呼吸中...", i);

        thread::sleep(Duration::from_secs(1));
    }

    println!("🧊ちゃんと冷えたかな？お疲れ様");
}