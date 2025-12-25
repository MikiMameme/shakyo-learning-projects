use std::io;

//構造体
#[derive(Debug, Clone)]
struct Task{
    id: usize,
    title: String,
    completed: bool,
}

impl Task{
    fn new(id: usize, title:String) -> Self {
        Task {
            id,
            title,
            completed: false,
        }
    }
    fn display(&self){
        let status = if self.completed {"☑"} else {"◻︎"};
        println!("[{}] {} {}", self.id, status, self.title);
    }
}

//タスクリストの管理
struct TodoList{
    tasks: Vec<Task>,
    next_id: usize,
}
impl TodoList {
    fn new()-> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    fn delete(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("タスク{}を削除しました", id);
        } else {
            println!("ID{}のタスクが見つかりません", id);
        }
    }

    //タスクを追加
    fn add(&mut self, title:String){
        let new_task = Task::new(self.next_id, title);
        self.tasks.push(new_task);
        println!("☑️タスクを追加しました（ID: {}）", self.next_id);
        self.next_id += 1;
    }
    //タスク一覧
    fn list(&self){
        if self.tasks.is_empty(){
            println!("タスクはありません");
            return;
        }
        println!("\nタスク一覧");
        for task in &self.tasks {
            task.display();
        }
        println!();
    }
    //タスク完了
    fn complete(&mut self, id:usize){
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id){
            task.completed = true;
            println!("タスク{}を完了にしました", id);
        }else{
            println!("ID{}のタスクが見つかりません",id);
        }
    }
}

fn main() {
    println!("===シンプルTodoリスト===");

    let mut todo_list = TodoList::new();

    loop {
        println!("\nコマンドを入力してください");
        println!("add <タスク名> - タスク追加");
        println!("list         - 一覧表示");
        println!("done <ID>     - 完了にする");
        println!("del <ID>      - 削除");
        println!("quit         - 終了");
        print!("> ");

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "add" => {
                if parts.len() < 2 {
                    println!("エラー：タスク名を入力してください");
                    continue;
                }
                let title = parts[1..].join(" ");
                todo_list.add(title);
            }
            "list" => {
                todo_list.list();
            }
            "done" => {
                if parts.len() < 2 {
                    println!("エラー：IDを指定してください");
                    continue;
                }
                match parts[1].parse::<usize>(){
                    Ok(id) => todo_list.complete(id),
                    Err(_) => println!("エラー：数値を入力してください"),
                }
            }
            "del" => {
                if parts.len() < 2 {
                    println!("エラー：IDを指定してください");
                    continue;
                }
                match parts[1].parse::<usize>() {
                    Ok(id) => todo_list.delete(id),
                    Err(_) => println!("エラー：数値を入力してください"),
                }
            }

            "quit" | "q" => {
                println!("\n終了します、お疲れ様でした");
                break;
            }
            _ => {
                println!("エラー：不明なコマンドです");
            }
        }
    }
}