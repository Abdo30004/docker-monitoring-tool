use docker_monitor::DockerMonitor;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Docker Monitor - Container Management Example\n");

    let monitor = DockerMonitor::new().await?;

    // List all running containers
    let containers = monitor.list_containers().await?;

    if containers.is_empty() {
        println!("No running containers found.");
        println!("\nTip: Start a container with:");
        println!("  docker run -d --name test-container nginx");
        return Ok(());
    }

    println!("Found {} running container(s)\n", containers.len());

    for container in &containers {
        println!("Container: {}", container.name);
        println!("  ID: {}", container.id);
        println!("  Image: {}", container.image);
        println!("  Status: {}", container.status);

        // Get detailed information
        match monitor.get_container_details(&container.id).await {
            Ok(details) => {
                println!("  State: {}", details.state);
                println!("  Labels: {:?}", details.labels);
            }
            Err(e) => println!("  Error getting details: {}", e),
        }

        println!();
    }

    // Example: Stop and start a container (commented out for safety)
    // Uncomment and modify the container name to use
    /*
    let container_name = "test-container";
    
    println!("Stopping container: {}", container_name);
    monitor.stop_container(container_name).await?;
    println!("✓ Container stopped");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    println!("Starting container: {}", container_name);
    monitor.start_container(container_name).await?;
    println!("✓ Container started");
    */

    Ok(())
}
