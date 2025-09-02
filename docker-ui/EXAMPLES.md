# Docker UI Examples

This document provides comprehensive examples of using the Docker UI CLI tool.

## Basic Examples

### Example 1: List All Containers

```bash
docker-ui list
```

**Output:**

```
╔═══════════════════════════════════════════════════════════════╗
║            🐳 Docker Container Monitor CLI 🐳              ║
╚═══════════════════════════════════════════════════════════════╝

╭────────────┬──────────────┬───────────────┬─────────┬─────────╮
│ ID         │ Name         │ Image         │ Status  │ State   │
├────────────┼──────────────┼───────────────┼─────────┼─────────┤
│ a1b2c3d4e5 │ web-server   │ nginx:latest  │ Running │ running │
│ f6g7h8i9j0 │ db-server    │ postgres:14   │ Running │ running │
│ k1l2m3n4o5 │ cache-server │ redis:alpine  │ Running │ running │
╰────────────┴──────────────┴───────────────┴─────────┴─────────╯

ℹ Total: 3 container(s)
```

### Example 2: Container Information

```bash
docker-ui info web-server
```

**Output:**

```
━━━ Container Information ━━━

ID:                  a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
Name:                web-server
Image:               nginx:latest
Status:              Running
State:               running
Created:             2025-11-04 10:30:00

━━━ Port Mappings ━━━

  80 tcp → 8080 tcp
  443 tcp → 8443 tcp
```

### Example 3: Real-time Metrics

```bash
docker-ui metrics web-server
```

**Output:**

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

## Advanced Examples

### Example 4: Continuous Monitoring

Monitor a container with automatic refresh every 1 second:

```bash
docker-ui metrics web-server --follow --interval 1
```

**Result:** Screen updates every second with latest metrics (press Ctrl+C to stop)

### Example 5: Dashboard View

Monitor all containers simultaneously:

```bash
docker-ui dashboard --interval 3
```

**Output:**

```
━━━ Container Dashboard ━━━

╭──────────────┬────────┬────────────┬──────────┬───────────────────────┬──────────────────────╮
│ Container    │ CPU %  │ Memory     │ Memory % │ Net I/O               │ Disk I/O             │
├──────────────┼────────┼────────────┼──────────┼───────────────────────┼──────────────────────┤
│ web-server   │ 15.32% │ 256.00 MB  │ 12.50%   │ ⬇ 1.25 GB / ⬆ 512 MB  │ 📖 128 MB / 📝 64 MB  │
│ db-server    │ 45.67% │ 1.50 GB    │ 75.00%   │ ⬇ 2.10 GB / ⬆ 1.20 GB │ 📖 512 MB / 📝 256 MB │
│ cache-server │ 8.43%  │ 64.00 MB   │ 3.13%    │ ⬇ 512 MB / ⬆ 256 MB   │ 📖 16 MB / 📝 8 MB    │
╰──────────────┴────────┴────────────┴──────────┴───────────────────────┴──────────────────────╯

ℹ Last updated: 2025-11-04 15:35:20
```

### Example 6: Event Monitoring

Watch all Docker events in real-time:

```bash
docker-ui events
```

**Output:**

```
ℹ Monitoring Docker events (press Ctrl+C to stop)...

━━━ Event Stream ━━━

[15:30:12] Container Start - web-server (a1b2c3d4e5)
[15:30:45] Container Stop - old-api (f6g7h8i9j0)
[15:31:20] Container Die - crashed-app (k1l2m3n4o5)
[15:32:05] Container Create - new-service (p6q7r8s9t0)
```

### Example 7: Filtered Events

Monitor only container crashes:

```bash
docker-ui events --filter die
```

**Output:**

```
ℹ Monitoring Docker events (press Ctrl+C to stop)...

━━━ Event Stream ━━━

[15:31:20] Container Die - crashed-app (k1l2m3n4o5)
[15:45:33] Container Die - unstable-service (u1v2w3x4y5)
```

## Output Format Examples

### Example 8: JSON Output

```bash
docker-ui list --format json
```

**Output:**

```json
[
  {
    "id": "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
    "name": "web-server",
    "image": "nginx:latest",
    "status": "Running",
    "state": "running",
    "created": "2025-11-04T10:30:00Z",
    "labels": {
      "env": "production",
      "app": "web"
    },
    "ports": [
      {
        "private_port": 80,
        "public_port": 8080,
        "port_type": "tcp"
      }
    ]
  }
]
```

### Example 9: Compact Format

```bash
docker-ui list --format compact
```

**Output:**

```
a1b2c3d4e5 web-server nginx:latest Running
f6g7h8i9j0 db-server postgres:14 Running
k1l2m3n4o5 cache-server redis:alpine Running

ℹ Total: 3 container(s)
```

## Container Management Examples

### Example 10: Start Container

```bash
docker-ui start my-stopped-container
```

**Output:**

