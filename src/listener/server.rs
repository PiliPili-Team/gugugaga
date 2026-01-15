use std::panic;

use axum::{
    Router,
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::*,
};
use tokio::net::TcpListener;

use crate::conf::ServerConf;

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    const NOTIFY_PATH: &'static str = "/notification";
    const PLACEHOLDER: &'static str = "/reserved";

    async fn on_notify(headers: HeaderMap, body: Bytes) -> Result<impl IntoResponse, StatusCode> {
        let header_value = |name: &str| -> Result<&str, StatusCode> {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .ok_or(StatusCode::BAD_REQUEST)
        };

        let channel_id = header_value("X-Goog-Channel-ID")?;
        let resource_id = header_value("X-Goog-Resource-ID")?;
        let resource_state = header_value("X-Goog-Resource-State")?;
        let resource_uri = header_value("X-Goog-Resource-URI")?;
        let message_number = header_value("X-Goog-Message-Number")?;
        let changed = header_value("X-Goog-Changed").ok();

        tracing::info!(
            "Received Drive notification: channel_id={channel_id}, resource_id={resource_id}, state={resource_state}, msg_no={message_number}, uri={resource_uri}, body_bytes={}",
            body.len(),
        );

        if let Some(changed) = changed {
            tracing::debug!("Changed fields: {changed}");
        }

        Ok(StatusCode::OK)
    }

    async fn on_placeholder() -> impl IntoResponse {
        StatusCode::OK
    }

    pub async fn new(conf: ServerConf) -> Self {
        tracing::info!("Starting server on port {}", conf.port);

        let router = Router::new()
            .route(Self::NOTIFY_PATH, post(Self::on_notify))
            .route(Self::PLACEHOLDER, get(Self::on_placeholder));

        let listener = TcpListener::bind(format!("0.0.0.0:{}", conf.port))
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Failed to bind to port {}: {}", conf.port, e);
                panic!("Failed to bind to port {}", conf.port);
            });
        Server { router, listener }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router.clone()).await?;
        Ok(())
    }
}
