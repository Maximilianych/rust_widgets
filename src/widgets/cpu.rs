use std::collections::VecDeque;
use sysinfo::System;

pub mod prelude {
    pub use super::cpu_usage;
    pub use super::cpu_usage_history;
}

pub fn cpu_usage_history(system: &mut System, cpu_usage_history: &mut VecDeque<f32>) {
    let cpu_usage: f32 = cpu_usage(system).iter().sum::<f32>() / system.cpus().len() as f32;
    cpu_usage_history.push_back(cpu_usage);
    if cpu_usage_history.len() > 100 {
        cpu_usage_history.pop_front();
    };
}

pub fn cpu_usage(system: &mut System) -> Vec<f32> {
    let mut cpu_usage = Vec::new();

    system.refresh_cpu_usage();

    for cpu in system.cpus() {
        cpu_usage.push(cpu.cpu_usage());
    }

    cpu_usage
}
