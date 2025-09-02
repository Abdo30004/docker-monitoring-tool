use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };

/// Container metrics including CPU, memory, network, and disk I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetrics {
    pub container_id: String,
    pub container_name: String,
    pub timestamp: DateTime<Utc>,
    pub cpu_stats: CpuStats,
    pub memory_stats: MemoryStats,
    pub network_stats: NetworkStats,
    pub disk_stats: DiskStats,
}

/// CPU usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// CPU usage percentage (0-100)
    pub usage_percent: f64,
    /// Total CPU time in nanoseconds
    pub total_usage: u64,
    /// System CPU time in nanoseconds
    pub system_cpu_usage: u64,
    /// Number of online CPUs
    pub online_cpus: u64,
}

impl CpuStats {
    pub fn new() -> Self {
        Self {
            usage_percent: 0.0,
            total_usage: 0,
            system_cpu_usage: 0,
            online_cpus: 0,
        }
    }
}

impl Default for CpuStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Current memory usage in bytes
    pub usage: u64,
    /// Maximum memory usage in bytes
    pub max_usage: u64,
    /// Memory limit in bytes
    pub limit: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
}

impl MemoryStats {
    pub fn new() -> Self {
        Self {
            usage: 0,
            max_usage: 0,
            limit: 0,
            usage_percent: 0.0,
        }
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Network I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total bytes received
    pub rx_bytes: u64,
    /// Total bytes transmitted
    pub tx_bytes: u64,
    /// Total packets received
    pub rx_packets: u64,
    /// Total packets transmitted
    pub tx_packets: u64,
    /// Receive errors
    pub rx_errors: u64,
    /// Transmit errors
    pub tx_errors: u64,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
        }
    }
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Disk I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    /// Total bytes read from disk
    pub read_bytes: u64,
    /// Total bytes written to disk
    pub write_bytes: u64,
    /// Number of read operations
    pub read_ops: u64,
    /// Number of write operations
    pub write_ops: u64,
}

impl DiskStats {
    pub fn new() -> Self {
        Self {
            read_bytes: 0,
            write_bytes: 0,
            read_ops: 0,
            write_ops: 0,
        }
    }
}

impl Default for DiskStats {
    fn default() -> Self {
        Self::new()
    }
}
