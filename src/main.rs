use std::sync::Arc;
use tokio::sync::mpsc;
use dotenv::dotenv;
use std::env;
use tokio::sync::Semaphore;

mod agent;
mod server;

use agent::data_collector::{MetricsRotation, collect_system_metrics};
use server::endpoints::metrics_endpoints;
use crate::agent::logging::setup_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let semaphore = Arc::new(Semaphore::new(100));
    
    let max_metrics = env::var("MAX_METRICS")
        .unwrap_or("1000".to_string())
        .parse::<usize>()?;
    
    let interval_secs = env::var("COLLECT_INTERVAL")
        .unwrap_or("10".to_string())
        .parse::<u64>()?;

    setup_logging()?;

    let (tx, _rx) = mpsc::channel(100);
    let metrics_rotation = Arc::new(MetricsRotation::new(max_metrics));
    let metrics_rotation_clone = metrics_rotation.clone();

    tokio::spawn(collect_system_metrics(
        tx, 
        metrics_rotation_clone, 
        interval_secs,
    ));

    let routes = metrics_endpoints(metrics_rotation, semaphore);

    println!("Monitoring agent running on http://127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}