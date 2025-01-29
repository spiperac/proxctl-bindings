use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub storage: String, // Storage Name
    #[serde(rename = "type")]
    pub storage_type: String, // Storage type (e.g., "dir", "lvm")
    pub total: Option<u64>, // Total storage space
    pub used: Option<u64>, // Used storage space
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageListResponse {
    pub data: Vec<Storage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    pub volid: String, // Backup volume ID
    pub size: u64,     // Backup size
    pub ctime: u64,    // Creation time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupListResponse {
    pub data: Vec<Backup>,
}
