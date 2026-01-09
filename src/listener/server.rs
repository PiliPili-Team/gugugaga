use axum::Router;
use tokio::net::TcpListener;
const SERVER_PORT: u16 = 6933;

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Self {
        tracing::info!("Starting server on port {}", SERVER_PORT);

        let router = Router::new();

        let listener = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))
            .await
            .expect("Failed to bind to address");
        Server { router, listener }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
