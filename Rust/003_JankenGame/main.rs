use rand::Rng;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand{
    Rock,
    Paper,
    Scissors,
}

impl Hand{
    fn random() -> Self {
        let mut rng=rand::thread_rng();
        match rng.gen_range(0..3){
            0 => Hand::Rock,
            1 => Hand::Paper,
            _ => Hand::Scissors,
        }
    }

    //文字列から手を生成する
    fn from_str(s: &str) -> Option<Self> {
        match s.trim(){
            "1" | "グー" | "ぐー" => Some(Hand::Rock),
            "2" | "チョキ" | "ちょき" => Some(Hand::Paper),
            "3" | "パー" | "ぱー" => Some(Hand::Scissors),
            _ => None,
        }
    }

    //手を日本語表示する
    fn to_japanese(&self) -> &str {
        match self {
            Hand::Rock => "グー",
            Hand::Paper => "チョキ",
            Hand::Scissors => "パー",
        }
    }

    //勝敗判定
    fn  beats(&self, other: &Hand) -> GameResult {
        if self == other {
            GameResult::Draw
        }else if (*self == Hand::Rock && *other == Hand::Scissors)
            || (*self == Hand::Paper && *other == Hand::Rock)
            || (*self == Hand::Scissors && *other == Hand::Paper)
        {
            GameResult::Win
        }else {
            GameResult::Lose
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult{
    Win,
    Lose,
    Draw,
}

fn main(){
    println!("==============================");
    println!("  じゃんけんゲーム");
    println!("==============================");

    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    loop {
        println!("\nジャンケンポン！");
        println!("1: グー");
        println!("2: チョキ");
        println!("3: パー");
        println!("q: 終了");
        print!("> ");

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

        let input = input.trim();

        if input == "q" || input == "Q"{
            break;
        }

        let player_hand = match Hand::from_str(input){
            Some(hand) => hand,
            None => {
                println!("エラー：無効な入力です");
                continue;
            }
        };

        let computer_hand = Hand::random();

        println!("\nあなた：{}", player_hand.to_japanese());
        println!("コンピュータ：{}", computer_hand.to_japanese());

        let result = player_hand.beats(&computer_hand);

        match result {
            GameResult::Win => {
                println!("あなたの勝ちです");
                wins += 1;
            }
            GameResult::Lose => {
                println!("あなたの負けです");
                losses += 1;
            }
            GameResult::Draw => {
                println!("あいこです");
                draws += 1;
            }
        }
        println!("\n【戦績】勝ち：{} / 負け：{} / あいこ：{}", wins, losses, draws);
    }

    println!("\n==============================");
    println!("最終戦績");
    println!("勝ち：{} / 負け：{} / あいこ：{}", wins, losses, draws);
    println!("==============================");
    println!("終了します");
}