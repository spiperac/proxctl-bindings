// tests/node_test.rs
use proxctl_bindings::ProxmoxApi;

#[tokio::test]
async fn test_get_node_list() {
    // Set up the server.mock server to simulate Proxmox's node list response
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes")
        .with_status(200)
        .with_body(r#"{"data": [{"node": "node1", "status": "online"}]}"#)
        .create_async()
        .await;

    // Initialize ProxmoxApi with the mock server URL
    let proxmox_api = ProxmoxApi::new(server.url());

    // Call get_node_list
    let response = proxmox_api.get_nodes().await.unwrap();

    // Verify the response
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].node, "node1");
    assert_eq!(response.data[0].status, "online");
}

#[tokio::test]
async fn test_get_node_status() {
    // Set up the mock server to simulate Proxmox's node status response
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/status")
        .with_status(200)
        .with_body(r#"online"#)
        .create_async()
        .await;

    // Initialize ProxmoxApi with the mock server URL
    let proxmox_api = ProxmoxApi::new(server.url());

    // Cat_node_status
    let response = proxmox_api.get_node_status("node1").await.unwrap();

    // Verify the response
    assert_eq!(response, "online");
}

#[tokio::test]
async fn test_get_node_config() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/config")
        .with_status(200)
        .with_body(r#"{"data": {"description": "Test Node", "digest": "abc123"}}"#)
        .create_async()
        .await;

    let proxmox_api = ProxmoxApi::new(server.url());
    let response = proxmox_api.get_node_config("node1").await.unwrap();
    assert!(response.contains("Test Node"));
}

#[tokio::test]
async fn test_get_node_dns() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/dns")
        .with_status(200)
        .with_body(r#"{"data": {"dns1": "8.8.8.8", "search": "example.com"}}"#)
        .create_async()
        .await;

    let proxmox_api = ProxmoxApi::new(server.url());
    let response = proxmox_api.get_node_dns("node1").await.unwrap();
    assert_eq!(response.data.dns1, Some("8.8.8.8".to_string()));
    assert_eq!(response.data.search, Some("example.com".to_string()));
}

#[tokio::test]
async fn test_get_node_hosts() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/hosts")
        .with_status(200)
        .with_body(r#"{"data": {"data": "127.0.0.1 localhost", "digest": "abc123"}}"#)
        .create_async()
        .await;

    let proxmox_api = ProxmoxApi::new(server.url());
    let response = proxmox_api.get_node_hosts("node1").await.unwrap();
    assert_eq!(response.data.data, "127.0.0.1 localhost");
    assert_eq!(response.data.digest, Some("abc123".to_string()));
}

#[tokio::test]
async fn test_get_node_journal() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/journal")
        .with_status(200)
        .with_body(r#"{"data": ["log1", "log2"]}"#)
        .create_async()
        .await;

    let proxmox_api = ProxmoxApi::new(server.url());
    let response = proxmox_api.get_node_journal("node1").await.unwrap();
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0], "log1");
}

#[tokio::test]
async fn test_get_node_netstat() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/api2/json/nodes/node1/netstat")
        .with_status(200)
        .with_body(r#"{"data": [{"dev": "net0", "in": "1234", "out": "5678", "vmid": "100"}]}"#)
        .create_async()
        .await;

    let proxmox_api = ProxmoxApi::new(server.url());
    let response = proxmox_api.get_node_netstat("node1").await.unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].dev, "net0");
    assert_eq!(response.data[0].in_, "1234");
    assert_eq!(response.data[0].out, "5678");
}
