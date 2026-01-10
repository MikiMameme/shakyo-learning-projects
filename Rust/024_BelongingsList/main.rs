use std::io;

fn main() {
    let mut checklist: Vec<String> = Vec::new();

    println!("=== 持ち物登録アプリ ===");
    println!("持っていくものを登録します、'end'で入力を終了します");

    loop {
        println!("追加するアイテム名：");

        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("読み込みに失敗しました");

        let item = item.trim().to_string();

        if item == "end" {
            break;
        }

        if !item.is_empty() {
            checklist.push(item);
            println!("リストに追加しました");
        }
    }
    println!("\n---- 今日の持ち物リスト ----");
    if checklist.is_empty() {
        println!("持ち物が登録されていません、忘れ物はありませんか？");
    } else {
        for (i, thing) in checklist.iter().enumerate() {
            println!("{}: [] {}", i + 1, thing);
        }
    }
    println!("準備できましたか？今日も一日、ご安全に！")
}