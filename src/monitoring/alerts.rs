// Novo arquivo: src/monitoring/alerts.rs
pub struct AlertConfig {
    cpu_threshold: f32,
    memory_threshold: f32,
    disk_threshold: f32,
    thread_threshold: usize,
}

impl AlertConfig {
    pub fn check_metrics(&self, data: &ExtendedSystemData) -> Vec<Alert> {
        let mut alerts = Vec::new();
        
        if data.cpu_usage > self.cpu_threshold {
            alerts.push(Alert::new(
                AlertType::CpuUsage,
                format!("CPU usage critical: {}%", data.cpu_usage)
            ));
        }
        
        let memory_usage = data.used_memory as f32 / data.total_memory as f32;
        if memory_usage > self.memory_threshold {
            alerts.push(Alert::new(
                AlertType::MemoryUsage,
                format!("Memory usage critical: {}%", memory_usage * 100.0)
            ));
        }
        
        if data.thread_metrics.total_threads > self.thread_threshold {
            alerts.push(Alert::new(
                AlertType::ThreadCount,
                format!("High thread count: {}/{} threads", 
                    data.thread_metrics.active_threads,
                    data.thread_metrics.total_threads
                )
            ));
        }
        
        alerts
    }
}