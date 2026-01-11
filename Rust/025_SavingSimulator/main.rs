use std::io;

fn main() {
    println!("貯金シミュレーター");

    println!("１、目標金額はいくらですか？");
    let goal = get_input_f64();

    println!("２、今の貯金額はいくらですか？");
    let mut current_balance = get_input_f64();

    println!("３、毎月の貯金可能額はいくらですか？");
    let monthly_saving = get_input_f64();

    if monthly_saving <= 0.0 {
        println!("エラー：その金額では貯金ができません");
        return;
    }

    let mut month = 0;

    while current_balance < goal {
        month += 1;
        current_balance += monthly_saving;
        println!("{}ヶ月目：残高{:.0}円",month, current_balance);

        if month > 1200 {
            println!("この組み合わせでは１００年かかっても達成できませんでした、見直しをお勧めします");
            break;
        }
    }
    println!("\n====シミュレーションが完了しました====");
    println!("目標達成まで、あと{}ヶ月です", month);
    println!("収入・支出を見直し、コツコツと頑張りましょう");
}

fn get_input_f64() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("読み込みに失敗しました");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("エラー：数字を入れてください");
                continue;
            }
        }
    }
}