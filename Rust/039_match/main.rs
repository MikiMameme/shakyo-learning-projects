fn main (){

    let status_code = 1;

    println!("---- 安否確認報告 ----");

    let message = match status_code {
        0 => "確認されていません、支給確認せよ",
        1 => "異常なし、巡回を継続せよ",
        _ => "不明なエラー、入力を確認せよ",
    };

    println!("報告内容：{}", message);
}