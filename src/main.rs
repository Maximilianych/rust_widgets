pub mod gui;
pub mod widgets;

use egui::ViewportBuilder;
use gui::WidgetApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        // viewport: ViewportBuilder::default().with_always_on_top().with_decorations(false).with_transparent(true).with_resizable(false).with_titlebar_shown(true),
        ..Default::default()
    };
    eframe::run_native(
        "My Widgets Yopta",
        native_options,
        Box::new(|cc| Ok(Box::new(WidgetApp::new(cc)))),
    )
}


