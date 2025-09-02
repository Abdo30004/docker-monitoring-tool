# Docker Monitor Architecture

## Overview

The Docker Monitor library is designed with a modular architecture that separates concerns and provides a clean, type-safe API for Docker container monitoring.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Client Application                       │
│                  (User's Rust Application)                   │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ Uses Public API
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                      DockerMonitor                           │
│                    (monitor.rs - Main API)                   │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  new() - Create monitor instance                      │   │
│  │  list_containers() - Get running containers          │   │
│  │  get_container_details() - Get container info        │   │
│  │  start_container() - Start a container               │   │
│  │  stop_container() - Stop a container                 │   │
│  │  get_container_metrics() - Get real-time metrics     │   │
│  │  monitor_events() - Listen for Docker events         │   │
│  └──────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ Uses Bollard Client
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                     Bollard Docker Client                    │
│                  (Docker API Communication)                  │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ Docker API (Unix socket/TCP)
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                      Docker Daemon                           │
│                    (Container Runtime)                       │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

1. **lib.rs** - Public API and module exports

   - Entry point for the library
   - Re-exports all public types and functions
   - Provides library-level documentation

2. **monitor.rs** - Main implementation

   - `DockerMonitor` struct - Core monitoring client
   - All container management functions
   - Metrics retrieval implementation
   - Event monitoring logic

3. **metrics.rs** - Metrics data structures

   - `ContainerMetrics` - Container-level metrics
   - `CpuStats` - CPU usage statistics
   - `MemoryStats` - Memory usage statistics
   - `NetworkStats` - Network I/O statistics
   - `DiskStats` - Disk I/O statistics

4. **container.rs** - Container information

   - `ContainerInfo` - Container metadata
   - `ContainerStatus` - Status enumeration
   - `PortMapping` - Port mapping info

5. **events.rs** - Event handling

   - `DockerEvent` - Event data structure
   - `EventType` - Event type enumeration
   - `EventCallback` - Callback type definition

6. **error.rs** - Error handling
   - `DockerMonitorError` - Custom error types
   - `Result<T>` - Convenient result type

## Data Flow

### Metrics Retrieval Flow

```
User Request
    ↓
DockerMonitor::get_container_metrics()
    ↓
Bollard stats() API call
    ↓
Raw Stats from Docker
    ↓
Parse CPU/Memory/Network/Disk stats
    ↓
Create ContainerMetrics
    ↓
Return to User
```

### Event Monitoring Flow

```
User provides EventCallback
    ↓
DockerMonitor::monitor_events()
    ↓
Bollard events() stream
    ↓
Async stream processing loop
    ↓
Parse EventMessage
    ↓
Create DockerEvent
    ↓
Invoke user's callback
    ↓
Continue listening
```

### Container Management Flow

```
User Request (start/stop/list)
    ↓
DockerMonitor method
    ↓
Bollard API call
    ↓
Docker Daemon action
    ↓
Success/Error response
    ↓
Return Result to User
```

## Error Handling Strategy

```
Operation Attempt
    ↓
Success? ──Yes──→ Return Ok(result)
    │
    No
    ↓
Map to DockerMonitorError
    ↓
Add context
    ↓
Return Err(error)
```

## Thread Safety

- `DockerMonitor` uses `Docker` client from Bollard (internally thread-safe)
- Event callbacks use `Arc<dyn Fn>` for thread-safe sharing
- All async operations use Tokio runtime
- No manual synchronization primitives needed

## Performance Considerations

1. **Async Operations** - All I/O operations are async for non-blocking execution
2. **Streaming** - Events use streaming API to avoid polling
3. **One-shot Stats** - Metrics use one-shot mode to get instant snapshots
4. **Zero-copy** - Minimal data copying where possible

## Extension Points

The library can be extended by:

1. Adding new metric types in `metrics.rs`
2. Supporting additional event types in `events.rs`
3. Adding new container management operations in `monitor.rs`
4. Implementing custom error types in `error.rs`

## Dependencies

- **bollard** - Docker API client
- **tokio** - Async runtime
- **serde** - Serialization framework
- **chrono** - Date/time handling
- **thiserror** - Error type derivation
- **futures** - Async utilities
