# Docker Monitor - Getting Started

This guide will help you get started with the Docker Monitor library.

## Prerequisites

- Rust 1.70 or later
- Docker installed and running
- Basic knowledge of Rust and async programming

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
docker-monitor = { path = "../docker-monitor" }  # Or use the published version
tokio = { version = "1.40", features = ["full"] }
```

## Quick Start

### 1. Create a Simple Monitor

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    println!("Connected to Docker daemon!");
    Ok(())
}
```

### 2. List Running Containers

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let containers = monitor.list_containers().await?;

    for container in containers {
        println!("{} ({})", container.name, container.id);
    }

    Ok(())
}
```

### 3. Get Container Metrics

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let metrics = monitor.get_container_metrics("my-container").await?;

    println!("CPU: {:.2}%", metrics.cpu_stats.usage_percent);
    println!("Memory: {} MB", metrics.memory_stats.usage / 1024 / 1024);

    Ok(())
}
```

### 4. Monitor Events

```rust
use docker_monitor::{DockerMonitor, DockerEvent};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    let callback = Arc::new(|event: DockerEvent| {
        println!("{}: {}", event.event_type, event.container_name);
    });

    monitor.monitor_events(callback).await?;
    Ok(())
}
```

## Next Steps

- Check out the [examples](../examples/) directory for more detailed examples
- Read the API documentation with `cargo doc --open`
- See the [README](../README.md) for comprehensive feature overview
