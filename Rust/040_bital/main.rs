fn main() {
     let mut temperature = 36.2;

    println!("---- バイタルチェック ----");
    println!("測定前：{}℃", temperature);

    measure_temperature(&mut temperature);

    println!("測定後；{}℃（更新しました）", temperature);
}

fn measure_temperature(temperature: &mut f64) {
    println!("...測定中...");

    *temperature = 36.8;
}