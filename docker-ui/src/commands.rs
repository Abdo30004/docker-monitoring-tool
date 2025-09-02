use docker_monitor::{ DockerMonitor, DockerEvent, EventType };
use anyhow::{ Context, Result };
use colored::*;
use std::sync::Arc;
use tokio::time::{ sleep, Duration };

use crate::display;

/// List all containers
pub async fn list_containers(_all: bool, format: &str) -> Result<()> {
    let spinner = display::create_spinner("Connecting to Docker daemon...");

    // TODO: Implement --all flag to show stopped containers
    let monitor = DockerMonitor::new().await.context(
        "Failed to connect to Docker daemon. Is Docker running?"
    )?;

    spinner.finish_and_clear();

    let containers = monitor.list_containers().await.context("Failed to list containers")?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&containers)?;
            println!("{}", json);
        }
        "compact" => {
            display::display_containers_compact(&containers);
        }
        _ => {
            display::display_containers_table(&containers);
        }
    }

    Ok(())
}

/// Show detailed information about a specific container
pub async fn container_info(container_id: &str) -> Result<()> {
    let spinner = display::create_spinner(&format!("Fetching information for {}...", container_id));

    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    let container = monitor
        .get_container_details(container_id).await
        .context(format!("Failed to get details for container '{}'", container_id))?;

    spinner.finish_and_clear();

    display::display_container_details(&container);

    Ok(())
}

/// Display metrics for a specific container
pub async fn container_metrics(container_id: &str, follow: bool, interval: u64) -> Result<()> {
    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    if follow {
        display::print_info(
            &format!(
                "Monitoring {} (refresh every {}s, press Ctrl+C to stop)",
                container_id.bright_green(),
                interval
            )
        );
        println!();

        loop {
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");
            display::print_header();

            match monitor.get_container_metrics(container_id).await {
                Ok(metrics) => {
                    display::display_metrics(&metrics);
                }
                Err(e) => {
                    display::print_error(&format!("Failed to get metrics: {}", e));
                    break;
                }
            }

            sleep(Duration::from_secs(interval)).await;
        }
    } else {
        let spinner = display::create_spinner(&format!("Fetching metrics for {}...", container_id));

        let metrics = monitor
            .get_container_metrics(container_id).await
            .context(format!("Failed to get metrics for container '{}'", container_id))?;

        spinner.finish_and_clear();

        display::display_metrics(&metrics);
    }

    Ok(())
}

/// Display dashboard with metrics for all running containers
pub async fn dashboard(interval: u64) -> Result<()> {
    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    display::print_info(&format!("Dashboard (refresh every {}s, press Ctrl+C to stop)", interval));
    println!();

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        display::print_header();

        display::print_section("Container Dashboard");

        match monitor.list_containers().await {
            Ok(containers) => {
                let mut metrics_list = Vec::new();

                for container in &containers {
                    if let Ok(metrics) = monitor.get_container_metrics(&container.id).await {
                        metrics_list.push(metrics);
                    }
                }

                if !metrics_list.is_empty() {
                    display::display_metrics_table(&metrics_list);
                } else {
                    display::print_warning("No metrics available");
                }

                println!();
                display::print_info(
                    &format!("Last updated: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
                );
            }
            Err(e) => {
                display::print_error(&format!("Failed to fetch containers: {}", e));
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }
}

/// Start a container
pub async fn start_container(container_id: &str) -> Result<()> {
    let spinner = display::create_spinner(&format!("Starting container {}...", container_id));

    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    monitor
        .start_container(container_id).await
        .context(format!("Failed to start container '{}'", container_id))?;

    spinner.finish_and_clear();

    display::print_success(&format!("Container '{}' started successfully", container_id.green()));

    Ok(())
}

/// Stop a container
pub async fn stop_container(container_id: &str) -> Result<()> {
    let spinner = display::create_spinner(&format!("Stopping container {}...", container_id));

    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    monitor
        .stop_container(container_id).await
        .context(format!("Failed to stop container '{}'", container_id))?;

    spinner.finish_and_clear();

    display::print_success(&format!("Container '{}' stopped successfully", container_id.green()));

    Ok(())
}

/// Monitor Docker events in real-time
pub async fn monitor_events(filter: Option<String>) -> Result<()> {
    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    display::print_info("Monitoring Docker events (press Ctrl+C to stop)...");
    display::print_section("Event Stream");

    let filter_clone = filter.clone();
    let callback = Arc::new(move |event: DockerEvent| {
        // Apply filter if specified
        if let Some(ref filter_str) = filter_clone {
            let event_name = format!("{:?}", event.event_type).to_lowercase();
            if !event_name.contains(&filter_str.to_lowercase()) {
                return;
            }
        }

        let timestamp = event.timestamp.format("%H:%M:%S").to_string().dimmed();
        let event_type_colored = match event.event_type {
            EventType::ContainerStart => format!("{}", event.event_type).green(),
            EventType::ContainerStop => format!("{}", event.event_type).yellow(),
            EventType::ContainerDie | EventType::ContainerKill =>
                format!("{}", event.event_type).red().bold(),
            EventType::ContainerOom => format!("{}", event.event_type).red().on_yellow(),
            EventType::ContainerPause | EventType::ContainerUnpause =>
                format!("{}", event.event_type).cyan(),
            EventType::ContainerCreate => format!("{}", event.event_type).blue(),
            EventType::ContainerDestroy => format!("{}", event.event_type).red(),
            EventType::ContainerHealthStatus => format!("{}", event.event_type).magenta(),
            _ => format!("{}", event.event_type).white(),
        };

        println!(
            "[{}] {} - {} ({})",
            timestamp,
            event_type_colored,
            event.container_name.bright_white().bold(),
            event.container_id[..12].dimmed()
        );
    });

    monitor.monitor_events(callback).await.context("Failed to monitor events")?;

    Ok(())
}

/// Display system-wide Docker information
pub async fn system_info() -> Result<()> {
    let spinner = display::create_spinner("Gathering system information...");

    let monitor = DockerMonitor::new().await.context("Failed to connect to Docker daemon")?;

    let containers = monitor.list_containers().await.context("Failed to list containers")?;

    spinner.finish_and_clear();

    display::print_section("Docker System Information");

    println!("{:<25} {}", "Docker Daemon:".bright_white().bold(), "Connected ✓".green());
    println!(
        "{:<25} {}",
        "Running Containers:".bright_white().bold(),
        containers.len().to_string().cyan().bold()
    );

    // Count by status
    let running = containers
        .iter()
        .filter(|c| matches!(c.status, docker_monitor::ContainerStatus::Running))
        .count();
    let stopped = containers.len() - running;

    println!(
        "{:<25} {} running, {} stopped",
        "Container Status:".bright_white().bold(),
        running.to_string().green(),
        stopped.to_string().red()
    );

    // Collect unique images
    let unique_images: std::collections::HashSet<_> = containers
        .iter()
        .map(|c| c.image.as_str())
        .collect();

    println!(
        "{:<25} {}",
        "Unique Images:".bright_white().bold(),
        unique_images.len().to_string().cyan()
    );

    println!();
    display::print_info("Use 'docker-ui list' to see all containers");
    display::print_info("Use 'docker-ui dashboard' for real-time monitoring");

    Ok(())
}
