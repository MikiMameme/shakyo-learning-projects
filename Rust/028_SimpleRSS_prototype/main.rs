use reqwest;
use rss::Channel;

fn main() {
    println!("NHK ニュースを取得中...");

    // RSS URL
    let url = "https://www.nhk.or.jp/rss/news/cat0.xml";

    // HTTP GET リクエスト
    let content = reqwest::blocking::get(url)
        .expect("RSS取得に失敗しました")
        .bytes()
        .expect("データの読み取りに失敗しました");

    // RSSをパース
    let channel = Channel::read_from(&content[..])
        .expect("RSSのパースに失敗しました");

    println!("\n=== {} ===\n", channel.title());

    // 記事を表示（最初の5件）
    for (i, item) in channel.items().iter().take(5).enumerate() {
        println!("{}. {}", i + 1, item.title().unwrap_or("タイトルなし"));
        println!("   {}", item.description().unwrap_or("説明なし"));
        println!("   リンク: {}\n", item.link().unwrap_or("リンクなし"));
    }
}