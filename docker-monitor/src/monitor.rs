use bollard::Docker;
use bollard::container::{ ListContainersOptions, StatsOptions, InspectContainerOptions };
use bollard::system::EventsOptions;
use bollard::models::EventMessage;
use futures::stream::StreamExt;
use std::collections::HashMap;
use chrono::Utc;

use crate::error::{ DockerMonitorError, Result };
use crate::metrics::{ ContainerMetrics, CpuStats, MemoryStats, NetworkStats, DiskStats };
use crate::container::{ ContainerInfo, ContainerStatus, PortMapping };
use crate::events::{ DockerEvent, EventType, EventCallback };

/// Main Docker monitoring client
pub struct DockerMonitor {
    docker: Docker,
}

impl DockerMonitor {
    /// Create a new Docker monitor instance
    ///
    /// # Example
    ///
    /// ```no_run
    /// use docker_monitor::DockerMonitor;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let monitor = DockerMonitor::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults().map_err(|e|
            DockerMonitorError::DaemonUnavailable(e.to_string())
        )?;

        // Test connection
        docker
            .ping().await
            .map_err(|e| {
                DockerMonitorError::DaemonUnavailable(
                    format!("Failed to connect to Docker daemon: {}", e)
                )
            })?;

        Ok(Self { docker })
    }

    /// List all running containers
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use docker_monitor::DockerMonitor;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let monitor = DockerMonitor::new().await?;
    /// let containers = monitor.list_containers().await?;
    /// for container in containers {
    ///     println!("{}: {}", container.name, container.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        let mut filters = HashMap::new();
        filters.insert("status", vec!["running"]);

        let options = Some(ListContainersOptions {
            all: false,
            filters,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;

        let mut result = Vec::new();
        for container in containers {
            let id = container.id.unwrap_or_default();
            let names = container.names.unwrap_or_default();
            let name = names.first().unwrap_or(&String::new()).trim_start_matches('/').to_string();
            let image = container.image.unwrap_or_default();
            let state = container.state.unwrap_or_default();
            let status = ContainerStatus::from(state.as_str());
            let created = Utc::now(); // Simplified for now
            let labels = container.labels.unwrap_or_default();

            let ports = container.ports
                .unwrap_or_default()
                .iter()
                .map(|p| PortMapping {
                    private_port: p.private_port,
                    public_port: p.public_port,
                    port_type: p.typ
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or_default(),
                })
                .collect();

            result.push(ContainerInfo {
                id,
                name,
                image,
                status,
                state,
                created,
                labels,
                ports,
            });
        }

        Ok(result)
    }

    /// Get detailed information about a specific container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container
    pub async fn get_container_details(&self, container_id: &str) -> Result<ContainerInfo> {
        let options = InspectContainerOptions { size: false };
        let container = self.docker
            .inspect_container(container_id, Some(options)).await
            .map_err(|_| DockerMonitorError::ContainerNotFound(container_id.to_string()))?;

        let id = container.id.unwrap_or_default();
        let name = container.name.unwrap_or_default().trim_start_matches('/').to_string();
        let image = container.image.unwrap_or_default();

        let state_obj = container.state.unwrap_or_default();
        let state = state_obj.status
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let status = ContainerStatus::from(state.as_str());

        let created = Utc::now(); // Simplified
        let labels = container.config.and_then(|c| c.labels).unwrap_or_default();

        Ok(ContainerInfo {
            id,
            name,
            image,
            status,
            state,
            created,
            labels,
            ports: Vec::new(), // Simplified
        })
    }

    /// Start a container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container to start
    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        self.docker
            .start_container::<String>(container_id, None).await
            .map_err(|e| DockerMonitorError::ApiError(e))?;
        Ok(())
    }

    /// Stop a container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container to stop
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        self.docker
            .stop_container(container_id, None).await
            .map_err(|e| DockerMonitorError::ApiError(e))?;
        Ok(())
    }

    /// Get real-time metrics for a container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID or name of the container
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use docker_monitor::DockerMonitor;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let monitor = DockerMonitor::new().await?;
    /// let metrics = monitor.get_container_metrics("my-container").await?;
    /// println!("CPU Usage: {:.2}%", metrics.cpu_stats.usage_percent);
    /// println!("Memory Usage: {} MB", metrics.memory_stats.usage / 1024 / 1024);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_container_metrics(&self, container_id: &str) -> Result<ContainerMetrics> {
        let options = StatsOptions {
            stream: false,
            one_shot: true,
        };

        let mut stats_stream = self.docker.stats(container_id, Some(options));

        if let Some(stats_result) = stats_stream.next().await {
            let stats = stats_result.map_err(|e| {
                DockerMonitorError::MetricsUnavailable(format!("Failed to get stats: {}", e))
            })?;

            // Parse CPU stats
            let cpu_stats = {
                let cpu_delta =
                    stats.cpu_stats.cpu_usage.total_usage -
                    stats.precpu_stats.cpu_usage.total_usage;
                let system_delta =
                    stats.cpu_stats.system_cpu_usage.unwrap_or(0) -
                    stats.precpu_stats.system_cpu_usage.unwrap_or(0);
                let online_cpus = stats.cpu_stats.online_cpus.unwrap_or(1);

                let usage_percent = if system_delta > 0 {
                    ((cpu_delta as f64) / (system_delta as f64)) * (online_cpus as f64) * 100.0
                } else {
                    0.0
                };

                CpuStats {
                    usage_percent,
                    total_usage: stats.cpu_stats.cpu_usage.total_usage,
                    system_cpu_usage: stats.cpu_stats.system_cpu_usage.unwrap_or(0),
                    online_cpus,
                }
            };

            // Parse memory stats
            let memory_stats = {
                let usage = stats.memory_stats.usage.unwrap_or(0);
                let limit = stats.memory_stats.limit.unwrap_or(1);
                let usage_percent = if limit > 0 {
                    ((usage as f64) / (limit as f64)) * 100.0
                } else {
                    0.0
                };

                MemoryStats {
                    usage,
                    max_usage: stats.memory_stats.max_usage.unwrap_or(0),
                    limit,
                    usage_percent,
                }
            };

            // Parse network stats
            let network_stats = if let Some(networks) = stats.networks {
                let mut total_rx = 0u64;
                let mut total_tx = 0u64;
                let mut total_rx_packets = 0u64;
                let mut total_tx_packets = 0u64;
                let mut total_rx_errors = 0u64;
                let mut total_tx_errors = 0u64;

                for (_, net_stats) in networks {
                    total_rx += net_stats.rx_bytes;
                    total_tx += net_stats.tx_bytes;
                    total_rx_packets += net_stats.rx_packets;
                    total_tx_packets += net_stats.tx_packets;
                    total_rx_errors += net_stats.rx_errors;
                    total_tx_errors += net_stats.tx_errors;
                }

                NetworkStats {
                    rx_bytes: total_rx,
                    tx_bytes: total_tx,
                    rx_packets: total_rx_packets,
                    tx_packets: total_tx_packets,
                    rx_errors: total_rx_errors,
                    tx_errors: total_tx_errors,
                }
            } else {
                NetworkStats::new()
            };

            // Parse disk stats
            let disk_stats = {
                let mut read_bytes = 0u64;
                let mut write_bytes = 0u64;

                if let Some(io_service_bytes) = &stats.blkio_stats.io_service_bytes_recursive {
                    for entry in io_service_bytes {
                        match entry.op.as_str() {
                            "Read" => {
                                read_bytes += entry.value;
                            }
                            "Write" => {
                                write_bytes += entry.value;
                            }
                            _ => {}
                        }
                    }
                }

                let mut read_ops = 0u64;
                let mut write_ops = 0u64;

                if let Some(io_serviced) = &stats.blkio_stats.io_serviced_recursive {
                    for entry in io_serviced {
                        match entry.op.as_str() {
                            "Read" => {
                                read_ops += entry.value;
                            }
                            "Write" => {
                                write_ops += entry.value;
                            }
                            _ => {}
                        }
                    }
                }

                DiskStats {
                    read_bytes,
                    write_bytes,
                    read_ops,
                    write_ops,
                }
            };

            // Get container name
            let container_name = stats.name.clone();

            Ok(ContainerMetrics {
                container_id: container_id.to_string(),
                container_name,
                timestamp: Utc::now(),
                cpu_stats,
                memory_stats,
                network_stats,
                disk_stats,
            })
        } else {
            Err(DockerMonitorError::MetricsUnavailable("No stats available".to_string()))
        }
    }

    /// Monitor Docker events and trigger callbacks
    ///
    /// # Arguments
    ///
    /// * `callback` - A callback function to be called when an event occurs
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use docker_monitor::{DockerMonitor, DockerEvent};
    /// # use std::sync::Arc;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let monitor = DockerMonitor::new().await?;
    ///
    /// let callback = Arc::new(|event: DockerEvent| {
    ///     println!("Event: {} - Container: {}", event.event_type, event.container_name);
    /// });
    ///
    /// monitor.monitor_events(callback).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn monitor_events(&self, callback: EventCallback) -> Result<()> {
        let options = Some(EventsOptions::<String> {
            filters: HashMap::from([("type".to_string(), vec!["container".to_string()])]),
            ..Default::default()
        });

        let mut events = self.docker.events(options);

        while let Some(event_result) = events.next().await {
            match event_result {
                Ok(event) => {
                    if let Some(docker_event) = self.parse_event(event) {
                        callback(docker_event);
                    }
                }
                Err(e) => {
                    return Err(DockerMonitorError::EventError(e.to_string()));
                }
            }
        }

        Ok(())
    }

    fn parse_event(&self, event: EventMessage) -> Option<DockerEvent> {
        let action = event.action?;
        let actor = event.actor?;
        let container_id = actor.id?;

        let attributes = actor.attributes.unwrap_or_default();
        let container_name = attributes
            .get("name")
            .cloned()
            .unwrap_or_else(|| container_id.clone());

        let event_type = EventType::from(action.as_str());
        let timestamp = Utc::now();

        Some(DockerEvent {
            event_type,
            container_id,
            container_name,
            timestamp,
            attributes,
        })
    }
}
