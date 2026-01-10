use std::panic;

use axum::Router;
use tokio::net::TcpListener;

use crate::notifier::on_notify;
const SERVER_PORT: u16 = 6933;
const NOTIFY_PATH: &str = "/notification";

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Self {
        tracing::info!("Starting server on port {}", SERVER_PORT);

        let router = Router::new().route(NOTIFY_PATH, axum::routing::post(on_notify));

        let listener = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to bind to port {}: {}", SERVER_PORT, e);
                panic!("Failed to bind to port {}", SERVER_PORT);
            });
        Server { router, listener }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        todo!()
    }
}
