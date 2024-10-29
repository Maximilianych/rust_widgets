mod gui;

use gui::WidgetApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Widgets Yopta",
        native_options,
        Box::new(|cc| Ok(Box::new(WidgetApp::new(cc)))),
    )
}


