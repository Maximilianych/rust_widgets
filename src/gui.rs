use chrono::Local;
use egui_plot::{Line, Plot};
use std::collections::VecDeque;
use std::time;
use sysinfo::{Disks, System};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winapi::{shared::windef::HWND, um::winuser::{GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, GWL_STYLE, HWND_BOTTOM, HWND_TOPMOST, LWA_COLORKEY, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_BORDER, WS_CAPTION, WS_EX_TRANSPARENT, WS_THICKFRAME}};

use crate::widgets;

// WidgetApp
pub struct WidgetApp {
    window_handle: Option<RawWindowHandle>,
    system: System,
    disks: Disks,
    cpu_usage_history: VecDeque<f32>,
    memory_usage_history: VecDeque<f32>,
    last_update: time::Instant,
    update_interval: time::Duration,
    cpu_usage: Vec<f32>,
    memory_usage: widgets::MemoryUsage,
    weather_client: reqwest::blocking::Client,
    weather: widgets::Weather,
    weather_last_update: time::Instant,
    weather_update_interval: time::Duration,
}

// Default implementation
impl Default for WidgetApp {
    fn default() -> Self {
        Self {
            window_handle: None,
            last_update: time::Instant::now()
                .checked_sub(time::Duration::from_secs(1))
                .unwrap(),
            update_interval: time::Duration::from_secs(1),
            system: System::new(),
            disks: Disks::new_with_refreshed_list(),
            cpu_usage_history: VecDeque::with_capacity(100),
            memory_usage_history: VecDeque::with_capacity(100),
            cpu_usage: Vec::new(),
            memory_usage: widgets::MemoryUsage::default(),
            weather_client: reqwest::blocking::Client::new(),
            weather: widgets::Weather::default(),
            weather_last_update: time::Instant::now()
                .checked_sub(time::Duration::from_secs(60 * 5))
                .unwrap(),
            weather_update_interval: time::Duration::from_secs(60 * 5),
        }
    }
}

// Some implementation
impl WidgetApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let window_handle = cc.window_handle().unwrap().as_raw();
       
        // FUCK FUCK FUCK
        // match window_handle {
        //     RawWindowHandle::Win32(window_handle) => {
        //         let hwnd = window_handle.hwnd.get() as HWND;
        //         eprintln!("Hwnd: {:?}", hwnd);
        //         unsafe {
        //             SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE );
        //             let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        //             SetWindowLongPtrW(hwnd, GWL_STYLE, style &!(WS_CAPTION | WS_BORDER | WS_THICKFRAME) as isize);
        //             SetLayeredWindowAttributes(hwnd, 0, 0, LWA_COLORKEY);
        //             SetWindowLongPtrW(hwnd, GWL_EXSTYLE, GetWindowLongPtrW(hwnd, GWL_EXSTYLE) | WS_EX_TRANSPARENT as isize);
        //         }
        //     }
        //     _ => {}
        // };

        Self {
            window_handle: Some(window_handle),
            ..Default::default()
        }
    }

    pub fn need_update(&mut self) -> bool {
        if self.last_update.elapsed() > self.update_interval {
            true
        } else {
            false
        }
    }

    fn weather_need_update(&mut self) -> bool {
        if self.weather_last_update.elapsed() > self.weather_update_interval {
            true
        } else {
            false
        }
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

                if self.weather_need_update() {
                    self.weather = widgets::weather_request(&mut self.weather_client);
                    self.weather_last_update = time::Instant::now();
                }
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
                ui.label(format!(
                    "Temperature: {:.2} {}",
                    self.weather.current.temperature_2m, self.weather.current_units.temperature_2m
                ));
                ui.label(format!(
                    "Feels like: {:.2} {}",
                    self.weather.current.apparent_temperature,
                    self.weather.current_units.apparent_temperature
                ));
                ui.label(format!(
                    "Humidity: {:.2} {}",
                    self.weather.current.relative_humidity_2m,
                    self.weather.current_units.relative_humidity_2m
                ));
                ui.label(format!(
                    "Cloud cover: {:.2} {}",
                    self.weather.current.cloud_cover, self.weather.current_units.cloud_cover
                ));
                ui.label(format!(
                    "Current time: {}",
                    if self.weather.current.is_day == 1 {
                        "day"
                    } else {
                        "night"
                    }
                ));
                ui.label(format!(
                    "Precipitation: {:.2} {}",
                    self.weather.current.precipitation, self.weather.current_units.precipitation
                ));
                ui.label(format!(
                    "Rain: {:.2} {}",
                    self.weather.current.rain, self.weather.current_units.rain
                ));
                ui.label(format!(
                    "Showers: {:.2} {}",
                    self.weather.current.showers, self.weather.current_units.showers
                ));
                ui.label(format!(
                    "Snowfall: {:.2} {}",
                    self.weather.current.snowfall, self.weather.current_units.snowfall
                ));
                // TODO: add different guis for different weather_code
                ui.label(format!(
                    "Weather code: {}",
                    self.weather.current.weather_code
                ));
                ui.label(format!(
                    "Wind direction: {} ({})",
                    widgets::degrees_to_direction(self.weather.current.wind_direction_10m),
                    self.weather.current.wind_direction_10m
                ));
                ui.label(format!(
                    "Wind speed: {:.2} {}",
                    self.weather.current.wind_speed_10m, self.weather.current_units.wind_speed_10m
                ));
                ui.label(format!(
                    "Wind gusts: {:.2} {}",
                    self.weather.current.wind_gusts_10m, self.weather.current_units.wind_gusts_10m
                ));
                ui.label(format!(
                    "Pressure: {:.2} {}",
                    self.weather.current.surface_pressure,
                    self.weather.current_units.surface_pressure
                ));
            });
        });
        ctx.request_repaint_after(time::Duration::from_millis(100));
    }
}
