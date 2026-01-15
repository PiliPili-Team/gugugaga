#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirCache {
    pub parent_dir_id: String,
    pub children: Vec<CacheEntry>,
}

// File(id) or Dir(id)
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CacheEntry {
    File(String),
    Dir(String),
}