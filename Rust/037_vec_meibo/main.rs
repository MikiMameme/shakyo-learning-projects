fn main() {
    let mut staffs = vec![String::from("佐藤"), String::from("鈴木")];

    println!("初期名簿:{:?}",staffs);

    staffs.push(String::from("高橋"));
    staffs.push(String::from("田中"));

    println!("追加後の名簿：{:?}", staffs);

    let resting_staff = staffs.pop();

    match resting_staff {
        Some(name) => println!("{}さんは休憩に入りました。", name),
        None => println!("誰もいません"),
    }

    println!("現在の稼働スタッフ:{:?}",staffs);
}