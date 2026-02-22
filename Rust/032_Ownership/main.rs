//所有権の学習

fn main() {
    let nursing_log = String::from("10；00 バイタル異常なし");
    println!("佐藤さんが保持中； {}", nursing_log);

    //これはコピーではなく移動している！
    let suzuki_log = nursing_log;

    println!("鈴木さんが受け取った；{}", suzuki_log);

    //つまり、もしこれをやると...
    //println!("佐藤さんが受け取った； {}", nursing_log);
    //値が違うためコンパイルエラーになる！
}