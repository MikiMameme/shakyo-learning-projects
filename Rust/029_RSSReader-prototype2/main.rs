use iced::{
    widget::{column, container, row, scrollable, text, button},
    Alignment, Element, Length, Sandbox, Settings,
};
use reqwest;
use rss::Channel;

fn main() -> iced::Result {
    RssReader::run(Settings {
        // ここを「バイトデータ」として直接渡します
        default_font: iced::Font::with_name("Hiragino Sans"),
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
struct Article {
    title: String,
    description: String,
    link: String,
}

struct RssReader {
    articles: Vec<Article>,
    selected_article: Option<usize>,
}

#[derive(Debug,Clone)]
enum Message {
    SelectArticle(usize),
}

impl Sandbox for RssReader {
    type Message = Message;
    fn new() -> Self {
        let articles = fetch_rss();

        RssReader {
            articles,
            selected_article: None,
        }
    }

    fn title(&self) -> String {
        String::from("RssReader(動作試験版)")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectArticle(index) => {
                self.selected_article = Some(index);
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let mut article_list = column![
            text("ニュース").size(20),
        ]
            .padding(10)
            .spacing(5);

        for(index, article) in self.articles.iter().enumerate() {
            let is_selected = self.selected_article == Some(index);

            let article_button = button(
                text(&article.title)
                    .size(14)
            )
                .on_press(Message::SelectArticle(index))
                .padding(10)
                .width(Length::Fill);

            article_list = article_list.push(article_button);
        }

        let list_scroll = scrollable(article_list)
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let detail_view = if let Some(index) = self.selected_article {
            if let Some(article) = self.articles.get(index) {
                column![
                    text(&article.title).size(18),
                    text("").size(10),
                    text(&article.description).size(14),
                    text("").size(10),
                    text(&article.link).size(12),
                ]
                    .padding(20)
                    .spacing(10)
            } else {
                column![text("記事が見つかりません")]
                    .padding(20)
            }
        } else {
            column![text("記事を選択してください")]
                .padding(20)
        };

        let detail_scroll = scrollable(detail_view)
            .width(Length::FillPortion(2))
            .height(Length::Fill);

        let  content = row![list_scroll, detail_scroll]
            .spacing(0)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn fetch_rss() -> Vec<Article> {
    let url = "https://www.nhk.or.jp/rss/news/cat0.xml";

    match reqwest::blocking::get(url) {
        Ok(response) => {
            match response.bytes() {
                Ok(content) =>{
                    match Channel::read_from(&content[..]) {
                        Ok(channel) =>{
                            channel
                                .items()
                                .iter()
                                .map(|item| Article {
                                    title: item
                                        .title()
                                        .unwrap_or("タイトルなし")
                                        .to_string(),
                                    description: item
                                        .description()
                                        .unwrap_or("説明なし")
                                        .to_string(),
                                    link: item
                                        .link()
                                        .unwrap_or("リンクなし")
                                        .to_string(),
                                })
                                .collect()
                        }
                        Err(_) => vec![],
                    }
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    }
}