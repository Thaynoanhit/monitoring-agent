use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use crate::agent::system_data::{ExtendedSystemData, collect_extended_metrics};
use log::{info, error};

pub struct MetricsRotation {
    max_entries: usize,
    metrics: Arc<Mutex<Vec<ExtendedSystemData>>>,
}

impl MetricsRotation {
    pub fn new(max_entries: usize) -> Self {
        MetricsRotation {
            max_entries,
            metrics: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_metric(&self, metric: ExtendedSystemData) {
        let mut metrics = self.metrics.lock().unwrap();
        
        metrics.push(metric);
        
        if metrics.len() > self.max_entries {
            metrics.remove(0);
        }
    }

    pub fn get_metrics(&self) -> Vec<ExtendedSystemData> {
        let metrics = self.metrics.lock().unwrap();
        metrics.clone()
    }
}

pub async fn collect_system_metrics(
    tx: mpsc::Sender<ExtendedSystemData>, 
    rotation: Arc<MetricsRotation>,
    interval_secs: u64,
) {
    let mut interval = time::interval(Duration::from_secs(interval_secs));
    
    loop {
        match collect_metrics_with_logging(tx.clone(), rotation.clone()) {
            Ok(_) => {},
            Err(e) => error!("Error collecting metrics: {}", e),
        }
        interval.tick().await;
    }
}

fn collect_metrics_with_logging(
    tx: mpsc::Sender<ExtendedSystemData>,
    rotation: Arc<MetricsRotation>,
) -> Result<(), String> {
    let data = collect_extended_metrics();
    
    info!(
        "Collected Metrics - CPU: {:.1}%, Mem: {:.1}/{:.1} GB, Threads: {}/{}",
        data.cpu_usage,
        data.used_memory as f64 / 1024.0 / 1024.0 / 1024.0,
        data.total_memory as f64 / 1024.0 / 1024.0 / 1024.0,
        data.thread_metrics.active_threads,
        data.thread_metrics.total_threads
    );
    
    let data_for_channel = data.clone();
    
    tokio::spawn(async move {
        if let Err(e) = tx.send(data_for_channel).await {
            error!("Failed to send metrics: {}", e);
        }
    });
    
    rotation.add_metric(data);
    
    Ok(())
}
