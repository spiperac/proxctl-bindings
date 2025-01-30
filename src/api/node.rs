//! Node API Bindings
//!
//! This mod provides access to the Node API.
//! https://pve.proxmox.com/pve-docs/api-viewer/index.html#/nodes/

// api/node.rs
use crate::resources::node::{
    NetstatData, Node, NodeHostsResponse, NodeJournalResponse, NodeResponse,
};

use crate::ProxmoxApi;

impl ProxmoxApi {
    pub async fn get_nodes(&self) -> Result<NodeResponse<Node>, reqwest::Error> {
        let response = self.client.get("/api2/json/nodes").await?;
        let data: NodeResponse<Node> = response.json().await?;
        Ok(data)
    }

    pub async fn get_node_status(&self, node: &str) -> Result<String, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/status", node);
        let response = self.client.get(&path).await?;
        response.text().await
    }

    pub async fn get_node_config(&self, node: &str) -> Result<String, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/config", node);
        let response = self.client.get(&path).await?;
        response.text().await
    }

    pub async fn get_node_aplinfo(&self, node: &str) -> Result<String, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/aplinfo", node);
        let response = self.client.get(&path).await?;
        response.text().await
    }

    pub async fn get_node_dns(&self, node: &str) -> Result<String, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/dns", node);
        let response = self.client.get(&path).await?;
        response.text().await
    }

    pub async fn get_node_hosts(
        &self,
        node: &str,
    ) -> Result<NodeResponse<NodeHostsResponse>, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/hosts", node);
        let response = self.client.get(&path).await?;
        let data: NodeResponse<NodeHostsResponse> = response.json().await?;
        Ok(data)
    }

    pub async fn get_node_journal(
        &self,
        node: &str,
    ) -> Result<NodeJournalResponse, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/journal", node);
        let response = self.client.get(&path).await?;
        let data: NodeJournalResponse = response.json().await?;
        Ok(data)
    }
    pub async fn get_node_netstat(
        &self,
        node: &str,
    ) -> Result<NodeResponse<NetstatData>, reqwest::Error> {
        let path = format!("/api2/json/nodes/{}/netstat", node);
        let response = self.client.get(&path).await?;
        let data: NodeResponse<NetstatData> = response.json().await?;
        Ok(data)
    }
}
