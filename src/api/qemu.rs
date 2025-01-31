//! QEMU API Bindings
//!
//! This mod provides access to the QEMU API.
//! https://pve.proxmox.com/pve-docs/api-viewer/index.html#/nodes/{node}/qemu

// api/qemu.rs
use crate::resources::qemu::{
    NodeQemuListResponse, NodeQemuResponse, QemuConfig, QemuStatusCurrent, QEMU,
};

use crate::ProxmoxApi;

impl ProxmoxApi {
    pub async fn get_node_qemu(
        &self,
        node: &str,
    ) -> Result<NodeQemuListResponse<QEMU>, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/qemu", node);
        let response = self.client.get(&path).await?;

        let qemu_response: NodeQemuListResponse<QEMU> = response.json().await?;
        Ok(qemu_response)
    }

    pub async fn get_qemu_status(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<NodeQemuResponse<QemuStatusCurrent>, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/qemu/{}/status/current", node, vmid);
        let response = self.client.get(&path).await?;

        let data: NodeQemuResponse<QemuStatusCurrent> = response.json().await?;
        Ok(data)
    }

    pub async fn get_qemu_config(
        &self,
        node: &str,
        vmid: u32,
    ) -> Result<NodeQemuResponse<QemuConfig>, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/qemu/{}/config", node, vmid);
        let response = self.client.get(&path).await?;

        let data: NodeQemuResponse<QemuConfig> = response.json().await?;
        Ok(data)
    }
}
