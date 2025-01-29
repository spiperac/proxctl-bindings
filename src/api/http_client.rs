use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient,
};
use serde::Serialize;

pub struct Client {
    client: ReqwestClient,
    base_url: String,
    api_token: Option<String>,
    csrf_token: Option<String>,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = ReqwestClient::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_token: None,
            csrf_token: None,
        }
    }

    pub fn set_tokens(&mut self, ticket: String, csrf_token: Option<String>) {
        let cookie = format!("PVEAuthCookie={}", ticket);
        self.client = ReqwestClient::builder()
            .danger_accept_invalid_certs(true)
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert("Accept", HeaderValue::from_static("application/json"));
                headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
                headers
            })
            .build()
            .expect("Failed to create HTTP client");

        self.api_token = Some(ticket);
        self.csrf_token = csrf_token;
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let req = self.client.get(&url);
        req.send().await
    }

    pub async fn post<T>(&self, path: &str, body: &T) -> Result<reqwest::Response, reqwest::Error>
    where
        T: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.post(&url).json(body);

        if let Some(csrf) = &self.csrf_token {
            req = req.header("CSRFPreventionToken", csrf);
        }

        req.send().await
    }
}
