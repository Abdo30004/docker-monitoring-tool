# Docker UI - Quick Start Guide

This guide will help you get started with the Docker UI CLI tool.

## Installation

### Build from Source

```bash
cd docker-ui
cargo build --release
```

The compiled binary will be at `target/release/docker-ui.exe` (Windows) or `target/release/docker-ui` (Linux/macOS).

### Quick Test (Development)

```bash
cargo run -- --help
```

## First Steps

### 1. Check System Status

Verify Docker is running and get system information:

```bash
docker-ui system
```

**Expected Output:**

```
╔═══════════════════════════════════════════════════════════════╗
║            🐳 Docker Container Monitor CLI 🐳              ║
╚═══════════════════════════════════════════════════════════════╝

━━━ Docker System Information ━━━

Docker Daemon:            Connected ✓
Running Containers:       5
Container Status:         3 running, 2 stopped
Unique Images:            4

ℹ Use 'docker-ui list' to see all containers
ℹ Use 'docker-ui dashboard' for real-time monitoring
```

### 2. List Your Containers

View all running containers:

```bash
docker-ui list
```

Or use the shorter alias:

```bash
docker-ui ls
```

### 3. Get Container Details

Get detailed information about a specific container:

```bash
docker-ui info <container-name>
```

Example:

```bash
docker-ui info web-server
```

### 4. Monitor Container Metrics

View real-time metrics for a container:

```bash
docker-ui metrics <container-name>
```

For continuous monitoring:

```bash
docker-ui metrics <container-name> --follow
```

### 5. Dashboard View

Monitor all containers at once:

```bash
docker-ui dashboard
```

Press `Ctrl+C` to exit.

## Common Use Cases

### Monitor a Production Container

Keep an eye on a production container with 1-second refresh:

```bash
docker-ui metrics production-api --follow --interval 1
```

### Check All Container Health

Quick overview of all containers:

```bash
docker-ui dashboard --interval 3
```

### Watch for Container Events

Monitor when containers start, stop, or crash:

```bash
docker-ui events
```

Filter specific events:

```bash
docker-ui events --filter die
```

### Export Container List

Get container list in JSON format:

```bash
docker-ui list --format json > containers.json
```

### Quick Container Management

Start a container:

```bash
docker-ui start my-container
```

Stop a container:

```bash
docker-ui stop my-container
```

## Output Formats

### Table Format (Default)

Pretty table with all information:

```bash
docker-ui list --format table
```

### Compact Format

Condensed single-line per container:

```bash
docker-ui list --format compact
```

### JSON Format

Machine-readable output:

```bash
docker-ui list --format json
```

## Tips & Tricks

### 1. Use Aliases

Create shell aliases for frequently used commands:

**Bash/Zsh (.bashrc or .zshrc):**

```bash
alias dls='docker-ui list'
alias dstats='docker-ui metrics'
alias ddash='docker-ui dashboard'
alias devents='docker-ui events'
```

**PowerShell ($PROFILE):**

```powershell
function dls { docker-ui list $args }
function dstats { docker-ui metrics $args }
function ddash { docker-ui dashboard $args }
```

### 2. Quick Container ID

Use partial container IDs or names:

```bash
docker-ui info web    # matches 'web-server'
docker-ui metrics a1b2 # matches container ID starting with a1b2
```

### 3. Monitoring in Background

On Linux/macOS, you can run monitoring in the background:

```bash
docker-ui events > events.log 2>&1 &
```

### 4. Watch Specific Metrics

Combine with other tools for specific monitoring:

```bash
docker-ui metrics api --follow | grep "CPU Usage"
```

### 5. Quick Status Check

Create a script for quick health checks:

```bash
#!/bin/bash
echo "=== Docker Status ==="
docker-ui system
echo ""
echo "=== Critical Containers ==="
docker-ui list --format compact
```

## Keyboard Shortcuts

When running interactive commands (dashboard, metrics --follow, events):

- `Ctrl+C` - Stop monitoring and exit
- Screen automatically refreshes at specified interval

## Color Scheme

The CLI uses colors to convey information:

- **Green** - Healthy status, running containers, success
- **Red** - Stopped containers, errors, critical alerts
- **Yellow** - Warnings, paused containers
- **Blue** - Informational messages
- **Cyan** - Container IDs, images
- **Magenta** - Health status
- **Gray/Dimmed** - Secondary information

## Troubleshooting

### "Failed to connect to Docker daemon"

**Solution:**

1. Verify Docker is running: `docker ps`
2. Check Docker service:
   - Windows: Docker Desktop should be running
   - Linux: `sudo systemctl status docker`
3. Verify permissions (Linux): `sudo usermod -aG docker $USER` then logout/login

### "Container not found"

**Solution:**

1. List all containers: `docker-ui list`
2. Use the exact container name or ID from the list
3. Make sure the container exists and is running

### Colors not showing

**Solution:**

- Ensure your terminal supports ANSI colors
- Try a different terminal (Windows Terminal, iTerm2, etc.)
- Check if output is redirected (colors are disabled when piping)

### Slow performance

**Solution:**

1. Increase refresh interval: `--interval 5`
2. Monitor fewer containers
3. Check Docker daemon performance

## Next Steps

1. **Explore Commands** - Try each command with `--help`:

   ```bash
   docker-ui metrics --help
   ```

2. **Read Full Documentation** - See [README.md](README.md) for complete reference

3. **Set Up Monitoring** - Create scripts for automated monitoring

4. **Customize Output** - Experiment with different formats and intervals

## Example Workflows

### Development Workflow

```bash
# 1. Check what's running
docker-ui list

# 2. Start your dev container
docker-ui start dev-api

# 3. Monitor it while developing
docker-ui metrics dev-api --follow --interval 2

# 4. Watch for issues
docker-ui events --filter die
```

### Production Monitoring

```bash
# 1. Dashboard overview
docker-ui dashboard --interval 5

# 2. Deep dive into specific container
docker-ui metrics production-api --follow --interval 1

# 3. Check detailed info
docker-ui info production-api

# 4. Export status
docker-ui list --format json > status-$(date +%Y%m%d).json
```

### Debugging Workflow

```bash
# 1. Check system status
docker-ui system

# 2. Find the problematic container
docker-ui list --format compact

# 3. Get detailed info
docker-ui info problematic-container

# 4. Check metrics
docker-ui metrics problematic-container

# 5. Monitor events
docker-ui events --filter die
```

## Support

For issues, questions, or feature requests, please refer to the project repository.

Happy monitoring! 🐳
