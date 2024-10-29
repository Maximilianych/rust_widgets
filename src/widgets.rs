pub mod cpu;

use sysinfo::System;

pub fn cpu_usage(system: &mut System) -> Vec<f32> {
    cpu::cpu_usage(system)
}
