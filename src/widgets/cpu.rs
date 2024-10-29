use sysinfo::System;

pub fn cpu_usage(system: &mut System) -> Vec<f32> {

    let mut cpu_usage = Vec::new();

    system.refresh_cpu_usage();

    for cpu in system.cpus() {
        cpu_usage.push(cpu.cpu_usage());
    }

    cpu_usage
}
