mod args;
mod conf;
mod listener;
mod notifier;

use args::*;
use clap::Parser;
use listener::*;
use notifier::*;

pub async fn run() {
    Args::parse().init();
    
    start_serve().await;
}
