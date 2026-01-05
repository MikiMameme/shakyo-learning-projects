use rand::Rng;
use std::io;

fn main() {
    println!("=====本日のことわざ=====");

    let kotowaza = vec![
        "継続は力なり",
        "急がば回れ",
        "失敗は成功のもと",
        "必要は発明の母",
        "千里の道も一歩から",
        "地獄への道は善意で舗装されている",
        "塵も積もれば山となる",
        "石の上にも三年",
        "成せば成る",
        "七転び八起き",
        "思い立ったが吉日",
        "ルビコン川を渡る",
        "人事を尽くして天命を待つ",
        "沈黙は金、雄弁は銀",
        "敵を知り己を知れば百戦危うからず",
        "天は二物を与えず",
        "チャンスの神様は前髪しかない",
        "聞くは一時の恥、聞かぬは一生の恥",
        "捨てる神あれば拾う神あり",
    ];

    let mut rng = rand::thread_rng();

    loop {
        println!("\nEnterキーで表示します(q:終了)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        if input.trim() == "q" {
            println!("\n終了します");
            break;
        }

        let index = rng.gen_range(0..kotowaza.len());
        println!("\n 「{}」", kotowaza[index]);
    }
}