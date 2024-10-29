use sysinfo::System;
use std::time;

use crate::widgets;

pub struct WidgetApp {
    system: System,
    frame_duration: time::Duration,
}

impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            system: System::new(),
            frame_duration: time::Duration::from_secs_f64(1.0/5.0),
        }
    }
}

impl WidgetApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for WidgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let start = std::time::Instant::now();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Widgets");
            egui::Window::new("Cpu Usage").show(ctx, |ui| {
                let cpu_usage = widgets::cpu_usage(&mut self.system);
                for (i, cpu) in cpu_usage.iter().enumerate() {
                    ui.label(format!("Cpu {} usage: {:.2}%", i, cpu));
                }
            });
        });

        let duration = start.elapsed();
        if duration < self.frame_duration {
            std::thread::sleep(self.frame_duration - duration);
        }
    }
}
