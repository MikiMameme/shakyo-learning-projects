fn main() {
    let staff_list = ["佐藤", "鈴木", "高橋", "田中"];
    let input_name = "加藤";

    println!("----スタッフ照会システム----");

    // ここで &staff_list と「&」をつけて貸し出します
    let is_registered = check_staff_id(&staff_list, input_name);

    if is_registered {
        println!("{}さんは登録済みです、作業を開始してください", input_name);
    } else {
        println!("警告：{}さんは名簿にありません、作業が許可されていないか入力ミスが考えられます", input_name);
    }
}　

// 修正ポイント：[&str] ではなく &[&str] にする（先頭に & を追加）
fn check_staff_id(list: &[&str], name: &str) -> bool {
    list.contains(&name)
}