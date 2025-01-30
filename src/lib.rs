//! Proxmox API Bindings
//!
//! This crate provides bindings for Proxmox HTTP API.
//!

pub mod api;
pub mod resources;

use crate::api::http_client::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProxmoxResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub ticket: String,
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf_prevention_token: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub release: String,
}

pub struct ProxmoxApi {
    pub client: Client,
}

impl ProxmoxApi {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }

    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str,
        realm: &str,
    ) -> Result<ProxmoxResponse<AuthResponse>, reqwest::Error> {
        let auth_data = serde_json::json!({
            "username": username,
            "password": password,
            "realm": realm,
        });

        let response = self
            .client
            .post("/api2/json/access/ticket", &auth_data)
            .await?;

        let auth_response: ProxmoxResponse<AuthResponse> = response.json().await?;

        // Store the authentication token in Cookie
        // Can also be set in "Authorize" header.
        self.client.set_tokens(
            auth_response.data.ticket.clone(),
            Some(auth_response.data.csrf_prevention_token.clone()),
        );

        Ok(auth_response)
    }

    pub async fn get_version(&self) -> Result<ProxmoxResponse<VersionInfo>, reqwest::Error> {
        let response = self.client.get("/api2/json/version").await?;
        let data: ProxmoxResponse<VersionInfo> = response.json().await?;
        Ok(data)
    }
}
