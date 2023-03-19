mod rusty_headlines;
use eframe::{egui::{self, ScrollArea, Ui, Separator, TopBottomPanel, Context}, App};
use emojis;
use rusty_headlines::{RustyHeadlines};

const PADDING: f32 = 5.0;

// Here you need to improove the quality of code by looking at the examples proovided at:
// https://www.egui.rs/#demo
// Use the sources here:
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/scrolling.rs

impl App for RustyHeadlines {
    fn update(&mut self, cc: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_top_panel(cc);
        egui::CentralPanel::default().show(cc, |ui| {
            render_header(ui);
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(cc);
        });
    }
}

fn render_footer(cc: &Context){
    TopBottomPanel::bottom("footer").show(cc, | ui| {
        ui.vertical_centered(| ui| {
            ui.add_space(10.);
            
            let news = emojis::get("📰").unwrap();
            ui.hyperlink_to(format!("{} API source: newsapi.org", news),"https://newsapi.org/");

            use egui::special_emojis::GITHUB;
            ui.hyperlink_to(format!("{} My GitHub", GITHUB),"https://github.com/AviVarma");

            ui.add_space(10.);
        })
    });
}

fn render_header(ui: &mut Ui ){
    ui.vertical_centered(| ui| {
        ui.heading("Headlines")
    });
    ui.add_space(PADDING);
    let seperator = Separator::default().spacing(20.0);
    ui.add(seperator);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(540.0, 760.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Headlines",
        options,
        Box::new(|cc| Box::new(RustyHeadlines::new(cc))),
    )
}
