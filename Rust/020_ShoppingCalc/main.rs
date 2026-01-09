struct Item {
    name:String,
    price: u32,
}

fn main() {
    println!("===== 買い物リスト（合計金額計算機能付き） =====");

    let mut cart = Vec::new();

    cart.push(Item { name: String::from("プロテイン"), price: 3500 });
    cart.push(Item { name: String::from("サバ缶"), price: 200 });
    cart.push(Item { name: String::from("オートミール"), price: 800});

    let mut total = 0;

    println!("\n【カートの中身】");
    for item in &cart {
        println!("{}: {}円", item.name, item.price);
        total += item.price;
    }

    println!("======== 結果 ========");
    println!("合計金額は{} 円です", total);

    let total_with_tax = (total as f64 * 1.1) as u32;
    println!("税込価格（10％）は{} 円です", total_with_tax);
}