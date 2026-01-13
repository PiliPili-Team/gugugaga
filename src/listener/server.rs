use std::panic;

use axum::Router;
use axum::routing::*;
use tokio::net::TcpListener;

const SERVER_PORT: u16 = 6933;

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    const NOTIFY_PATH: &'static str = "/notification";
    const PLACEHOLDER: &'static str = "/reserved";

    async fn on_notify() {

    }

    async fn on_placeholder() {

    }

    pub async fn new() -> Self {
        tracing::info!("Starting server on port {}", SERVER_PORT);

        let router = Router::new()
        .route(Self::NOTIFY_PATH, post(Self::on_notify))
        .route(Self::PLACEHOLDER, get(Self::on_placeholder));

        let listener = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to bind to port {}: {}", SERVER_PORT, e);
                panic!("Failed to bind to port {}", SERVER_PORT);
            });
        Server { router, listener }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router.clone()).await?;
        Ok(())
    }
}
