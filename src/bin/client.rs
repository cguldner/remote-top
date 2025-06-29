//! Binary that collects information about the
//! currently running system and sends it to the server

use std::{thread::sleep, time::Duration};

use remote_top::SystemInformation;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

fn main() {
    let refresh_params = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::everything())
        .with_cpu(CpuRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh_params);

    loop {
        sys.refresh_specifics(refresh_params);

        // println!("total memory: {} bytes", sys.total_memory());
        // println!("used memory: {} bytes", sys.used_memory());
        // for cpu in sys.cpus() {
        //     print!("{}% ", cpu.cpu_usage());
        // }
        // println!();

        let info = SystemInformation {
            ram_usage: sys.used_memory(),
        };

        println!("{:?}", info);

        sleep(Duration::from_secs(1));
    }
}
