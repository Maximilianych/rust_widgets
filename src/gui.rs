use std::time;
use sysinfo::System;

use crate::widgets;

// WidgetApp
pub struct WidgetApp {
    system: System,
    frame_duration: time::Duration,
}

// Default implementation
impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            system: System::new(),
            frame_duration: time::Duration::from_secs_f64(1.0 / 5.0),
        }
    }
}

// Some implementation
impl WidgetApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// App implementation
impl eframe::App for WidgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);
        let start = std::time::Instant::now();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Widgets");

            // Cpu Usage
            egui::Window::new("Cpu Usage").show(ctx, |ui| {
                let cpu_usage = widgets::cpu_usage(&mut self.system);
                for (i, cpu) in cpu_usage.iter().enumerate() {
                    ui.label(format!("Cpu {} usage: {:.2}%", i, cpu));
                }
            });

            // Memory Usage
            egui::Window::new("Memory Usage").show(ctx, |ui| {
                let memory_usage = widgets::memory_usage(&mut self.system);

                // TODO: beautiful groups
                ui.label(format!(
                    "Total memory: {:.2} MB",
                    memory_usage.total_memory / 1_048_576
                ));
                ui.label(format!(
                    "Used memory: {:.2} MB",
                    memory_usage.used_memory / 1_048_576
                ));
                ui.label(format!(
                    "Free memory: {:.2} MB",
                    memory_usage.free_memory / 1_048_576
                ));
                ui.label(format!(
                    "Total swap: {:.2} MB",
                    memory_usage.total_swap / 1_048_576
                ));
                ui.label(format!(
                    "Used swap: {:.2} MB",
                    memory_usage.used_swap / 1_048_576
                ));
                ui.label(format!(
                    "Free swap: {:.2} MB",
                    memory_usage.free_swap / 1_048_576
                ))
            });
        });

        // TODO: adequate frame limitation
        let duration = start.elapsed();
        if duration < self.frame_duration {
            std::thread::sleep(self.frame_duration - duration);
        }
    }
}
