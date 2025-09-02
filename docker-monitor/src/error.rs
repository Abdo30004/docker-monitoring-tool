use thiserror::Error;

/// Custom error type for the Docker monitoring library
#[derive(Error, Debug)]
pub enum DockerMonitorError {
    #[error("Docker daemon is unavailable: {0}")] DaemonUnavailable(String),

    #[error("Docker API error: {0}")] ApiError(#[from] bollard::errors::Error),

    #[error("Container not found: {0}")] ContainerNotFound(String),

    #[error("Invalid container ID: {0}")] InvalidContainerId(String),

    #[error("Metrics unavailable: {0}")] MetricsUnavailable(String),

    #[error("Event monitoring error: {0}")] EventError(String),

    #[error("Serialization error: {0}")] SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")] IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")] Unknown(String),
}

/// Result type alias for Docker monitoring operations
pub type Result<T> = std::result::Result<T, DockerMonitorError>;
