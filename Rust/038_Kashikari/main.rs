// 消える変数と貸し出し

fn main(){
    let report = String::from("14:00 全員安眠中");

    save_to_database(report);

    //println!("保存した内容:{}", report);
    println!("--- 実験１終了：reportはもう使えません ---");

    let report2 =  String::from("15:00おやつレク開始。");

    show_report(&report2);

    println!("貸し出した後でも使える:{}", report2);
}

fn save_to_database(data: String) {
    println!("データベースに保存しました:{}", data);
}

fn show_report(data: &String){
    println!("日誌の内容を確認:{}", data);
}