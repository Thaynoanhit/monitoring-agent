use warp::Filter;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::agent::data_collector::MetricsRotation;
use warp::ws::Ws;
use crate::server::websocket::handle_ws_connection;

pub fn metrics_endpoints(
    metrics_rotation: Arc<MetricsRotation>,
    _semaphore: Arc<Semaphore>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .build();

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_metrics(metrics_rotation.clone()))
        .map(|ws: Ws, metrics: Arc<MetricsRotation>| {
            ws.on_upgrade(move |socket| handle_ws_connection(socket, metrics))
        });

    ws_route.with(cors)
}

fn with_metrics(
    metrics_rotation: Arc<MetricsRotation>,
) -> impl Filter<Extract = (Arc<MetricsRotation>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&metrics_rotation))
}
