mod register;
mod server;

use register::*;
use server::*;

use crate::conf::*;

pub async fn start_serve(conf: Conf) {
    let mut register = Register::new(conf.register_conf).await;
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
