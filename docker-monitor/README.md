# Docker Monitoring Library

A Rust library for monitoring Docker containers, providing real-time metrics and container management capabilities.

## Features

- **Real-time Metrics Retrieval**: Fetch CPU, memory, network I/O, and disk I/O statistics for Docker containers
- **Container Management**: List, start, stop, and inspect containers
- **Event Monitoring**: Listen for Docker events (container start, stop, crash, etc.) with callback support
- **Robust Error Handling**: Comprehensive error types for Docker daemon issues and API errors
- **Async/Await Support**: Built on Tokio for efficient async operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
docker-monitor = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
```

## Quick Start

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new monitor instance
    let monitor = DockerMonitor::new().await?;

    // List all running containers
    let containers = monitor.list_containers().await?;
    for container in containers {
        println!("{}: {}", container.name, container.status);
    }

    Ok(())
}
```

## Usage Examples

### Getting Container Metrics

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    // Get metrics for a specific container
    let metrics = monitor.get_container_metrics("my-container").await?;

    println!("CPU Usage: {:.2}%", metrics.cpu_stats.usage_percent);
    println!("Memory Usage: {} MB", metrics.memory_stats.usage / 1024 / 1024);
    println!("Network RX: {} bytes", metrics.network_stats.rx_bytes);
    println!("Disk Read: {} bytes", metrics.disk_stats.read_bytes);

    Ok(())
}
```

### Container Management

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    // Start a container
    monitor.start_container("my-container").await?;

    // Get detailed information
    let details = monitor.get_container_details("my-container").await?;
    println!("Container: {} ({})", details.name, details.status);

    // Stop a container
    monitor.stop_container("my-container").await?;

    Ok(())
}
```

### Event Monitoring

```rust
use docker_monitor::{DockerMonitor, DockerEvent};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    // Define a callback for events
    let callback = Arc::new(|event: DockerEvent| {
        println!("[{}] {} - {}",
            event.timestamp,
            event.event_type,
            event.container_name
        );
    });

    // Start monitoring (this blocks)
    monitor.monitor_events(callback).await?;

    Ok(())
}
```

## API Documentation

### DockerMonitor

The main struct for interacting with Docker.

#### Methods

- `new() -> Result<Self>` - Create a new monitor instance
- `list_containers() -> Result<Vec<ContainerInfo>>` - List all running containers
- `get_container_details(container_id: &str) -> Result<ContainerInfo>` - Get detailed container information
- `start_container(container_id: &str) -> Result<()>` - Start a container
- `stop_container(container_id: &str) -> Result<()>` - Stop a container
- `get_container_metrics(container_id: &str) -> Result<ContainerMetrics>` - Get real-time metrics
- `monitor_events(callback: EventCallback) -> Result<()>` - Monitor Docker events

### ContainerMetrics

Contains comprehensive metrics for a container:

- `cpu_stats: CpuStats` - CPU usage statistics
- `memory_stats: MemoryStats` - Memory usage statistics
- `network_stats: NetworkStats` - Network I/O statistics
- `disk_stats: DiskStats` - Disk I/O statistics

### Error Handling

All operations return `Result<T, DockerMonitorError>`. Error types include:

- `DaemonUnavailable` - Docker daemon is not accessible
- `ApiError` - Docker API returned an error
- `ContainerNotFound` - Specified container doesn't exist
- `MetricsUnavailable` - Unable to retrieve metrics
- `EventError` - Error in event monitoring

## Examples

The library includes several example programs in the `examples/` directory:

- `basic_usage.rs` - List containers and get metrics
- `event_monitoring.rs` - Monitor Docker events in real-time
- `container_management.rs` - Start/stop containers
- `metrics_stream.rs` - Stream metrics continuously

Run an example with:

```bash
cargo run --example basic_usage
```

## Requirements

- Rust 1.70 or later
- Docker daemon running and accessible
- Tokio runtime for async operations

## Testing

Run the test suite:

```bash
cargo test
```

Note: Some tests require a running Docker daemon.

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
