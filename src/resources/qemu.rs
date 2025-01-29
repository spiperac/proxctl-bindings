use serde::{Deserialize, Serialize};

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
pub struct NodeQemuResponse {
    pub data: Vec<QEMU>, // List of virtual machines
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
