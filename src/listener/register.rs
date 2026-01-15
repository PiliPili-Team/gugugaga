use anyhow::Result;
use google_drive3::DriveHub;
use google_drive3::api::Channel;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use std::path::Path;
use yup_oauth2::*;

use crate::{DB, conf::RegisterConf};

pub struct Register {
    hub: DriveHub<HttpsConnector<HttpConnector>>,
    register_conf: RegisterConf,
}

impl Register {
    const SCOPE: &'static str = "https://www.googleapis.com/auth/drive.metadata.readonly";
    const CHANNEL_ID: &'static str = "gugugaga-notification-channelv2";

    pub async fn new(conf: RegisterConf) -> Self {
        tracing::info!("Initializing Google Drive Notification Listener");
        let secret_dir = dirs::config_dir()
            .expect("Failed to get home directory")
            .join("gugugaga");
        let secret_path = Path::new(&secret_dir).join("client_secret.json");

        if !secret_path.exists() {
            panic!("client_secret.json not found at {:?}", secret_path);
        }

        let secret = read_application_secret(&secret_path)
            .await
            .unwrap_or_else(|e| panic!("{}", e.to_string()));

        // Since we wont implement a web server to handle the redirect, this program should be run
        // firstly in a GUI environment to complete the OAuth2 flow.
        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(secret_dir.join("token_cache.json"))
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
            register_conf: conf,
        }
    }

    async fn register_channel(&mut self) -> Result<Channel> {
        tracing::info!("Registering channel for Google Drive notifications");
        let channel = Channel {
            address: Some(self.register_conf.address.clone()),
            id: Some(Self::CHANNEL_ID.to_string()),
            type_: Some("webhook".to_string()),
            ..Default::default()
        };

        let channel = self
            .hub
            .changes()
            .watch(channel, "pageToken")
            .add_scope(Self::SCOPE)
            .doit()
            .await
            .map_err(anyhow::Error::from)?
            .1;

        Ok(channel)
    }

    async fn remove_channel(&mut self, channel: Channel) -> Result<()> {
        tracing::info!("Removing channel...");

        self.hub
            .channels()
            .stop(channel)
            .add_scope(Self::SCOPE)
            .doit()
            .await
            .map_err(anyhow::Error::from)?;
        Ok(())
    }

    pub async fn try_renew_channel(&mut self) {
        tracing::info!("Renewing channel...");

        if let Some(last_channel) = DB.last_channel() {
            if let Err(e) = self.remove_channel(last_channel).await {
                tracing::error!("Failed to remove last channel: {}", e.to_string());
            }
        }

        match self.register_channel().await {
            Ok(channel) => {
                tracing::info!("Channel renewed successfully");
                DB.set_last_channel(channel);
            }
            Err(e) => {
                tracing::error!("Failed to register channel: {}", e.to_string());
            }
        }
    }
}
