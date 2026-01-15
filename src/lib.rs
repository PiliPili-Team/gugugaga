mod args;
mod conf;
mod listener;
mod notifier;
mod db;

use args::*;
use clap::Parser;
use listener::*;
use notifier::*;

pub use conf::*;
pub use db::DB;

pub async fn run() {
    Args::parse().init();
    
    let conf = Conf::load_or_create().unwrap_or_else(|e| {
        tracing::error!("{}", e.to_string());
        panic!("Failed to load or create config");
    });
    
    start_serve(conf).await;
}
