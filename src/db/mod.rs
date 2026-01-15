mod cache;

use std::sync::LazyLock;

use google_drive3::api::Channel;
use sled::*;

#[derive(Debug)]
pub struct Db {
    pub db: sled::Db,
}


pub static DB: LazyLock<Db> = LazyLock::new(|| Db::new());

impl Db {
    fn new() -> Self {
        let db_path = dirs::config_dir()
            .expect("Failed to get home directory")
            .join("gugugaga/db");

        let db = sled::open(db_path).expect("Failed to open database");
        Self { db }
    }

    pub fn set_files(&self, dir_id: &str, dir_cache: cache::DirCache) {
        let serialized = postcard::to_allocvec(&dir_cache)
            .expect("Failed to serialize directory cache");
        self.db
            .insert(dir_id, serialized)
            .expect("Failed to set directory cache");
    }

    pub fn files(&self, dir_id: &str) -> Option<cache::DirCache> {
        let bytes = self.db.get(dir_id).ok()??;
        match postcard::from_bytes(&bytes) {
            Ok(cache) => Some(cache),
            Err(e) => {
                tracing::error!(error = ?e, "Failed to deserialize directory cache");
                None
            }
        }
    }

    pub fn set_last_channel(&self, channel: Channel) {
        let serialized = postcard::to_allocvec(&channel)
            .expect("Failed to serialize channel");
        self.db
            .insert("last_channel", serialized)
            .expect("Failed to set last channel");
    }

    pub fn last_channel(&self) -> Option<Channel> {
        let bytes = self.db.get("last_channel").ok()??;
        match postcard::from_bytes(&bytes) {
            Ok(channel) => Some(channel),
            Err(e) => {
                tracing::error!(error = ?e, "Failed to deserialize last channel");
                None
            }
        }
    }
}