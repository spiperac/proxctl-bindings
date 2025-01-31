use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeQemuListResponse<T> {
    pub data: Vec<T>, // List of virtual machines
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeQemuResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QEMU {
    #[serde(rename = "vmid")]
    pub vm_id: u64, // Virtual Machine ID
    pub name: String,           // Name of the VM
    pub status: String,         // Current status (e.g., "running", "stopped")
    pub cpus: Option<u64>,      // Number of CPUs
    pub maxmem: Option<u64>,    // Maximum memory in bytes
    pub mem: Option<u64>,       // Used memory in bytes
    pub disk: Option<u64>,      // Disk space in bytes
    pub uptime: Option<u64>,    // Uptime in seconds
    pub netin: Option<u64>,     // Network input in bytes
    pub netout: Option<u64>,    // Network output in bytes
    pub diskread: Option<u64>,  // Disk read in bytes
    pub diskwrite: Option<u64>, // Disk write in bytes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VmStatus {
    pub vmid: u64,           // VM ID
    pub status: String,      // VM status (e.g., "running")
    pub cpus: Option<u64>,   // Number of CPUs
    pub mem: Option<u64>,    // Memory usage
    pub disk: Option<u64>,   // Disk usage
    pub uptime: Option<u64>, // Uptime in seconds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QemuConfig {
    pub acpi: Option<bool>,
    pub cores: Option<u32>,
    pub sockets: Option<u32>,
    pub memory: Option<u64>,
    pub name: Option<String>,
    pub net0: Option<String>,
    pub ide2: Option<String>,
    pub scsihw: Option<String>,
    pub ostype: Option<String>,
    pub bootdisk: Option<String>,
    pub description: Option<String>,
    pub onboot: Option<bool>,
    pub startup: Option<String>,
    pub hotplug: Option<String>,
    pub agent: Option<String>,
    pub balloon: Option<u64>,
    pub bios: Option<String>,
    pub boot: Option<String>,
    pub cpu: Option<String>,
    pub efidisk0: Option<String>,
    pub hookscript: Option<String>,
    pub machine: Option<String>,
    pub numa: Option<bool>,
    pub protection: Option<bool>,
    pub scsi0: Option<String>,
    pub smbios1: Option<String>,
    pub vga: Option<String>,
    pub vmgenid: Option<String>,
    pub vmstatestorage: Option<String>,
    pub tpmstate0: Option<String>,
    pub tags: Option<String>,
    pub template: Option<bool>,
    pub lock: Option<String>,
    pub digest: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QemuStatusCurrent {
    pub qmpstatus: Option<String>, // VM status (e.g., "running")
    pub diskread: Option<u64>,     // Disk read in bytes
    #[serde(rename = "running-qemu")]
    pub running_qemu: Option<String>, // Running QEMU version
    pub status: Option<String>,    // VM status
    pub freemem: Option<u64>,      // Free memory in bytes
    pub blockstat: Option<BlockStat>, // Block statistics
    pub pid: Option<u32>,          // Process ID
    pub maxdisk: Option<u64>,      // Maximum disk space in bytes
    pub tags: Option<String>,      // Tags associated with VM
    pub proxmox_support: Option<ProxmoxSupport>, // Proxmox support features
    pub ha: Option<Ha>,            // High Availability status
    pub agent: Option<u32>,        // Agent status
    #[serde(rename = "running-machine")]
    pub running_machine: Option<String>, // Running machine type
    pub ballooninfo: Option<BalloonInfo>, // Balloon info
    pub netin: Option<u64>,        // Network input in bytes
    pub mem: Option<u64>,          // Memory used in bytes
    pub balloon: Option<u64>,      // Balloon memory
    pub diskwrite: Option<u64>,    // Disk write in bytes
    pub name: Option<String>,      // VM name
    pub cpus: Option<u32>,         // Number of CPUs
    pub nics: Option<HashMap<String, Nic>>, // NIC statistics
    pub cpu: Option<f64>,          // CPU usage
    pub vmid: Option<u64>,         // Virtual Machine ID
    pub disk: Option<u64>,         // Disk space used in bytes
    pub netout: Option<u64>,       // Network output in bytes
    pub uptime: Option<u64>,       // Uptime in seconds
    pub maxmem: Option<u64>,       // Maximum memory in bytes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStat {
    pub ide2: BlockDeviceStat,
    pub scsi0: BlockDeviceStat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDeviceStat {
    pub wr_highest_offset: u64,
    pub account_invalid: bool,
    pub wr_bytes: u64,
    pub zone_append_bytes: u64,
    pub rd_total_time_ns: u64,
    pub invalid_zone_append_operations: u64,
    pub timed_stats: Vec<String>,
    pub zone_append_total_time_ns: u64,
    pub rd_operations: u64,
    pub wr_total_time_ns: u64,
    pub invalid_unmap_operations: u64,
    pub unmap_bytes: u64,
    pub wr_operations: u64,
    pub flush_total_time_ns: u64,
    pub unmap_total_time_ns: u64,
    pub zone_append_operations: u64,
    pub rd_bytes: u64,
    pub failed_flush_operations: u64,
    pub failed_wr_operations: u64,
    pub idle_time_ns: u64,
    pub flush_operations: u64,
    pub invalid_flush_operations: u64,
    pub rd_merged: u64,
    pub zone_append_merged: u64,
    pub failed_zone_append_operations: u64,
    pub unmap_operations: u64,
    pub wr_merged: u64,
    pub failed_unmap_operations: u64,
    pub invalid_wr_operations: u64,
    pub unmap_merged: u64,
    pub failed_rd_operations: u64,
    pub invalid_rd_operations: u64,
    pub account_failed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxmoxSupport {
    pub query_bitmap_info: Option<bool>,
    pub pbs_masterkey: Option<bool>,
    pub pbs_dirty_bitmap: Option<bool>,
    pub backup_max_workers: Option<bool>,
    pub pbs_dirty_bitmap_savevm: Option<bool>,
    pub pbs_library_version: Option<String>,
    pub backup_fleecing: Option<bool>,
    pub pbs_dirty_bitmap_migration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ha {
    pub managed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonInfo {
    pub major_page_faults: u64,
    pub total_mem: u64,
    pub mem_swapped_in: u64,
    pub last_update: u64,
    pub actual: u64,
    pub mem_swapped_out: u64,
    pub minor_page_faults: u64,
    pub free_mem: u64,
    pub max_mem: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nic {
    pub netout: Option<u64>, // Network output in bytes
    pub netin: Option<u64>,  // Network input in bytes
}
