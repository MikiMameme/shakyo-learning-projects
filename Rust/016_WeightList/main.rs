use std::io;

fn main() {
 println!("=====体重記録アプリ=====");

    let mut weights: Vec<f64> = Vec::new();

    loop {
        println!("\nコマンドを入力してください：");
        println!(" add <体重>  - 記録");
        println!(" list       - 一覧表示");
        println!(" avg        - 平均表示");
        println!(" quit       - 終了");
        print!(" >");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                if parts.len() < 2{
                    println!("エラー：体重を入力してください");
                    continue;
                }
                match parts[1].parse::<f64>() {
                    Ok(weight) => {
                        weights.push(weight);
                        println!("{}kgを記録しました", weight);
                    }
                    Err(_) => println!("エラー：数値を入力してください"),
                }
            }
            "list" => {
                if weights.is_empty() {
                    println!("記録がありません");
                } else  {
                    println!("\n==== 体重記録 ====");
                    for(i,weight) in weights.iter().enumerate() {
                        println!("{}. {}kg", i+1, weight);
                    }
                    println!();
                }
            }
            "avg" => {
                if weights.is_empty(){
                    println!("記録がありません");
                } else {
                    let sum: f64 = weights.iter().sum();
                    let avg = sum / weights.len() as f64;
                    println!("\n平均体重： {:.1}kg", avg);
                    println!("記録数：{}回", weights.len());
                }
            }
            "quit" | "q" => {
                println!("\n終了します");
                break;
            }
            _ => {
                println!("エラー：不明なコマンドです");
            }
        }
    }
}