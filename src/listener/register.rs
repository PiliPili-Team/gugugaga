use google_drive3::DriveHub;
use google_drive3::api::Channel;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use std::{env::home_dir, path::Path};
use yup_oauth2::*;

pub struct Register {
    hub: DriveHub<HttpsConnector<HttpConnector>>,
    current_channel: Option<Channel>,
}

impl Register {
    const SCOPE: &'static str = "https://www.googleapis.com/auth/drive.metadata.readonly";

    async fn new() -> Self {
        tracing::info!("Initializing Google Drive Notification Listener");
        let secret_dir = home_dir()
            .expect("Failed to get home directory")
            .join(".gugugaga");
        let secret_path = Path::new(&secret_dir).join("client_secret.json");

        if !secret_path.exists() {
            panic!("client_secret.json not found at {:?}", secret_path);
        }

        let secret = read_application_secret(&secret_path)
            .await
            .unwrap_or_else(|e| panic!("{}", e.to_string()));

        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .build()
                .await
                .unwrap_or_else(|e| panic!("{}", e.to_string()));

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .expect("Failed to create HttpsConnector")
                        .https_or_http()
                        .enable_http2()
                        .build(),
                );

        let hub = DriveHub::new(client, auth);

        tracing::info!("Google Drive Notification Listener initialized successfully");

        Self {
            hub,
            current_channel: None,
        }
    }

    async fn register_channel(&mut self) {
        tracing::info!("Registering channel for Google Drive notifications");
        let req = self
            .hub
            .files()
            .watch(Channel::default(), "pageToken")
            .doit()
            .await;

        match req {
            Ok((response, channel)) => {
                tracing::info!("Channel registered successfully: {:?}", channel);
                self.current_channel = Some(channel);
            }
            Err(e) => {
                tracing::error!("Error registering channel: {:?}", e);
                panic!("Failed to register channel");
            }
        }
    }

    async fn remove_channel(&mut self) {
        let Some(channel) = self.current_channel.take() else {
            tracing::warn!("No current client to remove channel for");
            return;
        };

        tracing::info!("Removing last channel...");
        let req = self.hub.channels().stop(channel).doit().await;

        match req {
            Ok(_) => {
                tracing::info!("Channel removed successfully");
            }
            Err(e) => {
                tracing::error!("Error removing channel: {:?}", e);
                panic!("Failed to remove channel");
            }
        }
    }

    async fn try_renew_channel(&mut self) {
        // TODO: awaits should return Result<>
        tracing::info!("Renewing channel...");
        self.remove_channel().await;
        self.register_channel().await;
    }
}
