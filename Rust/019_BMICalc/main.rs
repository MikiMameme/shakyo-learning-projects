use std::io;

fn main(){
    println!("==== BMI健康計算機 ====");

    println!("1. 身長（cm）を入力してください");
    let mut height_cm = String::new();
    io::stdin().read_line(&mut height_cm).expect("入力エラー：入力が正しくありません");

    println!("2.体重（kg）を入力してください");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("入力エラー：入力が正しくありません");

    let h: f64 = height_cm.trim().parse::<f64>().expect("エラー：数字を入力してください")/100.0;
    let w: f64 = weight.trim().parse::<f64>().expect("エラー：数字を入力してください");

    let bmi = w / (h * h);

    println!("-----結果-----");
    println!("あなたのBMIは {:.1} です", bmi);

    if bmi < 18.5 {
        println!("痩せ型です");
    } else if bmi < 25.0 {
        println!("標準体重です");
    } else {
        println!("太り気味〜肥満です");
    }
}