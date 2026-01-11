mod args;
mod conf;
mod listener;
mod notifier;

use args::*;
use clap::Parser;
use listener::*;
use notifier::*;

pub use conf::*;

pub async fn run() {
    Args::parse().init();
    
    let conf = Conf::load_or_create().unwrap_or_else(|e| {
        tracing::error!("{}", e.to_string());
        panic!("Failed to load or create config");
    });
    
    start_serve(conf).await;
}
