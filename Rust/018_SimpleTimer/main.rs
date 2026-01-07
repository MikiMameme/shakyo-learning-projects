use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn draw_progress_bar(current: u32, total: u32,width: usize){
    let progress = current as f64 / total as f64;
    let filled = (progress * width as f64) as usize;
    let empty = width - filled;

    let bar = "█".repeat(filled) + &"░".repeat(empty);
    let percentage = (progress * 100.0) as u32;

    print!("\r[{}] {}% ({}/{}秒)", bar, percentage, current, total);
    io::stdout().flush().unwrap();
}

fn main() {
    println!("==========簡易タイマー==========");

    loop {
        println!("\n秒数を入力してください（q:終了します）");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("エラー：入力の読み取りに失敗しました");

        let input = input.trim();

        if input == "q" {
            println!("\n終了します");
            break;
        }

        let seconds: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("エラー：数値を入力してください");
                continue;
            }
        };
        if seconds == 0 {
            println!("エラー：１以上の数値を入力してください");
            continue;
        }
        println!("\n{}秒のタイマーを開始します", seconds);
        println!();

        for elapsed in 0..=seconds {
            draw_progress_bar(elapsed, seconds, 30);

            if elapsed < seconds {
                thread::sleep(Duration::from_secs(1));
            }
        }
        println!("\n\n時間です！");
    }
}