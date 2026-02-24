//おやつ代チェッカー（コンソールアプリ・プロトタイプ）

enum Menu {
    Coffee,
    Cake,
    Set(i32),
}

fn main() {
    let my_order = Menu::Set(50);

    let price = match my_order {
        Menu::Coffee => 400,
        Menu::Cake => 500,
        Menu::Set(discount) => (400+500) - discount,
    };

    println!("☕️お会計は{}円です。ごゆっくりどうぞ！", price);
}