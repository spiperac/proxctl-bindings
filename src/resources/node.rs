// resources/node.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListResponse<T> {
    pub data: Vec<T>, // List of nodes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub node: String,
    pub status: String,
    pub cpu: Option<f32>,
    pub level: Option<String>,
    pub maxcpu: Option<u64>,
    pub mem: Option<u64>,
    pub ssl_fingerprint: Option<String>,
    pub uptime: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct NetstatData {
    pub dev: String,
    #[serde(rename = "in")]
    pub in_: String, // Using `in_` to avoid conflict with the Rust keyword `in`
    pub out: String,
    pub vmid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeJournalResponse {
    pub data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalQueryParams {
    pub node: String,
    pub endcursor: Option<String>,
    pub lastentries: Option<i32>,
    pub since: Option<i32>,
    pub startcursor: Option<String>,
    pub until: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeHostsResponse {
    pub data: String,
    pub digest: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub cpu: f64,               // CPU usage percentage
    pub memory: u64,            // Total memory
    pub rootfs: Option<RootFs>, // Root filesystem usage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDnsResponse {
    pub dns1: Option<String>,
    pub dns2: Option<String>,
    pub dns3: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeDomain {
    pub domain: String,
    pub alias: Option<String>,
    pub plugin: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeConfigResponse {
    pub acme: Option<String>,
    pub description: Option<String>,
    pub digest: Option<String>,
    pub startall_onboot_delay: Option<u32>,
    pub wakeonlan: Option<String>,
    pub acmedomain0: Option<AcmeDomain>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootFs {
    pub total: u64, // Total disk space
    pub used: u64,  // Used disk space
}
