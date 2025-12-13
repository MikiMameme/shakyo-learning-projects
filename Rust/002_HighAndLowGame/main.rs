//数当てゲーム

use std::io;
use rand::Rng;

fn main(){
    println!("===数当てゲーム===");
    println!("１～１０の数字を当ててください");

    let answer = rand::rng().random_range(1..=10);
    let mut attempts = 0;

    loop{
        println!("数字を入力してください：");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("入力に失敗しました");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("エラー：数字を入力してください\n");
                continue;
            }
        };

        attempts += 1;

        if guess < answer {
            println!("もっと大きいです\n");
        } else if guess > answer{
            println!("もっと小さいです\n");
        } else {
            println!("正解です {}回で当たりました", attempts);
            break;
        }
    }
}