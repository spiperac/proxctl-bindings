// resources/node.rs
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub data: Vec<Node>, // List of nodes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub cpu: f64,               // CPU usage percentage
    pub memory: u64,            // Total memory
    pub rootfs: Option<RootFs>, // Root filesystem usage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootFs {
    pub total: u64, // Total disk space
    pub used: u64,  // Used disk space
}
