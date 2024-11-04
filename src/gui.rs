use chrono::Local;
use egui_plot::{Line, Plot};
use std::collections::VecDeque;
use std::time;
use sysinfo::{Disks, System};

use crate::widgets;

// WidgetApp
pub struct WidgetApp {
    system: System,
    disks: Disks,
    cpu_usage_history: VecDeque<f32>,
    memory_usage_history: VecDeque<f32>,
    last_update: time::Instant,
    update_interval: time::Duration,
    cpu_usage: Vec<f32>,
    memory_usage: widgets::MemoryUsage,
    weather_client: reqwest::blocking::Client,
}

// Default implementation
impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            last_update: time::Instant::now().checked_sub(time::Duration::from_secs(1)).unwrap(),
            update_interval: time::Duration::from_secs(1),
            system: System::new(),
            disks: Disks::new_with_refreshed_list(),
            cpu_usage_history: VecDeque::with_capacity(100),
            memory_usage_history: VecDeque::with_capacity(100),
            cpu_usage: Vec::new(),
            memory_usage: widgets::MemoryUsage::default(),
            weather_client: reqwest::blocking::Client::new(),
        }
    }
}

// Some implementation
impl WidgetApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn need_update(&mut self) -> bool {
        if self.last_update.elapsed() > self.update_interval {
            true
        } else {
            false
        }
    }

    fn weather_need_update(&mut self) -> bool {

    }
}

// App implementation
impl eframe::App for WidgetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Widgets");
            
            if self.need_update() {
                self.cpu_usage = widgets::cpu_usage(&mut self.system);
                self.memory_usage = widgets::memory_usage(&mut self.system);
                self.disks.refresh();
                widgets::cpu_usage_history(&mut self.system, &mut self.cpu_usage_history);
                widgets::memory_usage_history(&mut self.system, &mut self.memory_usage_history);
                self.last_update = time::Instant::now();
            }

            // Cpu Usage
            egui::Window::new("Cpu Usage").show(ctx, |ui| {
                for (i, cpu) in self.cpu_usage.iter().enumerate() {
                    ui.label(format!("Cpu {} usage: {:.2}%", i, cpu));
                }
            });

            // Memory Usage
            egui::Window::new("Memory Usage").show(ctx, |ui| {

                // TODO: beautiful groups
                ui.label(format!(
                    "Total memory: {:.2} MB",
                    self.memory_usage.total_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Used memory: {:.2} MB",
                    self.memory_usage.used_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Free memory: {:.2} MB",
                    self.memory_usage.free_memory as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Total swap: {:.2} MB",
                    self.memory_usage.total_swap as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Used swap: {:.2} MB",
                    self.memory_usage.used_swap as f64 / 1_048_576.0
                ));
                ui.label(format!(
                    "Free swap: {:.2} MB",
                    self.memory_usage.free_swap as f64 / 1_048_576.0
                ))
            });

            // Disk Usage
            egui::Window::new("Disk Usage").show(ctx, |ui| {
                for disc in &self.disks {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} {}",
                            disc.name().to_string_lossy(),
                            disc.mount_point().to_string_lossy()
                        ));
                        ui.vertical(|ui| {
                            ui.label(format!(
                                "Available: {:.2} MB",
                                disc.available_space() as f64 / 1_048_576.0
                            ));
                            ui.label(format!(
                                "Total: {:.2} MB",
                                disc.total_space() as f64 / 1_048_576.0
                            ));
                        })
                    });
                }
            });

            // Cpu Usage Plot
            egui::Window::new("Cpu Usage Plot").show(ctx, |ui| {
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

            // Memory Usage Plot
            egui::Window::new("Memory Usage Plot").show(ctx, |ui| {
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

            // Clock
            egui::Window::new("Clock").show(ctx, |ui| {
                let dt = Local::now();
                ui.label(format!(
                    "Time: {}, Date: {}",
                    dt.time().format("%H:%M:%S"),
                    dt.date_naive()
                ));
            });

            // Weather
            egui::Window::new("Weather").show(ctx, |ui| {
                let weather = widgets::get_weather(&mut self.weather_client);
                ui.label(format!("Temperature: {:.2} °C", weather.current.temperature_2m));
                ui.label(format!("Feels like: {:.2} °C", weather.current.apparent_temperature));
                ui.label(format!("Humidity: {:.2} %", weather.current.relative_humidity_2m));
                ui.label(format!("Cloud cover: {:.2} %", weather.current.cloud_cover));
                ui.label(format!("Is day: {}", weather.current.is_day));
                ui.label(format!("Precipitation: {:.2} mm", weather.current.precipitation));
                ui.label(format!("Rain: {:.2}", weather.current.rain));
                ui.label(format!("Showers: {:.2}", weather.current.showers));
                ui.label(format!("Snowfall: {:.2}", weather.current.snowfall));
                ui.label(format!("Weather code: {}", weather.current.weather_code));
                ui.label(format!("Wind speed: {:.2} m/s", weather.current.wind_speed_10m));
                ui.label(format!("Wind direction: {}", weather.current.wind_direction_10m));
                ui.label(format!("Wind gusts: {:.2} m/s", weather.current.wind_gusts_10m));
                ui.label(format!("Pressure: {:.2} hPa", weather.current.surface_pressure));
            });
        });
    }
}
