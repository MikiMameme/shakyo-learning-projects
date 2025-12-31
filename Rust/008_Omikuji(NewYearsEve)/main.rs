//おみくじアプリ（２０２５年大晦日バージョン）

use rand::Rng;
use std::io;

fn main(){
    println!("=====大晦日 開運おみくじ===");

    let fortunes = vec!["大吉", "中吉", "小吉", "吉", "末吉", "凶", "大凶"];

    let messages = vec![
        "来年は最高の運気になるでしょう",
        "いい感じです、来年も期待できます",
        "まずまず、来年もコツコツ頑張りましょう",
        "悪くない一年になりそうです、無理は禁物",
        "慎重に、でも大丈夫です",
        "今年は控えめに、来年はきっと良くなります",
        "今年は特に慎重に、我慢の年です"
    ];

    loop{
        println!("\nEnterキーでおみくじを引く (q:終了)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        if input.trim() == "q"{
            println!("\n良いお年を！");
            break;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..fortunes.len());

        println!("\n【結果】{}", fortunes[index]);
        println!("{}", messages[index]);
    }
}
