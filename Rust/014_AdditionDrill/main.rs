use rand::Rng;
use std::io;

fn main() {
    println!("======== 足し算ドリル ========");

    let mut rng =rand::thread_rng();
    let mut score = 0;
    let mut total = 0;

    loop {
        let a = rng.gen_range(1..=20);
        let b = rng.gen_range(1..=20);
        let answer = a + b;

        total += 1;
        println!("\n問題 {}: {} + {} = ?", total, a, b);
        println!("（qで終了）");
        print!("> ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        let input = input.trim();

        if input == "q" {
            break;
        }

        match input.parse::<i32>() {
            Ok(user_answer) => {
                if user_answer == answer {
                println!("正解です");
                score += 1;
            } else {
            println!("不正解です、正解は {} です", answer);
            }
        }
        Err(_) => {
            println!("エラー：数値を入力してください");
            total -= 1;
        }
    }

    println!("現在のスコア： {} / {}", score, total);
    }

    if total > 0 {
        let accuracy = (score as f64 / total as f64) * 100.0;
        println!("\n最終結果");
        println!("正解数：{} / {}", score, total);
        println!("正答率：{:.1}%", accuracy);

        if accuracy == 100.0 {
            println!("パーフェクト！");
        } else if accuracy >= 80.0 {
            println!("よくできました");
        } else if accuracy >= 60.0 {
            println!("いい感じです");
        } else {
            println!("もう少し頑張りましょう");
        }
    }
    println!("\n終了します、お疲れ様でした");
}