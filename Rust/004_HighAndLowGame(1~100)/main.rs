//数当てゲーム（１〜１００）

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("=====数当てゲーム=====");

    //前回→１〜１０、今回→１〜１００に改良
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    println!("\n１から１００の数を当ててください");

    loop{
        println!("\n予想を入力してください");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("入力の読み取りに失敗しました");

        //エラー処理
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("数値を入力してください");
                continue;
            }
        };

        //範囲チェック
        if guess < 1 || guess > 100{
            println!("エラー：１から１００の間で入力してください");
            continue;
        }
        attempts += 1;

        //比較する
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("もっと大きいです"),
            Ordering::Greater => println!("もっと小さいです"),
            Ordering::Equal => {
                println!("\n正解です。答えは{}でした", secret_number);
                println!("試行回数：{}回", attempts);

                //評価する
                match attempts{
                    1 => println!("すごい、１発で当てましたね"),
                    2..=5 => println!("感がさえてますね、素晴らしいです"),
                    6..=10 => println!("いい感じです"),
                    _ => println!("よく頑張りました"),
                }

                break;
            }
        }
    }

    println!("\nお疲れ様でした、終了します")
}