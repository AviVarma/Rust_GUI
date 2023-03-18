use eframe::egui::{self, collapsing_header::HeaderResponse, scroll_area, ScrollArea};

#[derive(Default)]
struct RustyHeadlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl RustyHeadlines{
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("Article {}", a),
            description: format!("Description of article {}", a),
            url: format!("https://rusty.rs/{}", a),
        });

        Self::default();

        RustyHeadlines {
            articles: Vec::from_iter(iter),
        }
    }
}


// Here you need to improove the quality of code by looking at the examples proovided at:
// https://www.egui.rs/#demo
// Use the sources here:
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/scrolling.rs

impl eframe::App for RustyHeadlines {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for a in &self.articles{
                    ui.label(&a.title);
                    ui.label(&a.url);
                    ui.label(&a.description);
                
                }
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Rusty Headlines", native_options, Box::new(|cc| Box::new(RustyHeadlines::new(cc))));
}
