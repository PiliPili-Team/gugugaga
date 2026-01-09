mod args;
mod conf;
mod listener;

use args::*;
use clap::Parser;
use listener::*;

pub async fn start_serve() {
    Args::parse().init();

    if let Err(e) = Listener::new().await.serve().await {
        tracing::error!("Error occurred: {:?}", e);
    };
}
