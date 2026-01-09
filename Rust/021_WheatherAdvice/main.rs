enum Weather {
    Sunny,
    Rainy,
    Cloudy,
    Snowy,
}

fn main(){
    let current_weater = Weather::Sunny;

    println!("今日のお天気のアドバイス");

    match current_weater {
        Weather::Sunny => {
            println!("晴れは洗濯物を干すチャンスです！");
        }
        Weather::Rainy => {
            println!("雨は読書のチャンスです");
        }
        Weather::Cloudy => {
            println!("曇りはお守りがわりに折り畳み傘をどうぞ");
        }
        Weather::Snowy => {
            println!("雪道の運転は慎重に、スタッドレスタイヤやタイヤチェーンは必須です！");
        }
    }
    print_temperature(25);
}

fn print_temperature(temp: i32) {
    println!("今の気温は　{} 度ですね", temp);
    if temp > 20 {
        println!("過ごしやすい温度〜暑さを感じる温度です、暑さを感じる場合は水分補給をしましょう");
    } else {
        println!("肌寒かったり寒い温度です");
    }
}