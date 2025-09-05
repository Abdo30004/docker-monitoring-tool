use docker_monitor::DockerMonitor;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Docker Monitor - Basic Usage Example\n");

    let monitor = DockerMonitor::new().await?;
    println!("✓ Connected to Docker daemon\n");

    println!("=== Running Containers ===");
    let containers = monitor.list_containers().await?;

    if containers.is_empty() {
        println!("No running containers found.");
        return Ok(());
    }

    for container in &containers {
        println!("\nContainer: {}", container.name);
        println!("  ID: {}", container.id);
        println!("  Image: {}", container.image);
        println!("  Status: {}", container.status);
    }

    if let Some(container) = containers.first() {
        println!("\n=== Metrics for {} ===", container.name);
        let metrics = monitor.get_container_metrics(&container.id).await?;

        println!("CPU:");
        println!("  Usage: {:.2}%", metrics.cpu_stats.usage_percent);

        println!("Memory:");
        println!("  Usage: {} MB", metrics.memory_stats.usage / 1024 / 1024);
        println!("  Limit: {} MB", metrics.memory_stats.limit / 1024 / 1024);
        println!("  Percentage: {:.2}%", metrics.memory_stats.usage_percent);

        println!("Network:");
        println!("  RX: {} bytes", metrics.network_stats.rx_bytes);
        println!("  TX: {} bytes", metrics.network_stats.tx_bytes);

        println!("Disk:");
        println!("  Read: {} bytes", metrics.disk_stats.read_bytes);
        println!("  Write: {} bytes", metrics.disk_stats.write_bytes);
    }

    Ok(())
}
