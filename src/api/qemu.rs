//! QEMU API Bindings
//!
//! This mod provides access to the QEMU API.
//! https://pve.proxmox.com/pve-docs/api-viewer/index.html#/nodes/{node}/qemu

// api/qemu.rs
use crate::resources::qemu::NodeQemuResponse;

use crate::ProxmoxApi;

impl ProxmoxApi {
    pub async fn get_node_qemu(&self, node: &str) -> Result<NodeQemuResponse, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/qemu", node);
        let response = self.client.get(&path).await?;

        let qemu_response: NodeQemuResponse = response.json().await?;
        Ok(qemu_response)
    }
}
