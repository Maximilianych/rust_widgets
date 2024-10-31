use egui_plot::{Line, Plot};
use std::collections::VecDeque;
use std::time;
use sysinfo::{Disks, System};

use crate::widgets::{self, memory};

// WidgetApp
pub struct WidgetApp {
    system: System,
    discs: Disks,
    cpu_usage_history: VecDeque<f32>,
    memory_usage_history: VecDeque<f32>,
    frame_duration: time::Duration,
}

// Default implementation
impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            system: System::new(),
            discs: Disks::new_with_refreshed_list(),
            cpu_usage_history: VecDeque::with_capacity(100),
            memory_usage_history: VecDeque::with_capacity(100),
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

            // Cpu Usage Plot
            egui::Window::new("Cpu Usage Plot").show(ctx, |ui| {
                let cpu_usage: f32 = widgets::cpu_usage(&mut self.system).iter().sum::<f32>()
                    / self.system.cpus().len() as f32;
                self.cpu_usage_history.push_back(cpu_usage);
                if self.cpu_usage_history.len() > 100 {
                    self.cpu_usage_history.pop_front();
                };

                let cpu_usage_points: Vec<_> = self
                    .cpu_usage_history
                    .iter()
                    .enumerate()
                    .map(|(i, v)| [i as f64, *v as f64])
                    .collect();
                let cpu_usage_line = Line::new(cpu_usage_points);

                Plot::new("cpu_usage_plot")
                    .view_aspect(3.0)
                    .include_y(0.0)
                    .include_y(100.0)
                    .include_x(100.0)
                    .show(ui, |plot_ui| plot_ui.line(cpu_usage_line));
            });

            // memory usage plot
            egui::Window::new("Memory Usage Plot").show(ctx, |ui| {
                self.system.refresh_memory();

                self.memory_usage_history.push_back(self.system.used_memory() as f32 / 1_048_576.0);
                if self.memory_usage_history.len() > 100 {
                    self.memory_usage_history.pop_front();
                }
                
                let memory_usage_points: Vec<_> = self
                    .memory_usage_history
                    .iter()
                    .enumerate()
                    .map(|(i, v)| [i as f64, *v as f64])
                    .collect();
                let memory_usage_line = Line::new(memory_usage_points);

                Plot::new("memory_usage_plot")
                    .view_aspect(3.0)
                    .include_y(0.0)
                    .include_y(self.system.total_memory() as f64 / 1_048_576.0)
                    .include_x(100.0)
                    .show(ui, |plot_ui| plot_ui.line(memory_usage_line));
            });
        });

        // TODO: adequate frame limitation
        let duration = start.elapsed();
        if duration < self.frame_duration {
            std::thread::sleep(self.frame_duration - duration);
        }
    }
}
