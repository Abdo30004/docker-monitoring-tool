# Docker UI - CLI Interface for Docker Monitoring

A beautiful and intuitive command-line interface for monitoring and managing Docker containers, built on top of the docker-monitor library.

## Features

- 🖥️ **Interactive TUI Monitor** - Full-screen terminal user interface with real-time updates
- 🎨 **Beautiful Output** - Color-coded, formatted output for easy reading
- 📊 **Real-time Metrics** - Monitor CPU, memory, network, and disk I/O
- 📋 **Container Management** - List, start, stop, and inspect containers
- 🔄 **Live Dashboard** - Real-time monitoring of all containers
- 📡 **Event Monitoring** - Watch Docker events as they happen
- 🎯 **Multiple Formats** - Table, JSON, and compact output formats

## Installation

### Build from Source

```bash
cd docker-ui
cargo build --release
```

The binary will be available at `target/release/docker-ui` (or `docker-ui.exe` on Windows).

### Add to PATH (Optional)

You can copy the binary to a directory in your PATH for easy access:

```bash
# Linux/macOS
sudo cp target/release/docker-ui /usr/local/bin/

# Windows (run as Administrator in PowerShell)
copy target\release\docker-ui.exe C:\Windows\System32\
```

## Usage

### Basic Commands

#### List Containers

```bash
# List all running containers (table format)
docker-ui list

# List with compact format
docker-ui list --format compact

# List with JSON output
docker-ui list --format json

# Short alias
docker-ui ls
```

**Output Example:**

```
╔═══════════════════════════════════════════════════════════════╗
║            🐳 Docker Container Monitor CLI 🐳              ║
╚═══════════════════════════════════════════════════════════════╝

╭────────────┬─────────────┬──────────────┬─────────┬─────────╮
│ ID         │ Name        │ Image        │ Status  │ State   │
├────────────┼─────────────┼──────────────┼─────────┼─────────┤
│ a1b2c3d4e5 │ web-server  │ nginx:latest │ Running │ running │
│ f6g7h8i9j0 │ db-server   │ postgres:14  │ Running │ running │
╰────────────┴─────────────┴──────────────┴─────────┴─────────╯

ℹ Total: 2 container(s)
```

#### Container Information

```bash
# Show detailed information about a container
docker-ui info <container-name-or-id>

# Alias
docker-ui inspect web-server
```

**Output Example:**

```
━━━ Container Information ━━━

ID:                  a1b2c3d4e5f6g7h8i9j0
Name:                web-server
Image:               nginx:latest
Status:              Running
State:               running
Created:             2025-11-04 10:30:00

━━━ Port Mappings ━━━

  80 tcp → 8080 tcp
  443 tcp → 8443 tcp
```

#### Container Metrics

```bash
# Show current metrics for a container
docker-ui metrics <container-name-or-id>

# Continuous monitoring (refreshes every 2 seconds)
docker-ui metrics <container-name-or-id> --follow

# Custom refresh interval (5 seconds)
docker-ui metrics web-server --follow --interval 5

# Alias
docker-ui stats web-server
```

**Output Example:**

```
━━━ Metrics for web-server ━━━

Timestamp:           2025-11-04 15:30:45

CPU Usage:
  Usage:             15.32%
  Total Usage:       1234567890 ns
  Online CPUs:       4

Memory Usage:
  Usage:             256.00 MB (12.50%)
  Limit:             2.00 GB
  Max Usage:         512.00 MB

Network I/O:
  Bytes:             1.25 GB ⬇  512.00 MB ⬆
  Packets:           1000000 ⬇  500000 ⬆

Disk I/O:
  Bytes:             128.00 MB 📖  64.00 MB 📝
  Operations:        1000 📖  500 📝
```

#### Interactive TUI Monitor (NEW! 🎉)

Launch a beautiful full-screen terminal interface for monitoring Docker containers:

```bash
# Start the TUI monitor
docker-ui monitor

# Start with custom refresh interval (2 seconds)
docker-ui monitor --interval 2
```

**TUI Features:**

- 📊 **Overview Tab** - System-wide container statistics and summary
- 📋 **Containers Tab** - Navigate through all containers with arrow keys
- 📈 **Metrics Tab** - Detailed metrics for selected container with visual gauges
- ⌨️ **Keyboard Controls:**
  - `q` or `ESC` - Quit
  - `TAB` - Switch between tabs
  - `1`, `2`, `3` - Quick tab navigation
  - `↑`/`↓` or `j`/`k` - Navigate containers
  - `r` - Manual refresh
- 🔄 **Auto-refresh** - Automatic updates at specified interval
- 🎨 **Color-coded** - Status indicators and resource usage

#### Dashboard

```bash
# Monitor all running containers
docker-ui dashboard

# Custom refresh interval (3 seconds)
docker-ui dashboard --interval 3
```

**Output Example:**

