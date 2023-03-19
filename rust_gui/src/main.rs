use eframe::egui::accesskit::DescriptionFrom;
use eframe::egui::{self, ScrollArea, Color32, Label, Hyperlink, Separator, Layout};
use eframe::App;


const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);


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
        
        // Use the custom font.
        setup_custom_fonts(&cc.egui_ctx);


        // Add Dummy articles to the app
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("Article {}", a),
            description: format!("Description of article {}", a),
            url: format!("https://rusty.rs/{}", a),
        });

        RustyHeadlines {
            articles: Vec::from_iter(iter),
        }
    }

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            //render title
            let title = format!("=> {}", a.title);
            ui.colored_label(WHITE, title);

            // Render description as a button without it's frame
            ui.add_space(PADDING);
            ui.add(egui::Button::new(&a.description).frame(false));

            // Render the URL as a Hyperlink
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.hyperlink_to("Read more ⤴", &a.url);
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}


// Here you need to improove the quality of code by looking at the examples proovided at:
// https://www.egui.rs/#demo
// Use the sources here:
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/scrolling.rs

impl App for RustyHeadlines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Helvetica".to_owned(),egui::FontData::from_static(include_bytes!("../resources/Helvetica.ttf")));

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Helvetica".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Helvetica".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
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
