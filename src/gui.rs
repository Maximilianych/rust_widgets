#[derive(Default)]
pub struct WidgetApp {}

impl WidgetApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WidgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Widgets");
            egui::Window::new("Cpu Usage").show(ctx, |ui| {
                
            });
        });
    }
}