```
━━━ Container Dashboard ━━━

╭──────────────┬────────┬────────────┬──────────┬─────────────────────┬────────────────────╮
│ Container    │ CPU %  │ Memory     │ Memory % │ Net I/O             │ Disk I/O           │
├──────────────┼────────┼────────────┼──────────┼─────────────────────┼────────────────────┤
│ web-server   │ 15.32% │ 256.00 MB  │ 12.50%   │ ⬇ 1.25 GB / ⬆ 512 MB│ 📖 128 MB / 📝 64 MB│
│ db-server    │ 45.67% │ 1.50 GB    │ 75.00%   │ ⬇ 2.10 GB / ⬆ 1.2 GB│ 📖 512 MB / 📝 256 MB│
╰──────────────┴────────┴────────────┴──────────┴─────────────────────┴────────────────────╯

ℹ Last updated: 2025-11-04 15:35:20
```

#### Start/Stop Containers

```bash
# Start a stopped container
docker-ui start <container-name-or-id>

# Stop a running container
docker-ui stop <container-name-or-id>
```

**Output Example:**

```
✓ Container 'web-server' started successfully
```

#### Monitor Events

```bash
# Monitor all Docker events
docker-ui events

# Filter events (e.g., only start events)
docker-ui events --filter start
```

**Output Example:**

```
ℹ Monitoring Docker events (press Ctrl+C to stop)...

━━━ Event Stream ━━━

[15:30:12] Container Start - web-server (a1b2c3d4e5)
[15:30:45] Container Stop - db-server (f6g7h8i9j0)
[15:31:20] Container Die - old-container (k1l2m3n4o5)
```

#### System Information

```bash
# Show Docker system information
docker-ui system
```

**Output Example:**

```
━━━ Docker System Information ━━━

Docker Daemon:            Connected ✓
Running Containers:       5
Container Status:         3 running, 2 stopped
Unique Images:            4

ℹ Use 'docker-ui list' to see all containers
ℹ Use 'docker-ui dashboard' for real-time monitoring
```

## Command Reference

### Commands

| Command     | Alias     | Description                         |
| ----------- | --------- | ----------------------------------- |
| `list`      | `ls`      | List all running containers         |
| `info`      | `inspect` | Show detailed container information |
| `metrics`   | `stats`   | Display container metrics           |
| `dashboard` | -         | Monitor all containers in real-time |
| `start`     | -         | Start a stopped container           |
| `stop`      | -         | Stop a running container            |
| `events`    | -         | Monitor Docker events               |
| `system`    | -         | Show Docker system information      |

### Global Options

| Option          | Description               |
| --------------- | ------------------------- |
| `-h, --help`    | Print help information    |
| `-V, --version` | Print version information |

### Command-Specific Options

#### `list`

- `-a, --all` - Show all containers (including stopped)
- `-f, --format <FORMAT>` - Output format: table, json, compact (default: table)

#### `metrics`

- `-f, --follow` - Continuous monitoring
- `-i, --interval <SECONDS>` - Refresh interval in seconds (default: 2)

#### `dashboard`

- `-i, --interval <SECONDS>` - Refresh interval in seconds (default: 2)

#### `events`

- `-f, --filter <TYPE>` - Filter by event type (start, stop, die, etc.)

## Color Coding

The CLI uses colors to make information easier to understand:

- 🟢 **Green** - Healthy/Running status, success messages
- 🔴 **Red** - Errors, stopped containers, critical alerts
- 🟡 **Yellow** - Warnings, paused containers
- 🔵 **Blue** - Info messages, created containers
- 🟣 **Magenta** - Special status like health checks
- ⚫ **Gray/Dimmed** - Secondary information, IDs

## Examples

### Monitor a specific container continuously

```bash
docker-ui metrics nginx-web --follow --interval 1
```

### List containers in JSON format

```bash
docker-ui list --format json > containers.json
```

### Watch for container crashes

```bash
docker-ui events --filter die
```

### Quick container restart

```bash
docker-ui stop my-app && docker-ui start my-app
```

## Requirements

- Docker daemon must be running and accessible
- Rust 1.70 or later (for building from source)
- Terminal with color support (most modern terminals)

## Troubleshooting

### Docker daemon not available

If you see "Failed to connect to Docker daemon", ensure:

1. Docker is installed and running
2. You have permission to access Docker socket
3. Docker socket is at the default location

On Linux, you may need to add your user to the docker group:

```bash
sudo usermod -aG docker $USER
```

Then log out and log back in.

### Container not found

Make sure to use the correct container name or ID. You can list all containers with:

```bash
docker-ui list
```

### Colors not showing

Ensure your terminal supports ANSI colors. Most modern terminals do, but if you're using an older terminal or redirecting output, colors may not appear.

## Development

### Build

```bash
cargo build
```

### Run without installing

```bash
cargo run -- list
cargo run -- metrics <container-id>
```

### Run tests

```bash
cargo test
```

## Dependencies

- **docker-monitor** - Core Docker monitoring library
- **clap** - Command-line argument parsing
- **colored** - Terminal colors
- **tabled** - Beautiful table formatting
- **indicatif** - Progress bars and spinners
- **anyhow** - Error handling
- **tokio** - Async runtime

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
