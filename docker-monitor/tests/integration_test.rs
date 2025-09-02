use docker_monitor::{ DockerMonitor, ContainerStatus };

#[tokio::test]
async fn test_docker_monitor_creation() {
    // This test will only pass if Docker daemon is running
    let result = DockerMonitor::new().await;

    // We don't assert success because Docker might not be available in CI
    match result {
        Ok(_) => println!("Successfully connected to Docker daemon"),
        Err(e) => println!("Docker daemon not available: {}", e),
    }
}

#[tokio::test]
async fn test_list_containers() {
    let monitor = match DockerMonitor::new().await {
        Ok(m) => m,
        Err(_) => {
            println!("Skipping test: Docker daemon not available");
            return;
        }
    };

    let result = monitor.list_containers().await;
    assert!(result.is_ok(), "Failed to list containers");

    let containers = result.unwrap();
    println!("Found {} running containers", containers.len());
}

#[test]
fn test_container_status_from_string() {
    assert_eq!(ContainerStatus::from("running"), ContainerStatus::Running);
    assert_eq!(ContainerStatus::from("stopped"), ContainerStatus::Stopped);
    assert_eq!(ContainerStatus::from("paused"), ContainerStatus::Paused);
    assert_eq!(ContainerStatus::from("unknown"), ContainerStatus::Unknown);
}

#[test]
fn test_container_status_display() {
    assert_eq!(ContainerStatus::Running.to_string(), "Running");
    assert_eq!(ContainerStatus::Stopped.to_string(), "Stopped");
    assert_eq!(ContainerStatus::Paused.to_string(), "Paused");
}
