use std::io;
use std::collections::HashMap;

fn main() {
    let mut residents = HashMap::new();
    println!("=== 施設管理システム 1.0 ===");

    loop {
        println!("\n何をしますか？ (1: 登録, 2: 検索, その他: 終了)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("エラー：入力の読み取りに失敗しました");

        match choice.trim() {
            "1" => {
                println!("名前を入力：");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("エラー：入力の読み取りに失敗しました");

                println!("部屋番号を入力：");
                let mut room = String::new();
                io::stdin().read_line(&mut room).expect("エラー：入力の読み取りに失敗しました");

                residents.insert(name.trim().to_string(), room.trim().to_string());
                println!("登録完了");
            }
            "2" => {
                println!("検索する名前：");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("エラー：入力の読み取りに失敗しました");
                let name = name.trim();

                match residents.get(name) {
                    Some(room) => println!(" {} 様は {} 号室です", name, room),
                    None => println!(" {} 様は宿泊登録されていません", name),
                }
            }
            _ => break,
        }
    }
    println!("システムを終了します");
}