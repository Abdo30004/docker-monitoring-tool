use docker_monitor::DockerMonitor;
use std::error::Error;
use tokio::time::{ sleep, Duration };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Docker Monitor - Real-time Metrics Stream Example\n");

    let monitor = DockerMonitor::new().await?;

    let containers = monitor.list_containers().await?;

    if containers.is_empty() {
        println!("No running containers found.");
        return Ok(());
    }

    let container = &containers[0];
    println!("Monitoring metrics for: {}\n", container.name);
    println!("Press Ctrl+C to stop\n");

    loop {
        match monitor.get_container_metrics(&container.id).await {
            Ok(metrics) => {
                println!("=== {} ===", metrics.timestamp.format("%H:%M:%S"));
                println!("CPU:    {:.2}%", metrics.cpu_stats.usage_percent);
                println!(
                    "Memory: {} MB ({:.2}%)",
                    metrics.memory_stats.usage / 1024 / 1024,
                    metrics.memory_stats.usage_percent
                );
                println!(
                    "Network RX/TX: {} / {} bytes",
                    metrics.network_stats.rx_bytes,
                    metrics.network_stats.tx_bytes
                );
                println!(
                    "Disk R/W: {} / {} bytes\n",
                    metrics.disk_stats.read_bytes,
                    metrics.disk_stats.write_bytes
                );
            }
            Err(e) => {
                eprintln!("Error getting metrics: {}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}
