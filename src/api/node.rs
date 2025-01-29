// api/node.rs
use crate::resources::node::NodeResponse;

use crate::ProxmoxApi;

impl ProxmoxApi {
    // Nodes
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
