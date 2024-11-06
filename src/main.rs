pub mod gui;
pub mod widgets;

use egui::ViewportBuilder;
use gui::WidgetApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_decorations(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "My Widgets Yopta",
        native_options,
        Box::new(|cc| Ok(Box::new(WidgetApp::new(cc)))),
    )
}


