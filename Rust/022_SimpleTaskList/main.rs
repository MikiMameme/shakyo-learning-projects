use std::io;

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    println!("=== ToDoメモ ===");
    println!("使い方：追加したいタスクを入力してください、'exit'で終了します");

    loop {
        println!("\n[現在のタスク数：{}]", todo_list.len());
        println!("タスクを入力してください（exitで終了します）：");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("読み込みに失敗しました");

        let task = input.trim();

        if task == "exit" {
            println!("お疲れ様でした");
            break;
        } else if task == "del" {
            if !todo_list.is_empty() {
                let removed = todo_list.remove(0);
                println!("完了したタスクを削除しました: {}", removed);
            } else {
                println!("エラー：削除するタスクがありません");
            }
        } else if task.is_empty() {
            println!("エラー：入力してください");
    } else {
    todo_list.push(task.to_string());
            println!("リストに追加しました：{}", task);

            println!("----- 現在のリスト ------");
            for (i, t) in todo_list.iter().enumerate() {
                println!("{}: {}", i + 1, t);
            }
        }
    }
}
