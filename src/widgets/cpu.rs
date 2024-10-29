use sysinfo::System;

pub fn cpu_usage() {
    let mut sys = System::new();
    loop {
        sys.refresh_cpu_usage();
        for cpu in sys.cpus() {
            println!("{}%", cpu.cpu_usage());
        }
    }
}