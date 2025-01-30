// tests/node_test.rs
use mockito::{mock, server_url};
use proxctl_bindings::ProxmoxApi;

#[tokio::test]
async fn test_get_node_list() {
    // Set up the mock server to simulate Proxmox's node list response
    let _m = mock("GET", "/api2/json/nodes")
        .with_status(200)
        .with_body(r#"{"data": [{"node": "node1", "status": "online"}]}"#)
        .create();

    // Initialize ProxmoxApi with the mock server URL
    let proxmox_api = ProxmoxApi::new(server_url());

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
    let _m = mock("GET", "/api2/json/nodes/node1/status")
        .with_status(200)
        .with_body(r#"online"#)
        .create();

    // Initialize ProxmoxApi with the mock server URL
    let proxmox_api = ProxmoxApi::new(server_url());

    // Call get_node_status
    let response = proxmox_api.get_node_status("node1").await.unwrap();

    // Verify the response
    assert_eq!(response, "online");
}
