use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };
use std::sync::Arc;

/// Docker event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerEvent {
    /// Event type
    pub event_type: EventType,
    /// Container ID
    pub container_id: String,
    /// Container name
    pub container_name: String,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Additional event attributes
    pub attributes: std::collections::HashMap<String, String>,
}

/// Types of Docker events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    ContainerStart,
    ContainerStop,
    ContainerDie,
    ContainerKill,
    ContainerPause,
    ContainerUnpause,
    ContainerRestart,
    ContainerCreate,
    ContainerDestroy,
    ContainerOom,
    ContainerHealthStatus,
    Unknown(String),
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "start" => EventType::ContainerStart,
            "stop" => EventType::ContainerStop,
            "die" => EventType::ContainerDie,
            "kill" => EventType::ContainerKill,
            "pause" => EventType::ContainerPause,
            "unpause" => EventType::ContainerUnpause,
            "restart" => EventType::ContainerRestart,
            "create" => EventType::ContainerCreate,
            "destroy" => EventType::ContainerDestroy,
            "oom" => EventType::ContainerOom,
            "health_status" => EventType::ContainerHealthStatus,
            _ => EventType::Unknown(s.to_string()),
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::ContainerStart => write!(f, "Container Start"),
            EventType::ContainerStop => write!(f, "Container Stop"),
            EventType::ContainerDie => write!(f, "Container Die"),
            EventType::ContainerKill => write!(f, "Container Kill"),
            EventType::ContainerPause => write!(f, "Container Pause"),
            EventType::ContainerUnpause => write!(f, "Container Unpause"),
            EventType::ContainerRestart => write!(f, "Container Restart"),
            EventType::ContainerCreate => write!(f, "Container Create"),
            EventType::ContainerDestroy => write!(f, "Container Destroy"),
            EventType::ContainerOom => write!(f, "Container OOM"),
            EventType::ContainerHealthStatus => write!(f, "Container Health Status"),
            EventType::Unknown(s) => write!(f, "Unknown: {}", s),
        }
    }
}

/// Callback function type for event monitoring
pub type EventCallback = Arc<dyn Fn(DockerEvent) + Send + Sync>;
