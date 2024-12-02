use serde::{Serialize, Deserialize};
use sysinfo::{
    System, SystemExt, CpuExt, DiskExt, NetworkExt, ProcessExt, NetworksExt, PidExt,
};
use std::time::SystemTime;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use num_cpus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedSystemData {
    pub timestamp: u64,
    pub agent_id: Option<String>,
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub disk_usage: Vec<DiskIOMetrics>,
    pub network_usage: Vec<NetworkMetrics>,
    pub top_processes: Vec<ProcessMetrics>,
    pub system_load: LoadAverages,
    pub thread_metrics: ThreadMetrics
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskIOMetrics {
    pub nome: String,
    pub uso: f32,
    pub total: u64,
    pub disponivel: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkMetrics {
    pub interface_name: String,
    pub bytes_received: u64,
    pub bytes_sent: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessMetrics {
    pub pid: usize,
    pub name: String,
    pub cpu_usage: f64,
    pub memory_usage: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadAverages {
    pub one_minute: f64,
    pub five_minutes: f64,
    pub fifteen_minutes: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDataGB {
    pub timestamp: u64,
    pub cpu_usage: f64,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub disk_usage: Vec<DiskIOMetricsGB>,
    pub network_usage: Vec<NetworkMetrics>,
    pub top_processes: Vec<ProcessMetrics>,
    pub system_load: LoadAverages,
    pub thread_metrics: ThreadMetrics
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskIOMetricsGB {
    pub nome: String,
    pub uso: f64,
    pub total: f64,
    pub disponivel: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadMetrics {
    pub total_threads: usize,
    pub active_threads: usize,
    pub thread_per_core: f32,
    pub thread_details: Vec<ThreadDetail>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadDetail {
    pub process_name: String,
    pub thread_count: usize,
    pub cpu_usage: f32
}

fn format_percentage(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn collect_extended_metrics() -> ExtendedSystemData {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disk_usage = sys.disks()
        .iter()
        .map(|disk| {
            let name = disk.name().to_string_lossy().to_string();
            DiskIOMetrics {
                nome: name.clone(),
                uso: 100.0 * (disk.total_space() - disk.available_space()) as f32 
                    / disk.total_space() as f32,
                total: disk.total_space(),
                disponivel: disk.available_space(),
            }
        })
        .collect();

    let mut total_threads = 0;
    let mut active_threads = 0;
    let mut thread_details = Vec::new();

    for process in sys.processes().values() {
        let thread_count = match File::open(format!("/proc/{}/stat", process.pid().as_u32())) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    contents.split_whitespace()
                        .nth(19)
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(1)
                } else {
                    1
                }
            },
            Err(_) => 1
        };
        
        total_threads += thread_count;
        
        if process.cpu_usage() > 0.1 {
            active_threads += thread_count;
        }

        thread_details.push(ThreadDetail {
            process_name: process.name().to_string(),
            thread_count,
            cpu_usage: process.cpu_usage()
        });
    }

    ExtendedSystemData {
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        agent_id: None,
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        disk_usage,
        network_usage: sys.networks()
            .iter()
            .map(|(name, network)| NetworkMetrics {
                interface_name: name.to_string(),
                bytes_received: network.received(),
                bytes_sent: network.transmitted(),
            })
            .collect(),
        top_processes: sys.processes()
            .values()
            .filter(|process| process.cpu_usage() > 0.0)
            .map(|process| ProcessMetrics {
                pid: process.pid().as_u32() as usize,
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage() as f64,
                memory_usage: process.memory(),
            })
            .sorted_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap())
            .take(10)
            .collect(),
        system_load: LoadAverages {
            one_minute: sys.load_average().one,
            five_minutes: sys.load_average().five,
            fifteen_minutes: sys.load_average().fifteen,
        },
        thread_metrics: ThreadMetrics {
            total_threads,
            active_threads,
            thread_per_core: total_threads as f32 / num_cpus::get() as f32,
            thread_details
        }
    }
}

impl From<ExtendedSystemData> for SystemDataGB {
    fn from(data: ExtendedSystemData) -> Self {
        const GB: f64 = 1024.0 * 1024.0 * 1024.0;
        
        SystemDataGB {
            timestamp: data.timestamp,
            cpu_usage: data.cpu_usage as f64,
            total_memory_gb: data.total_memory as f64 / GB,
            used_memory_gb: data.used_memory as f64 / GB,
            disk_usage: data.disk_usage
                .into_iter()
                .filter(|disk| disk.nome.starts_with("/dev/"))
                .map(|disk| DiskIOMetricsGB {
                    nome: disk.nome,
                    uso: disk.uso as f64,
                    total: disk.total as f64 / GB,
                    disponivel: disk.disponivel as f64 / GB,
                })
                .collect(),
            network_usage: data.network_usage,
            top_processes: data.top_processes.into_iter()
                .map(|proc| ProcessMetrics {
                    pid: proc.pid,
                    name: proc.name,
                    cpu_usage: format_percentage(proc.cpu_usage),
                    memory_usage: proc.memory_usage,
                })
                .collect(),
            system_load: LoadAverages {
                one_minute: format_percentage(data.system_load.one_minute),
                five_minutes: format_percentage(data.system_load.five_minutes),
                fifteen_minutes: format_percentage(data.system_load.fifteen_minutes),
            },
            thread_metrics: data.thread_metrics
        }
    }
}
