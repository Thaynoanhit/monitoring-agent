
use tokio::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

pub struct Agent {
    id: String,
    interval: Duration,
    tx: mpsc::Sender<ExtendedSystemData>,
    metrics_rotation: Arc<MetricsRotation>,
}

impl Agent {
    pub fn new(
        id: String,
        interval_secs: u64,
        tx: mpsc::Sender<ExtendedSystemData>,
        metrics_rotation: Arc<MetricsRotation>,
    ) -> Self {
        Self {
            id,
            interval: Duration::from_secs(interval_secs),
            tx,
            metrics_rotation,
        }
    }

    pub async fn start(&self) {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            let data = collect_extended_metrics();
            data.agent_id = Some(self.id.clone());
            
            if let Err(e) = self.tx.send(data.clone()).await {
                error!("Agent {} failed to send metrics: {}", self.id, e);
            }
            
            self.metrics_rotation.add_metric(data);
        }
    }
}