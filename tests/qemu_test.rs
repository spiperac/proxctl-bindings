// tests/qemu_test.rs
use mockito::{mock, server_url};
use proxctl_bindings::ProxmoxApi;

#[tokio::test]
async fn test_get_node_qemu() {
    let _m = mock("GET", "/api2/json/nodes/node1/qemu")
        .with_status(200)
        .with_body(
            r#"{
            "data": [
                {
                    "vmid": 100,
                    "name": "test-vm",
                    "status": "running",
                    "cpus": 2,
                    "maxmem": 4096,
                    "mem": 2048,
                    "disk": 10240,
                    "uptime": 3600,
                    "netin": 5000,
                    "netout": 3000,
                    "diskread": 2000,
                    "diskwrite": 1000
                }
            ]
        }"#,
        )
        .create();

    let proxmox_api = ProxmoxApi::new(server_url().to_string());

    let result = proxmox_api.get_node_qemu("node1").await;

    assert!(result.is_ok());
    let qemu_response = result.unwrap();
    assert_eq!(qemu_response.data.len(), 1);
    let qemu = &qemu_response.data[0];
    assert_eq!(qemu.vm_id, 100);
    assert_eq!(qemu.name, "test-vm");
    assert_eq!(qemu.status, "running");
}
