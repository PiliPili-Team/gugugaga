mod register;
mod server;

use register::*;
use server::*;

pub async fn start_serve() {
    let mut register = Register::new().await;
    let mut server = Server::new().await;

    let register_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600 * 24));
        loop {
            register.try_renew_channel().await;
            interval.tick().await;
        }
    });
    
    tokio::select! {
        res = server.run() => {
            if let Err(e) = res {
                tracing::error!("Server error: {:?}", e);
            }
        }
        _ = register_handle => {
            tracing::info!("Register task ended");
        }
    }
}
