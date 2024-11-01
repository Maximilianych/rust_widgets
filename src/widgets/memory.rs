use sysinfo::System;

pub mod prelude {
    pub use super::{memory_usage, MemoryUsage};
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
