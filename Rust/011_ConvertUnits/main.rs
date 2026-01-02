use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64{
    c * 9.0/5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - 32.0) * 5.0 / 9.0

}

fn km_to_miles(km: f64) -> f64{
    km * 0.621371
}

fn miles_to_km(miles: f64) -> f64{
    miles / 0.621371
}

fn kg_to_pounds(kg: f64) -> f64{
    kg * 2.20462
}

fn pounds_to_kg(pounds: f64) -> f64{
    pounds / 2.20462
}

fn main() {
    println!("===単位変換ツール===");

    loop{
        println!("\n変換したい項目を選んでください");
        println!(" 1: 温度(摂氏(℃) ↔︎ 華氏(℉))");
        println!(" 2: 距離(キロメートル ↔ マイル)");
        println!(" 3: 重さ（キログラム ↔ ポンド）");
        println!(" q: 終了");
        print!("> ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("入力の読み取りに失敗しました");

        let choice = choice.trim();

        if choice == "q"{
            println!("\n終了します");
            break;
        }
        match choice {
            "1" => {
                println!("\n温度変換：");
                println!(" 1: 摂氏（℃） → 華氏（℉）");
                println!(" 2: 華氏（℉） → 摂氏（℃）");
                print!("> ");

                let mut direction= String::new();
                io::stdin().read_line(&mut direction).unwrap();

                print!("値を入力：");
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();

                let value: f64 = match value.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("エラー：数値を入力してください");
                        continue;
                    }
                };

                match direction.trim(){
                    "1" => {
                        let result = celsius_to_fahrenheit(value);
                        println!("\n{}℃ = {:.2}℉", value, result);
                    }
                    "2" =>{
                        let result = fahrenheit_to_celsius(value);
                        println!("\n{}℉ = {:.2}℃", value, result);
                    }
                    _ => println!("エラー：無効な選択です"),
                }
            }
            "2" => {
                println!("\n距離変換：");
                println!(" 1: キロメートル → マイル");
                println!(" 2: マイル → キロメートル");
                print!("> ");

                let mut direction= String::new();
                io::stdin().read_line(&mut direction).unwrap();

                print!("値を入力：");
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();

                let value: f64 = match value.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("エラー：数値を入力してください");
                        continue;
                    }
                };

                match direction.trim(){
                    "1" => {
                        let result = km_to_miles(value);
                        println!("\n{} キロメートル = {:.2} マイル", value, result);
                    }
                    "2" => {
                        let result = miles_to_km(value);
                        println!("\n{} マイル = {:.2} キロメートル", value, result);
                    }
                    _ => println!("エラー：無効な選択です"),
                }
            }
            "3" => {
                println!("\n重さ変換：");
                println!(" 1: キログラム → ポンド");
                println!(" 2: ポンド → キログラム");
                print!("> ");

                let mut direction= String::new();
                io::stdin().read_line(&mut direction).unwrap();

                print!("値を入力：");
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();

                let value: f64 = match value.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("エラー：数値を入力してください");
                        continue;
                    }
                };

                match direction.trim(){
                    "1" => {
                        let result = kg_to_pounds(value);
                        println!("\n{} キログラム = {:.2} ポンド", value, result);
                    }
                    "2" => {
                        let result = pounds_to_kg(value);
                        println!("\n{} ポンド = {:.2} キログラム", value, result);
                    }
                    _ => println!("エラー：無効な選択です"),
                }
            }
            _ => {
                println!("エラー：無効な選択です")
            }
        }
    }
}