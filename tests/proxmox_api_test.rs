// tests/proxmox_api_test.rs
use proxctl_bindings::ProxmoxApi;

#[tokio::test]
async fn test_authenticate() {
    // Set up the mock server to simulate Proxmox's API response
    // Request a new server from the pool
    let mut server = mockito::Server::new_async().await;

    let _m = server.mock("POST", "/api2/json/access/ticket")
        .with_status(200)
        .with_body(r#"{"data": {"ticket": "fake_ticket", "CSRFPreventionToken": "fake_csrf_token", "username": "test_user"}}"#)
        .create_async().await;

    // Initialize ProxmoxApi with the mock server URL
    let mut proxmox_api = ProxmoxApi::new(server.url());

    // Call authenticate
    let response = proxmox_api
        .authenticate("test_user", "test_password", "pam")
        .await
        .unwrap();

    // Verify the response
    assert!(proxmox_api.client.api_token.is_some());
    assert!(proxmox_api.client.csrf_token.is_some());
    assert_eq!(response.data.ticket, "fake_ticket");
    assert_eq!(response.data.csrf_prevention_token, "fake_csrf_token");
    assert_eq!(response.data.username, "test_user");
}
