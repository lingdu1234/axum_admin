use serde::Serialize;

#[derive(Debug, Serialize, Default, Clone)]
pub struct SysInfo {
    pub server: Server,
    pub cpu: Cpu,
    pub cpu_load: CpuLoad,
    pub memory: Memory,
    pub process: Process,
    pub network: Vec<Network>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Cpu {
    pub name: String,
    pub arch: String,
    pub processors: usize,
    pub frequency: u64,
    pub cores: String,
    pub total_use: f32,
}
#[derive(Debug, Serialize, Default, Clone)]
pub struct CpuLoad {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Memory {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Server {
    pub oper_sys_name: String,
    pub host_name: String,
    pub system_version: String,
    pub system_kerne: String,
}
#[derive(Debug, Serialize, Default, Clone)]
pub struct Process {
    pub name: String,
    pub used_memory: u64,
    pub used_virtual_memory: u64,
    pub cup_usage: f32,
    pub start_time: u64,
    pub run_time: u64,
    pub disk_usage: DiskUsage,
}
#[derive(Debug, Serialize, Default, Clone)]
pub struct DiskUsage {
    pub read_bytes: u64,
    pub total_read_bytes: u64,
    pub written_bytes: u64,
    pub total_written_bytes: u64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Network {
    pub name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
}
