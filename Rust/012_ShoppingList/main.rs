use std::io;

#[derive(Debug,Clone)]
struct Item {
    id: usize,
    name: String
}

struct ShoppingList {
    items: Vec<Item>,
    next_id: usize,
}

impl ShoppingList {
    fn new() -> Self {
    ShoppingList{
        items: Vec::new(),
        next_id: 1,
        }
    }

    fn add(&mut self, name: String){
        let item = Item {
            id: self.next_id,
            name,
        };
        self.items.push(item);
        println!("「{}」を追加しました （ID：{}）",self.items.last().unwrap().name, self.next_id);
        self.next_id += 1;
    }

    fn list(&self){
        if self.items.is_empty() {
            println!("リストは空です");
            return;
        }
        println!("\n=====買い物リスト=====");
        for item in &self.items {
            println!("{},{}", item.id,item.name);
        }
        println!();
    }
    fn remove(&mut self, id: usize){
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            let removed = self.items.remove(pos);
            println!("「{}」を削除しました", removed.name);
        }else{
            println!("ID {} のアイテムが見つかりません", id);
        }
    }
    fn count(&self) -> usize{
        self.items.len()
    }
}

fn main() {
    println!("======== 買い物リスト ========");

    let mut shopping_list = ShoppingList::new();

    loop {
        println!("\nコマンドを入力してください：");
        println!(" add <アイテム名> - 追加");
        println!(" list            - 一覧表示");
        println!(" del <ID>        - 削除");
        println!(" quit            - 終了");
        print!("> ");

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

        let input = input.trim();
        let parts: Vec<&str> = input.splitn(2, ' ').collect();

        match parts[0] {
            "add" => {
                if parts.len() < 2 {
                    println!("エラー：アイテム名を入力してください");
                    continue;
                }
                shopping_list.add(parts[1].to_string());
            }
            "list" => {
                shopping_list.list();
                println!("合計：{} アイテム", shopping_list.count());
            }
            "del" => {
                if parts.len() < 2 {
                    println!("エラー：IDを指定してください");
                    continue;
                }
                match parts[1].parse::<usize>(){
                    Ok(id) => shopping_list.remove(id),
                    Err(_) => println!("エラー：数値を入力してください"),
                }
            }
            "quit" | "q" => {
                println!("\n終了します、お疲れ様でした");
                break;
            }
            "" => continue,
            _ => {
                println!("エラー：不明なコマンドです");
            }
        }
    }
}