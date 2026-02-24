fn main(){
    let mut original_log = String::from("９：００　全員起床。");

    //Clone（複製）
    let mut copy_log = original_log.clone();
    copy_log.push_str("(コピーに追記：異常なし)");

    println!("元の内容；{}", original_log);
    println!("複製の内容：{}", copy_log);
    println!("---");

    //&mut（可変的借用）
    add_report(&mut original_log);

    println!("鈴木さんが追記した後の元の日誌：{}", original_log);
}

fn add_report(log: &mut String) {
    log.push_str("１０：００　鈴木が検温実施。");
}