```
╔═══════════════════════════════════════════════════════════════╗
║            🐳 Docker Container Monitor CLI 🐳              ║
╚═══════════════════════════════════════════════════════════════╝

✓ Container 'my-stopped-container' started successfully
```

### Example 11: Stop Container

```bash
docker-ui stop my-running-container
```

**Output:**

```
╔═══════════════════════════════════════════════════════════════╗
║            🐳 Docker Container Monitor CLI 🐳              ║
╚═══════════════════════════════════════════════════════════════╝

✓ Container 'my-running-container' stopped successfully
```

## Real-World Scenarios

### Scenario 1: Debugging High CPU Usage

```bash
# 1. Check which container is using CPU
docker-ui dashboard

# 2. Deep dive into the problematic container
docker-ui metrics high-cpu-container --follow --interval 1

# 3. Get detailed information
docker-ui info high-cpu-container

# 4. Check logs (using Docker directly)
docker logs high-cpu-container
```

### Scenario 2: Monitoring Production Deployment

```bash
# 1. Check system status before deployment
docker-ui system

# 2. Monitor events during deployment
docker-ui events --filter start

# 3. After deployment, check metrics
docker-ui metrics production-api --follow --interval 2

# 4. Verify all services are running
docker-ui list --format table
```

### Scenario 3: Memory Leak Detection

```bash
# Monitor memory usage over time
docker-ui metrics suspected-container --follow --interval 5

# Watch the "Memory Usage" and "Max Usage" values increase
# If memory keeps growing without bounds, you likely have a leak
```

### Scenario 4: Network Traffic Analysis

```bash
# Monitor network I/O for a web server
docker-ui metrics web-api --follow --interval 1

# Observe the "Network I/O" section
# High RX (receive) = lots of incoming requests
# High TX (transmit) = lots of outgoing responses
```

### Scenario 5: Container Health Check

Create a script for automated health checks:

```bash
#!/bin/bash
# health-check.sh

echo "=== Docker System Status ==="
docker-ui system

echo ""
echo "=== Critical Services ==="
for container in api-server db-server cache-server; do
    echo "Checking $container..."
    docker-ui metrics $container 2>&1 | grep "CPU Usage" || echo "  ⚠ Failed to get metrics"
done

echo ""
echo "=== Recent Events ==="
docker-ui events --filter die > /dev/null 2>&1 &
EVENT_PID=$!
sleep 10
kill $EVENT_PID 2>/dev/null
```

## Scripting Examples

### Example 12: Export All Container Info

```bash
# Export to JSON
docker-ui list --format json > containers-$(date +%Y%m%d-%H%M%S).json

# Pretty print with jq
docker-ui list --format json | jq '.'
```

### Example 13: Monitor and Alert

```bash
#!/bin/bash
# monitor-and-alert.sh

CONTAINER="production-api"
CPU_THRESHOLD=80

while true; do
    # Get metrics and check CPU
    OUTPUT=$(docker-ui metrics $CONTAINER 2>&1)
    CPU=$(echo "$OUTPUT" | grep "Usage:" | head -1 | awk '{print $2}' | tr -d '%')

    if (( $(echo "$CPU > $CPU_THRESHOLD" | bc -l) )); then
        echo "ALERT: $CONTAINER CPU usage is ${CPU}%"
        # Send alert (email, Slack, etc.)
    fi

    sleep 60
done
```

### Example 14: Container Restart Script

```bash
#!/bin/bash
# restart-if-unhealthy.sh

CONTAINER="my-service"

# Check if container is running
if docker-ui info $CONTAINER &>/dev/null; then
    echo "Container is running, checking health..."

    # Get metrics
    METRICS=$(docker-ui metrics $CONTAINER 2>&1)

    # Restart if needed (your logic here)
    # docker-ui stop $CONTAINER && docker-ui start $CONTAINER
else
    echo "Container not found or stopped, starting..."
    docker-ui start $CONTAINER
fi
```

## Tips for Effective Use

### Tip 1: Create Shortcuts

Add to your `.bashrc` or `.zshrc`:

```bash
# Docker UI shortcuts
alias dls='docker-ui list'
alias dinfo='docker-ui info'
alias dstats='docker-ui metrics'
alias ddash='docker-ui dashboard'
alias devents='docker-ui events'
alias dsys='docker-ui system'
```

### Tip 2: Combine with Watch

```bash
# Update every 2 seconds
watch -n 2 'docker-ui list --format compact'
```

### Tip 3: Pipe to Less for Scrolling

```bash
docker-ui list --format json | less
```

### Tip 4: Filter with Grep

```bash
# Find containers using nginx image
docker-ui list --format compact | grep nginx

# Find high CPU usage
docker-ui dashboard | grep -E "[5-9][0-9]\.[0-9]+%|100\.00%"
```

## Conclusion

The Docker UI CLI provides a powerful and beautiful way to monitor and manage your Docker containers. Experiment with different commands and options to find the workflow that works best for you!

For more information, see:

- [README.md](README.md) - Full documentation
- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- Run `docker-ui --help` for command reference
