use sysinfo::Disks;

pub mod prelude {
    pub use super::disk_usage;
}

pub fn disk_usage(disks: &mut Disks) -> Vec<&mut sysinfo::Disk> {
    let mut disc_usage = Vec::new();

    disks.refresh();

    for disk in disks {
        disc_usage.push(disk);
    }

    disc_usage
}