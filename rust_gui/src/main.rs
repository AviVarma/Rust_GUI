mod rusty_headlines;
use eframe::egui::{self, ScrollArea};

// Here you need to improove the quality of code by looking at the examples proovided at:
// https://www.egui.rs/#demo
// Use the sources here:
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/scrolling.rs

impl eframe::App for rusty_headlines::RustyHeadlines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(540.0, 760.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Headlines",
        options,
        Box::new(|cc| Box::new(rusty_headlines::RustyHeadlines::new(cc))),
    )
}
