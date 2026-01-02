use std::io;

struct Expence {
    title:String,
    amount: i32,
}

fn main(){
    let mut list:Vec<Expence> = Vec::new();

    loop{
        println!("\n=====かんたん家計簿=====");
        println!("1: 追加");
        println!("2: 一覧表示");
        println!("3: 合計表示");
        println!("q: 終了");
        print!("> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim(){
            "1" => add_expense(&mut list),
            "2" => show_list(&list),
            "3" => show_total(&list),
            "q" => {
                println!("終了します");
                break;
            }
            _ => println!("エラー：入力が正しくありません"),
        }
    }
}

fn add_expense(list: &mut Vec<Expence>){
    let mut title:String = String::new();
    let mut amount = String::new();

    println!("何に使いました？");
    io::stdin().read_line(&mut title).unwrap();

    println!("いくら使いました？");
    io::stdin().read_line(&mut amount).unwrap();

    let value: i32 = amount.trim().parse().unwrap_or(0);

    list.push(Expence {
        title: title.trim().to_string(),
        amount: value,
    });

    println!("追加しました");
}

fn show_list(list: &Vec<Expence>){
    if list.is_empty(){
        println!("エラー：何も登録されていません");
        return;
    }

    println!("\n----- 一覧 -----");
    for (i, e) in list.iter().enumerate() {
        println!("{}: {} - {} 円", i + 1, e.title, e.amount);
    }
}

fn show_total(list: &Vec<Expence>){
    let mut total = 0;
    for e in list{
        total += e.amount;
    }
    println!("合計：{} 円", total);
}