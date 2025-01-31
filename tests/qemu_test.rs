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

#[tokio::test]
async fn test_get_qemu_status_current() {
    let _m = mock("GET", "/api2/json/nodes/node1/qemu/103/status/current")
        .with_status(200)
        .with_body(
            r#"{
            "data": {
                "qmpstatus": "running",
                "diskread": 1000,
                "running-qemu": "5.2.0",
                "status": "running",
                "freemem": 1048576,
                "pid": 1234,
                "maxdisk": 102400,
                "tags": "vm_tag",
                "proxmox_support": {},
                "agent": 1,
                "running-machine": "pc-i440fx-5.2",
                "netin": 512,
                "mem": 2048,
                "balloon": 512,
                "diskwrite": 2048,
                "name": "Test VM",
                "cpus": 2,
                "cpu": 25.5,
                "vmid": 103,
                "disk": 1024,
                "netout": 512,
                "uptime": 3600,
                "maxmem": 8192,
                "nics":{"tap103i0":{"netout":92795878,"netin":496777938}}
            }
        }"#,
        )
        .create();

    let proxmox_api = ProxmoxApi::new(server_url().to_string());
    let result = proxmox_api.get_qemu_status("node1", 103).await;

    assert!(result.is_ok());
    let status_response = result.unwrap();
    let status = status_response.data;

    // Basic status checks
    assert_eq!(status.qmpstatus, Some("running".to_string()));
    assert_eq!(status.diskread, Some(1000));
    assert_eq!(status.running_qemu, Some("5.2.0".to_string()));
    assert_eq!(status.status, Some("running".to_string()));
    assert_eq!(status.freemem, Some(1048576));
    assert_eq!(status.pid, Some(1234));
    assert_eq!(status.name, Some("Test VM".to_string()));
    assert_eq!(status.vmid, Some(103));

    // NIC specific checks
    assert!(status.nics.is_some(), "Nics should not be None");

    if let Some(nics) = &status.nics {
        assert!(
            nics.contains_key("tap103i0"),
            "Should contain tap103i0 interface"
        );

        if let Some(nic) = nics.get("tap103i0") {
            assert_eq!(nic.netout, Some(92795878));
            assert_eq!(nic.netin, Some(496777938));
        }
    }
}

#[tokio::test]
async fn test_get_qemu_config() {
    let _m = mock("GET", "/api2/json/nodes/node1/qemu/100/config")
        .with_status(200)
        .with_body(
            r#"{
            "data": {
                "acpi": true,
                "cores": 2,
                "sockets": 1,
                "memory": 4096,
                "name": "test-vm",
                "net0": "virtio=DE:AD:BE:EF:00:01,bridge=vmbr0",
                "ide2": "local:iso/debian.iso,media=cdrom",
                "scsihw": "virtio-scsi-pci",
                "ostype": "l26",
                "bootdisk": "scsi0",
                "description": "Test VM",
                "onboot": true,
                "startup": "order=1",
                "hotplug": "network,disk,usb",
                "agent": "enabled=1",
                "balloon": 2048,
                "bios": "seabios",
                "boot": "order=scsi0",
                "cpu": "host",
                "efidisk0": "local-lvm:vm-100-disk-1,size=1M",
                "hookscript": "local:snippets/hookscript.sh",
                "machine": "q35",
                "numa": false,
                "protection": true,
                "scsi0": "local-lvm:vm-100-disk-0,size=20G",
                "smbios1": "uuid=1234-5678-90ab-cdef",
                "vga": "qxl",
                "vmgenid": "12345678-90ab-cdef-1234-567890abcdef",
                "vmstatestorage": "local",
                "tpmstate0": "local-lvm:vm-100-disk-2,size=4M",
                "tags": "test,vm",
                "template": false,
                "lock": "backup",
                "digest": "abc123"
            }
        }"#,
        )
        .create();

    let proxmox_api = ProxmoxApi::new(server_url().to_string());

    let result = proxmox_api.get_qemu_config("node1", 100).await;

    if let Err(e) = &result {
        println!("{:?}", e);
    }
    assert!(result.is_ok());
    let config_response = result.unwrap();
    let config = config_response.data;
    assert_eq!(config.acpi, Some(true));
    assert_eq!(config.cores, Some(2));
    assert_eq!(config.sockets, Some(1));
    assert_eq!(config.memory, Some(4096));
    assert_eq!(config.name.as_deref(), Some("test-vm"));
    assert_eq!(
        config.net0.as_deref(),
        Some("virtio=DE:AD:BE:EF:00:01,bridge=vmbr0")
    );
    assert_eq!(
        config.ide2.as_deref(),
        Some("local:iso/debian.iso,media=cdrom")
    );
    assert_eq!(config.scsihw.as_deref(), Some("virtio-scsi-pci"));
    assert_eq!(config.ostype.as_deref(), Some("l26"));
    assert_eq!(config.bootdisk.as_deref(), Some("scsi0"));
    assert_eq!(config.description.as_deref(), Some("Test VM"));
    assert_eq!(config.onboot, Some(true));
    assert_eq!(config.startup.as_deref(), Some("order=1"));
    assert_eq!(config.hotplug.as_deref(), Some("network,disk,usb"));
    assert_eq!(config.agent.as_deref(), Some("enabled=1"));
    assert_eq!(config.balloon, Some(2048));
    assert_eq!(config.bios.as_deref(), Some("seabios"));
    assert_eq!(config.boot.as_deref(), Some("order=scsi0"));
    assert_eq!(config.cpu.as_deref(), Some("host"));
    assert_eq!(
        config.efidisk0.as_deref(),
        Some("local-lvm:vm-100-disk-1,size=1M")
    );
    assert_eq!(
        config.hookscript.as_deref(),
        Some("local:snippets/hookscript.sh")
    );
    assert_eq!(config.machine.as_deref(), Some("q35"));
    assert_eq!(config.numa, Some(false));
    assert_eq!(config.protection, Some(true));
    assert_eq!(
        config.scsi0.as_deref(),
        Some("local-lvm:vm-100-disk-0,size=20G")
    );
    assert_eq!(config.smbios1.as_deref(), Some("uuid=1234-5678-90ab-cdef"));
    assert_eq!(config.vga.as_deref(), Some("qxl"));
    assert_eq!(
        config.vmgenid.as_deref(),
        Some("12345678-90ab-cdef-1234-567890abcdef")
    );
    assert_eq!(config.vmstatestorage.as_deref(), Some("local"));
    assert_eq!(
        config.tpmstate0.as_deref(),
        Some("local-lvm:vm-100-disk-2,size=4M")
    );
    assert_eq!(config.tags.as_deref(), Some("test,vm"));
    assert_eq!(config.template, Some(false));
    assert_eq!(config.lock.as_deref(), Some("backup"));
    assert_eq!(config.digest.as_deref(), Some("abc123"));
}
