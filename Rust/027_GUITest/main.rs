use eframe::egui;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "施設管理システム GUI版",
        native_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(MyAppData::default())
        }),
    )
}

struct MyAppData {
    name: String,
    room: String,
    status_msg: String,
}

impl Default for MyAppData {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            room: "".to_string(),
            status_msg: "情報を入力しください".to_string(),
        }
    }
}

// 2. 画面の見た目と動き
impl eframe::App for MyAppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(" 施設管理システム (CSV保存版)");

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("名前: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.horizontal(|ui| {
                ui.label("部屋: ");
                ui.text_edit_singleline(&mut self.room);
            });

            ui.add_space(10.0);

            if ui.button("CSVへ登録する").clicked() {
                if self.name.is_empty() || self.room.is_empty() {
                    self.status_msg = "名前と部屋番号を入力してください".to_string();
                } else {
                    match save_to_csv(&self.name, &self.room) {
                        Ok(_) => {
                            self.status_msg = format!(" {} 様(Room {}) を保存しました", self.name, self.room);
                            // 保存したら入力欄を空にする（親切設計）
                            self.name.clear();
                            self.room.clear();
                        }
                        Err(e) => self.status_msg = format!("エラー: {}", e),
                    }
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.label(&self.status_msg);
        });
    }
}

fn save_to_csv(name: &str, room: &str) -> std::io::Result<()> {
    let path = "residents_gui.csv";
    let is_new = !std::path::Path::new(path).exists();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if is_new {
        writeln!(file, "氏名,部屋番号")?;
    }
    writeln!(file, "{},{}", name, room)?;
    Ok(())
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "/System/Library/Fonts/Hiragino Sans GB.ttc"
        )),
    );
    fonts.families.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    ctx.set_fonts(fonts);
}