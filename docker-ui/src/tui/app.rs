use docker_monitor::{ DockerMonitor, ContainerInfo, ContainerMetrics, ContainerStatus };
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Overview,
    Containers,
    Metrics,
}

impl Tab {
    pub fn title(&self) -> &str {
        match self {
            Tab::Overview => "Overview",
            Tab::Containers => "Containers",
            Tab::Metrics => "Metrics",
        }
    }

    pub fn all() -> Vec<Tab> {
        vec![Tab::Overview, Tab::Containers, Tab::Metrics]
    }
}

pub struct App {
    pub current_tab: Tab,
    pub containers: Vec<ContainerInfo>,
    pub metrics: HashMap<String, ContainerMetrics>,
    pub selected_index: usize,
    pub refresh_interval: u64,
    pub total_containers: usize,
    pub running_containers: usize,
    pub error_message: Option<String>,
}

impl App {
    pub fn new(refresh_interval: u64) -> Self {
        Self {
            current_tab: Tab::Overview,
            containers: Vec::new(),
            metrics: HashMap::new(),
            selected_index: 0,
            refresh_interval,
            total_containers: 0,
            running_containers: 0,
            error_message: None,
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        self.error_message = None;

        match DockerMonitor::new().await {
            Ok(monitor) => {
                // Fetch containers
                match monitor.list_containers().await {
                    Ok(containers) => {
                        self.total_containers = containers.len();
                        self.running_containers = containers
                            .iter()
                            .filter(|c| matches!(c.status, ContainerStatus::Running))
                            .count();
                        self.containers = containers;

                        // Fetch metrics for running containers
                        self.metrics.clear();
                        for container in &self.containers {
                            if matches!(container.status, ContainerStatus::Running) {
                                if
                                    let Ok(metrics) = monitor.get_container_metrics(
                                        &container.id
                                    ).await
                                {
                                    self.metrics.insert(container.id.clone(), metrics);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to fetch containers: {}", e));
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Docker daemon unavailable: {}", e));
            }
        }

        Ok(())
    }

    pub fn next(&mut self) {
        if !self.containers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.containers.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.containers.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.containers.len() - 1;
            }
        }
    }

    pub fn next_tab(&mut self) {
        let tabs = Tab::all();
        let current_index = tabs
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        self.current_tab = tabs[(current_index + 1) % tabs.len()];
    }

    pub fn previous_tab(&mut self) {
        let tabs = Tab::all();
        let current_index = tabs
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        if current_index > 0 {
            self.current_tab = tabs[current_index - 1];
        } else {
            self.current_tab = tabs[tabs.len() - 1];
        }
    }

    pub fn select_tab(&mut self, index: usize) {
        let tabs = Tab::all();
        if index < tabs.len() {
            self.current_tab = tabs[index];
        }
    }

    pub fn get_selected_container(&self) -> Option<&ContainerInfo> {
        self.containers.get(self.selected_index)
    }

    pub fn get_selected_metrics(&self) -> Option<&ContainerMetrics> {
        self.get_selected_container().and_then(|c| self.metrics.get(&c.id))
    }
}
