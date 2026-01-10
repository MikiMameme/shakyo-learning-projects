enum Shift {
    Day,
    Early,
    Night,
}

fn get_shift_allowance(shift: Shift) -> i32 {
    match shift {
        Shift::Day => 0,
        Shift::Early => 1000,
        Shift::Night => 5000,
    }
}

fn main() {
    let today_shift = Shift::Night;

    let allowance = get_shift_allowance(today_shift);

    println!("今日のシフトは「夜勤」です");
    println!("今日の勤務手当は{}円です", allowance);

    let message = match allowance {
        0 => "本日もご安全に！",
        1..=2000 => "早朝勤務お疲れ様です",
        _ => "夜間勤務お疲れ様です",
    };

    println!("{}", message);
}