use db::system::models::server_info::{Cpu, CpuLoad, DiskUsage, Memory, Network, Process, Server, SysInfo};
use sysinfo::{Networks, System};

pub fn get_oper_sys_info() -> SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    let pid = sysinfo::get_current_pid().expect("failed to get PID");
    let server = Server {
        oper_sys_name: System::name().unwrap_or_else(|| "unknown".to_owned()),
        host_name: System::host_name().unwrap_or_else(|| "unknown".to_owned()),
        system_version: System::long_os_version().unwrap_or_else(|| "unknown".to_owned()),
        system_kerne: System::kernel_version().unwrap_or_else(|| "unknown".to_owned()),
    };
    let process = match sys.process(pid) {
        Some(p) => Process {
            name: format!("{:?}", p.name()),
            used_memory: p.memory(),
            used_virtual_memory: p.virtual_memory(),
            cup_usage: p.cpu_usage(),
            start_time: p.start_time(),
            run_time: p.run_time(),
            disk_usage: DiskUsage {
                read_bytes: p.disk_usage().read_bytes,
                total_read_bytes: p.disk_usage().total_read_bytes,
                written_bytes: p.disk_usage().written_bytes,
                total_written_bytes: p.disk_usage().total_written_bytes,
            },
        },
        None => Process { ..Default::default() },
    };

    let mut network: Vec<Network> = Vec::new();
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        network.push(Network {
            name: interface_name.to_string(),
            received: data.received(),
            total_received: data.total_received(),
            transmitted: data.transmitted(),
            total_transmitted: data.total_transmitted(),
        });
    }
    // todo: 这里应该要重写
    let cpu = Cpu {
        name: sys.cpus().first().unwrap().name().to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cores: sys.physical_core_count().map(|c| c.to_string()).unwrap_or_else(|| "Unknown".to_owned()),
        total_use: sys.global_cpu_usage(),
        frequency: sys.cpus().first().unwrap().frequency(),
        processors: sys.physical_core_count().unwrap() as usize,
    };
    let load_avg = System::load_average();
    let cpu_load = CpuLoad {
        one: load_avg.one,
        five: load_avg.five,
        fifteen: load_avg.fifteen,
    };
    let memory = Memory {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };

    SysInfo {
        server,
        cpu,
        memory,
        process,
        network,
        cpu_load,
    }
}
