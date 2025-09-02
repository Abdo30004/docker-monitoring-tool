# Docker Monitor - Complete Usage Guide

## Table of Contents

1. [Installation](#installation)
2. [Basic Setup](#basic-setup)
3. [Container Management](#container-management)
4. [Metrics Monitoring](#metrics-monitoring)
5. [Event Handling](#event-handling)
6. [Error Handling](#error-handling)
7. [Advanced Usage](#advanced-usage)
8. [Best Practices](#best-practices)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
docker-monitor = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
```

## Basic Setup

### Creating a Monitor Instance

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new monitor - connects to Docker daemon
    let monitor = DockerMonitor::new().await?;

    // Now you can use the monitor for various operations
    Ok(())
}
```

### Connection Error Handling

```rust
use docker_monitor::{DockerMonitor, DockerMonitorError};

#[tokio::main]
async fn main() {
    match DockerMonitor::new().await {
        Ok(monitor) => {
            println!("Connected to Docker daemon");
            // Use monitor...
        }
        Err(DockerMonitorError::DaemonUnavailable(msg)) => {
            eprintln!("Docker daemon not available: {}", msg);
            eprintln!("Make sure Docker is installed and running");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## Container Management

### List All Running Containers

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let containers = monitor.list_containers().await?;

    println!("Running containers:");
    for container in containers {
        println!("  {} ({})", container.name, container.id);
        println!("    Image: {}", container.image);
        println!("    Status: {}", container.status);
        println!("    State: {}", container.state);
    }

    Ok(())
}
```

### Get Container Details

```rust
let container_id = "my-container";
match monitor.get_container_details(container_id).await {
    Ok(details) => {
        println!("Container: {}", details.name);
        println!("Image: {}", details.image);
        println!("Status: {}", details.status);
        println!("Labels: {:?}", details.labels);
    }
    Err(DockerMonitorError::ContainerNotFound(id)) => {
        eprintln!("Container not found: {}", id);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### Start a Container

```rust
match monitor.start_container("my-container").await {
    Ok(_) => println!("Container started successfully"),
    Err(e) => eprintln!("Failed to start container: {}", e),
}
```

### Stop a Container

```rust
match monitor.stop_container("my-container").await {
    Ok(_) => println!("Container stopped successfully"),
    Err(e) => eprintln!("Failed to stop container: {}", e),
}
```

## Metrics Monitoring

### Get Real-time Metrics

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let metrics = monitor.get_container_metrics("my-container").await?;

    // CPU Metrics
    println!("CPU Usage: {:.2}%", metrics.cpu_stats.usage_percent);
    println!("Total CPU Time: {} ns", metrics.cpu_stats.total_usage);
    println!("Online CPUs: {}", metrics.cpu_stats.online_cpus);

    // Memory Metrics
    println!("\nMemory Usage: {} bytes", metrics.memory_stats.usage);
    println!("Memory Usage: {} MB", metrics.memory_stats.usage / 1024 / 1024);
    println!("Memory Limit: {} MB", metrics.memory_stats.limit / 1024 / 1024);
    println!("Memory %: {:.2}%", metrics.memory_stats.usage_percent);

    // Network Metrics
    println!("\nNetwork RX: {} bytes", metrics.network_stats.rx_bytes);
    println!("Network TX: {} bytes", metrics.network_stats.tx_bytes);
    println!("RX Packets: {}", metrics.network_stats.rx_packets);
    println!("TX Packets: {}", metrics.network_stats.tx_packets);

    // Disk Metrics
    println!("\nDisk Read: {} bytes", metrics.disk_stats.read_bytes);
    println!("Disk Write: {} bytes", metrics.disk_stats.write_bytes);
    println!("Read Ops: {}", metrics.disk_stats.read_ops);
    println!("Write Ops: {}", metrics.disk_stats.write_ops);

    Ok(())
}
```

### Continuous Metrics Monitoring

```rust
use docker_monitor::DockerMonitor;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let container_id = "my-container";

    // Monitor metrics every 2 seconds
    loop {
        match monitor.get_container_metrics(container_id).await {
            Ok(metrics) => {
                println!("[{}] CPU: {:.2}% | Memory: {} MB",
                    metrics.timestamp.format("%H:%M:%S"),
                    metrics.cpu_stats.usage_percent,
                    metrics.memory_stats.usage / 1024 / 1024
                );
            }
            Err(e) => {
                eprintln!("Error getting metrics: {}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}
```

### Monitor All Containers

```rust
use docker_monitor::DockerMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let containers = monitor.list_containers().await?;

    for container in containers {
        if let Ok(metrics) = monitor.get_container_metrics(&container.id).await {
            println!("{}: CPU={:.2}% MEM={} MB",
                container.name,
                metrics.cpu_stats.usage_percent,
                metrics.memory_stats.usage / 1024 / 1024
            );
        }
    }

    Ok(())
}
```

## Event Handling

### Basic Event Monitoring

```rust
use docker_monitor::{DockerMonitor, DockerEvent};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    let callback = Arc::new(|event: DockerEvent| {
        println!("[{}] {} - Container: {}",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.event_type,
            event.container_name
        );
    });

    println!("Monitoring Docker events... (Press Ctrl+C to stop)");
    monitor.monitor_events(callback).await?;

    Ok(())
}
```

### Event Filtering

```rust
use docker_monitor::{DockerMonitor, DockerEvent, EventType};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    let callback = Arc::new(|event: DockerEvent| {
        // Only log start and stop events
        match event.event_type {
            EventType::ContainerStart | EventType::ContainerStop => {
                println!("{}: {}", event.event_type, event.container_name);
            }
            _ => {} // Ignore other events
        }
    });

    monitor.monitor_events(callback).await?;
    Ok(())
}
```

### Event with Actions

```rust
use docker_monitor::{DockerMonitor, DockerEvent, EventType};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;

    let callback = Arc::new(move |event: DockerEvent| {
        match event.event_type {
            EventType::ContainerDie | EventType::ContainerOom => {
                eprintln!("ALERT: Container {} crashed!", event.container_name);
                // You could send notifications, restart container, etc.
            }
            EventType::ContainerStart => {
                println!("Container {} started", event.container_name);
            }
            _ => {}
        }
    });

    monitor.monitor_events(callback).await?;
    Ok(())
}
```

## Error Handling

### Comprehensive Error Handling

```rust
use docker_monitor::{DockerMonitor, DockerMonitorError};

#[tokio::main]
async fn main() {
    let monitor = match DockerMonitor::new().await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create monitor: {}", e);
            return;
        }
    };

    // Handle specific errors
    match monitor.get_container_metrics("test").await {
        Ok(metrics) => {
            println!("CPU: {:.2}%", metrics.cpu_stats.usage_percent);
        }
        Err(DockerMonitorError::ContainerNotFound(id)) => {
            eprintln!("Container '{}' not found", id);
        }
        Err(DockerMonitorError::MetricsUnavailable(msg)) => {
            eprintln!("Metrics unavailable: {}", msg);
        }
        Err(DockerMonitorError::DaemonUnavailable(msg)) => {
            eprintln!("Docker daemon issue: {}", msg);
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }
}
```

## Advanced Usage

### Concurrent Monitoring

```rust
use docker_monitor::DockerMonitor;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = DockerMonitor::new().await?;
    let containers = monitor.list_containers().await?;

    // Monitor multiple containers concurrently
    let mut handles = vec![];

    for container in containers {
        let container_id = container.id.clone();
        let monitor_clone = monitor.clone(); // Note: Would need Clone impl

        let handle = task::spawn(async move {
            // Monitor this container...
        });

        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
```

### Custom Metrics Dashboard

```rust
use docker_monitor::{DockerMonitor, ContainerMetrics};
use std::collections::HashMap;

struct Dashboard {
    monitor: DockerMonitor,
    metrics_history: HashMap<String, Vec<ContainerMetrics>>,
}

impl Dashboard {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            monitor: DockerMonitor::new().await?,
            metrics_history: HashMap::new(),
        })
    }

    async fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let containers = self.monitor.list_containers().await?;

        for container in containers {
            if let Ok(metrics) = self.monitor.get_container_metrics(&container.id).await {
                self.metrics_history
                    .entry(container.id.clone())
                    .or_insert_with(Vec::new)
                    .push(metrics);
            }
        }

        Ok(())
    }

    fn display_summary(&self) {
        for (id, history) in &self.metrics_history {
            if let Some(latest) = history.last() {
                println!("{}: CPU={:.2}% MEM={} MB",
                    &id[..12],
                    latest.cpu_stats.usage_percent,
                    latest.memory_stats.usage / 1024 / 1024
                );
            }
        }
    }
}
```

## Best Practices

### 1. Connection Management

```rust
// ✅ Create monitor once and reuse
let monitor = DockerMonitor::new().await?;
// Use monitor for multiple operations

// ❌ Don't create new monitor for each operation
// let monitor = DockerMonitor::new().await?; // Avoid
```

### 2. Error Handling

```rust
// ✅ Handle specific errors
match monitor.get_container_metrics(id).await {
    Ok(metrics) => { /* ... */ }
    Err(DockerMonitorError::ContainerNotFound(_)) => { /* Handle */ }
    Err(e) => { /* Other errors */ }
}

// ❌ Don't ignore errors
// monitor.start_container(id).await; // Avoid
```

### 3. Resource Cleanup

```rust
// Metrics monitoring in a controlled loop
for _ in 0..10 {
    let metrics = monitor.get_container_metrics(id).await?;
    println!("{:?}", metrics);
    tokio::time::sleep(Duration::from_secs(1)).await;
}
// Loop ends, resources cleaned up
```

### 4. Event Monitoring

```rust
// ✅ Use appropriate callback scope
let callback = Arc::new(|event| {
    // Lightweight operations
    println!("{}", event.event_type);
});

// ❌ Avoid blocking operations in callbacks
// let callback = Arc::new(|event| {
//     // Don't do heavy computation or blocking I/O
// });
```

### 5. Concurrent Operations

```rust
// ✅ Use tokio::join! for concurrent operations
let (containers, metrics) = tokio::join!(
    monitor.list_containers(),
    monitor.get_container_metrics("id")
);

// Process results...
```

## Troubleshooting

### Docker Daemon Not Available

```rust
// Check if Docker is running
if let Err(DockerMonitorError::DaemonUnavailable(_)) = DockerMonitor::new().await {
    eprintln!("Docker daemon not available. Please ensure Docker is running.");
}
```

### Container Not Found

```rust
// Verify container exists first
let containers = monitor.list_containers().await?;
let exists = containers.iter().any(|c| c.name == "my-container");
if !exists {
    eprintln!("Container not found in running containers");
}
```

### Metrics Unavailable

```rust
// Some containers may not have metrics immediately after starting
tokio::time::sleep(Duration::from_secs(1)).await;
let metrics = monitor.get_container_metrics(id).await?;
```

## See Also

- [API Documentation](https://docs.rs/docker-monitor)
- [Examples Directory](../examples/)
- [Architecture Guide](ARCHITECTURE.md)
- [Getting Started](GETTING_STARTED.md)
