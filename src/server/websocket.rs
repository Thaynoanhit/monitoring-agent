use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use tokio::time::{interval, Duration};
use std::sync::Arc;
use crate::agent::data_collector::MetricsRotation;
use log::error;

pub async fn handle_ws_connection(ws: WebSocket, metrics: Arc<MetricsRotation>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let metrics_data = metrics.get_metrics();
                if let Some(latest_metric) = metrics_data.last() {
                    match serde_json::to_string(&latest_metric) {
                        Ok(json) => {
                            if let Err(e) = ws_tx.send(Message::text(json)).await {
                                error!("Erro ao enviar mensagem WebSocket: {}", e);
                                break;
                            }
                        },
                        Err(e) => {
                            error!("Erro ao serializar métricas: {}", e);
                        }
                    }
                }
            }
            Some(result) = ws_rx.next() => {
                match result {
                    Ok(_) => (), // Ignora mensagens recebidas do cliente
                    Err(_) => break, // Conexão fechada ou erro
                }
            }
            else => break,
        }
    }
}