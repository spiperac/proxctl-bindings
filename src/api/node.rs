//! Node API Bindings
//!
//! This mod provides access to the Node API.
//! https://pve.proxmox.com/pve-docs/api-viewer/index.html#/nodes/

// api/node.rs
use crate::resources::node::NodeResponse;

use crate::ProxmoxApi;

impl ProxmoxApi {
    pub async fn get_node_list(&self) -> Result<NodeResponse, reqwest::Error> {
        let response = self.client.get("/api2/json/nodes").await?;
        let data: NodeResponse = response.json().await?;
        Ok(data)
    }

    pub async fn get_node_status(&self, node: &str) -> Result<String, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/status", node);
        let response = self.client.get(&path).await?;
        response.text().await
    }
}
