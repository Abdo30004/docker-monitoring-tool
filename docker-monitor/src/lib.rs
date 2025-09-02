//! # Docker Monitoring Library
//!
//! A Rust library for monitoring Docker containers, providing real-time metrics
//! and container management capabilities.
//!
//! ## Features
//!
//! - Real-time metrics retrieval (CPU, memory, network I/O, disk I/O)
//! - Container management (list, start, stop, details)
//! - Event monitoring with callbacks
//! - Robust error handling
//!
//! ## Example
//!
//! ```no_run
//! use docker_monitor::DockerMonitor;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let monitor = DockerMonitor::new().await?;
//!
//!     // List all running containers
//!     let containers = monitor.list_containers().await?;
//!     println!("Running containers: {}", containers.len());
//!
//!     Ok(())
//! }
//! ```

mod monitor;
mod metrics;
mod container;
mod events;
mod error;

pub use monitor::DockerMonitor;
pub use metrics::{ ContainerMetrics, CpuStats, MemoryStats, NetworkStats, DiskStats };
pub use container::{ ContainerInfo, ContainerStatus };
pub use events::{ DockerEvent, EventType, EventCallback };
pub use error::{ DockerMonitorError, Result };
