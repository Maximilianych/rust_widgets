use std::collections::VecDeque;
use sysinfo::System;

pub mod prelude {
    pub use super::{memory_usage, memory_usage_history, MemoryUsage};
}

#[derive(Default)]
pub struct MemoryUsage {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
}

pub fn memory_usage_history(system: &mut System, memory_usage_history: &mut VecDeque<f32>) {
    system.refresh_memory();

    memory_usage_history.push_back(system.used_memory() as f32 / 1_048_576.0);
    if memory_usage_history.len() > 100 {
        memory_usage_history.pop_front();
    }
}

pub fn memory_usage(system: &mut System) -> MemoryUsage {
    system.refresh_memory();

    MemoryUsage {
        total_memory: system.total_memory(),
        free_memory: system.free_memory(),
        used_memory: system.used_memory(),
        total_swap: system.total_swap(),
        used_swap: system.used_swap(),
        free_swap: system.free_swap(),
    }
}
