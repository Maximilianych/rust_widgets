use std::time;
use sysinfo::{Disks, System};
use egui_plot::{Line, Plot, PlotPoints};

use crate::widgets;

// WidgetApp
pub struct WidgetApp {
    system: System,
    discs: Disks,
    frame_duration: time::Duration,
}

// Default implementation
impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            system: System::new(),
            discs: Disks::new_with_refreshed_list(),
            frame_duration: time::Duration::from_secs_f64(1.0 / 60.0),
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
                    memory_usage.total_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Used memory: {:.2} MB",
                    memory_usage.used_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Free memory: {:.2} MB",
                    memory_usage.free_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Total swap: {:.2} MB",
                    memory_usage.total_swap as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Used swap: {:.2} MB",
                    memory_usage.used_swap as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Free swap: {:.2} MB",
                    memory_usage.free_swap as f64 / 1_048_576.0
                ))
            });

            // Disk Usage
            egui::Window::new("Disk Usage").show(ctx, |ui| {
                let disk_usage = widgets::disk_usage(&mut self.discs);
                for disc in disk_usage {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} {}",
                            disc.name().to_string_lossy(),
                            disc.mount_point().to_string_lossy()
                        ));
                        ui.vertical(|ui| {
                            ui.label(format!(
                                "Available: {:.2} GB",
                                disc.available_space() as f64 / 1_048_576.0
                            ));
                            ui.label(format!(
                                "Total: {:.2} GB",
                                disc.total_space() as f64 / 1_048_576.0
                            ));
                        })
                    });
                }
            });

            // test
            egui::Window::new("Test").show(ctx, |ui| {
                let sin: Vec<_> = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                }).collect();

                let line_1 = Line::new(sin.to_vec());
                let line_2 = Line::new(sin.to_vec());
                let line_3 = Line::new(sin.to_vec());

                Plot::new("sin_plot_1").view_aspect(10.0).show(ui, |plot_ui| plot_ui.line(line_1));
                Plot::new("sin_plot_2").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line_2));
                Plot::new("sin_plot_3").view_aspect(3.0).show(ui, |plot_ui| plot_ui.line(line_3));
            });
        });

        // TODO: adequate frame limitation
        let duration = start.elapsed();
        if duration < self.frame_duration {
            std::thread::sleep(self.frame_duration - duration);
        }
    }
}
