use rand::Rng;
use std::io;

struct Otoshidama {
    name: &'static str,
    amount: u32,
    message: &'static str,
}

fn main(){
    println!("=== 2026年 お年玉ガチャ ===");

    let list = vec![
        Otoshidama { name: "神",amount: 100000, message: "おめでとう！初売りは何買う？"},
        Otoshidama { name: "大当たり",amount: 50000, message: "すごい！新しいゲームを買う？"},
        Otoshidama { name: "当たり",amount: 30000, message: "臨時収入！今日は焼肉にでも行く？"},
        Otoshidama { name: "まあまあ",amount: 10000,message: "平均的な額、悪くないよ！"},
        Otoshidama { name: "渋い",amount: 5000, message: "うん、現実はこんなもん"},
        Otoshidama { name: "しょっぱい", amount: 3000, message: "まあ、ドンマイ"},
        Otoshidama { name: "どんまい", amount: 1000, message: "もらえるだけマシかな..."},
    ];

    loop{
        println!("\nEnterでガチャを引く(qで終了)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            println!("\nいい一年になりますように");
            break;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..list.len());
        let result = &list[index];

        println!("\n【{}】",result.name);
        println!("お年玉:{}円",result.amount);
        println!("{}",result.message);
    }
}