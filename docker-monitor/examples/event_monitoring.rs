use docker_monitor::{ DockerMonitor, DockerEvent };
use std::sync::Arc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Docker Monitor - Event Monitoring Example\n");
    println!("Listening for Docker events... (Press Ctrl+C to stop)\n");

    // Create a new monitor instance
    let monitor = DockerMonitor::new().await?;

    // Define a callback function
    let callback = Arc::new(|event: DockerEvent| {
        println!(
            "[{}] {} - Container: {} (ID: {})",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.event_type,
            event.container_name,
            &event.container_id[..12]
        );
    });

    // Start monitoring events
    monitor.monitor_events(callback).await?;

    Ok(())
}
