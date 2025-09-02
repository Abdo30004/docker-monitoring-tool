use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };

/// Container information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    /// Container ID
    pub id: String,
    /// Container name
    pub name: String,
    /// Container image
    pub image: String,
    /// Container status
    pub status: ContainerStatus,
    /// Container state
    pub state: String,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Container labels
    pub labels: std::collections::HashMap<String, String>,
    /// Port mappings
    pub ports: Vec<PortMapping>,
}

/// Container status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Paused,
    Restarting,
    Dead,
    Created,
    Exited,
    Unknown,
}

impl From<&str> for ContainerStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "running" => ContainerStatus::Running,
            "stopped" => ContainerStatus::Stopped,
            "paused" => ContainerStatus::Paused,
            "restarting" => ContainerStatus::Restarting,
            "dead" => ContainerStatus::Dead,
            "created" => ContainerStatus::Created,
            "exited" => ContainerStatus::Exited,
            _ => ContainerStatus::Unknown,
        }
    }
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerStatus::Running => write!(f, "Running"),
            ContainerStatus::Stopped => write!(f, "Stopped"),
            ContainerStatus::Paused => write!(f, "Paused"),
            ContainerStatus::Restarting => write!(f, "Restarting"),
            ContainerStatus::Dead => write!(f, "Dead"),
            ContainerStatus::Created => write!(f, "Created"),
            ContainerStatus::Exited => write!(f, "Exited"),
            ContainerStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Port mapping information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub port_type: String,
}
