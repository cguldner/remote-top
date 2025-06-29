//! Binary that collects information about the
//! currently running system and sends it to the server

use remote_top::{SystemInformation, create_command_line_arg_parser};
use std::{
    net::{Ipv4Addr, SocketAddr},
    thread::sleep,
    time::Duration,
};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = create_command_line_arg_parser(
        "Client that will gather system utilization metrics and send to a server".to_string(),
    );

    let refresh_params = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::everything())
        .with_cpu(CpuRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh_params);
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

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

        println!("{:?}", serde_json::to_string(&info)?);
        let client = reqwest::Client::new();
        let res = client
            .post(format!("http://{addr}/post"))
            .json(&info)
            .send()
            .await?;

        println!("{:?}", res);

        sleep(Duration::from_secs(1));
    }
}
