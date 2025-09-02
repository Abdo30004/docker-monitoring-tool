# Docker UI - TUI Monitor Guide

## Overview

The TUI (Terminal User Interface) Monitor is an interactive, full-screen terminal application for monitoring Docker containers in real-time. Built with [Ratatui](https://ratatui.rs/), it provides a beautiful and intuitive interface for container management and monitoring.

## Features

### 🎨 Visual Interface

- **Full-screen terminal UI** with automatic terminal size detection
- **Color-coded status indicators** for easy identification
- **Visual gauges** for CPU and memory usage
- **Professional layout** with bordered sections and clear typography
- **Responsive design** that adapts to terminal size

### 📊 Three Main Tabs

#### 1. Overview Tab

- **Container Statistics**: Total, running, and stopped containers
- **Container Summary List**: Quick view of all containers with status
- **System-wide** monitoring at a glance

#### 2. Containers Tab

- **Detailed table** of all containers
- **Scrollable list** with keyboard navigation (↑/↓ or j/k)
- **Container information**: Status, name, image, ID, ports
- **Visual selection** with highlighted row

#### 3. Metrics Tab

- **Selected container metrics** in real-time
- **CPU Usage**: Percentage gauge with color coding (green/yellow/red)
- **Memory Usage**: Percentage gauge with actual values
- **Network I/O**: RX/TX bytes and packet counts
- **Disk I/O**: Read/write bytes and operation counts
- **Automatic updates** at specified interval

### ⌨️ Keyboard Controls

| Key          | Action                          |
| ------------ | ------------------------------- |
| `q` or `ESC` | Quit the application            |
| `TAB`        | Next tab                        |
| `Shift+TAB`  | Previous tab                    |
| `1`          | Jump to Overview tab            |
| `2`          | Jump to Containers tab          |
| `3`          | Jump to Metrics tab             |
| `↑` or `k`   | Navigate up in container list   |
| `↓` or `j`   | Navigate down in container list |
| `r`          | Manual refresh (force update)   |

### 🔄 Auto-Refresh

The TUI automatically refreshes data at the specified interval:

```bash
# Default: 1 second refresh
docker-ui monitor

# Custom interval: 2 seconds
docker-ui monitor --interval 2

# Fast refresh: 500ms (not recommended for many containers)
docker-ui monitor --interval 0.5
```

## Getting Started

### Launch the TUI

```bash
# From project root
cd docker-ui
cargo run --release -- monitor

# Or use the compiled binary
./target/release/docker-ui monitor
```

### First Time Use

1. **Start the TUI** - Run `docker-ui monitor`
2. **Navigate tabs** - Press `TAB` or number keys (1-3)
3. **Select containers** - Use arrow keys or j/k in Containers tab
4. **View metrics** - Switch to Metrics tab to see details for selected container
5. **Quit** - Press `q` or `ESC` when done

## Interface Layout

```
╔═══════════════════════════════════════════════════════════════╗
║      🐳 Docker Monitor TUI      2025-11-04 15:30:45           ║
╚═══════════════════════════════════════════════════════════════╝
╔═══════════════════════════════════════════════════════════════╗
║ Overview  | Containers | Metrics                              ║
╚═══════════════════════════════════════════════════════════════╝
╔═══════════════════════════════════════════════════════════════╗
║                                                                 ║
║                      [TAB CONTENT AREA]                         ║
║                                                                 ║
║                    (Overview/Containers/Metrics)                ║
║                                                                 ║
╚═══════════════════════════════════════════════════════════════╝
╔═══════════════════════════════════════════════════════════════╗
║ q/ESC: Quit | TAB: Switch | ↑↓/jk: Navigate | r: Refresh     ║
╚═══════════════════════════════════════════════════════════════╝
```

## Tab Details

### Overview Tab

Shows system-wide statistics and container summary:

```
╭─────────────────╮ ╭─────────────────╮ ╭─────────────────╮
│  📦 Containers  │ │    ✓ Active     │ │  ■ Inactive     │
│                 │ │                 │ │                 │
│     Total       │ │    Running      │ │    Stopped      │
│       5         │ │       3         │ │       2         │
╰─────────────────╯ ╰─────────────────╯ ╰─────────────────╯

╭─────────────────────────────────────────────────────────────╮
│ Container Summary                                           │
│ ● web-server  nginx  Running                                │
│ ● api-server  node:18  Running                              │
│ ■ database    postgres  Exited                              │
╰─────────────────────────────────────────────────────────────╯
```

### Containers Tab

Detailed container information in table format:

```
╭─────────────────────────────────────────────────────────────╮
│ Containers (↑/↓ to navigate)                                │
├────────────┬──────────────┬─────────────┬──────────┬───────┤
│ Status     │ Name         │ Image       │ ID       │ Ports │
├────────────┼──────────────┼─────────────┼──────────┼───────┤
│ ✓ Running  │ web-server   │ nginx       │ 39fdc5f… │ 80:80 │
│ ✓ Running  │ api-server   │ node:18     │ 7a3bc2e… │ 3000  │
│ ■ Exited   │ database     │ postgres    │ 1b5f8d9… │ 5432  │
╰────────────┴──────────────┴─────────────┴──────────┴───────╯
```

Use arrow keys to select a container. Selected row is highlighted.

### Metrics Tab

Detailed metrics for the currently selected container:

```
╭─────────────────────────────────────────────────────────────╮
│         Container: web-server  nginx                        │
╰─────────────────────────────────────────────────────────────╯
╭─────────────────────────────────────────────────────────────╮
│ 🔥 CPU Usage                                                │
│ ████████████████░░░░░░░░░░░░░░░░░░░░░░ 15.32%              │
╰─────────────────────────────────────────────────────────────╯
╭─────────────────────────────────────────────────────────────╮
│ 💾 Memory Usage                                             │
│ ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 12.50% (256MB/2GB) │
╰─────────────────────────────────────────────────────────────╯
╭─────────────────────────────────────────────────────────────╮
│ 🌐 Network I/O                                              │
│                                                              │
│ ⬇ RX: 1.25 GB  ⬆ TX: 512.00 MB                             │
│ Packets: ↓1000000 ↑500000                                   │
╰─────────────────────────────────────────────────────────────╯
╭─────────────────────────────────────────────────────────────╮
│ 💿 Disk I/O                                                 │
│                                                              │
│ 📖 Read: 128.00 MB  📝 Write: 64.00 MB                      │
│ Operations: R:1000 W:500                                    │
╰─────────────────────────────────────────────────────────────╯
```

## Color Coding

### Status Colors

- **Green (✓)**: Running containers
- **Red (■)**: Stopped/Exited containers
- **Yellow**: Paused/Restarting containers

### Resource Usage Colors

- **Green**: 0-50% usage (healthy)
- **Yellow**: 51-80% usage (warning)
- **Red**: 81-100% usage (critical)

### UI Element Colors

- **Cyan**: Headers and titles
- **Blue**: Image names and secondary info
- **Gray**: Hints and labels
- **White**: Primary text content

## Error Handling

### Docker Daemon Unavailable

If Docker daemon is not running, you'll see:

```
╭─────────────────────────────────────────────────────────────╮
│ Error                                                        │
│                                                              │
│               ⚠ Error                                        │
│                                                              │
│     Docker daemon unavailable: Connection refused           │
│                                                              │
│             Press 'r' to retry                               │
╰─────────────────────────────────────────────────────────────╯
```

Press `r` to retry the connection.

### No Containers Found

```
╭─────────────────────────────────────────────────────────────╮
│ Containers                                                   │
│                                                              │
│              No containers found                             │
│                                                              │
╰─────────────────────────────────────────────────────────────╯
```

### No Metrics Available

```
╭─────────────────────────────────────────────────────────────╮
│ Metrics                                                      │
│                                                              │
│    No metrics available - Container may not be running      │
│                                                              │
╰─────────────────────────────────────────────────────────────╯
```

## Tips & Best Practices

### 1. Optimal Refresh Interval

- **Default (1s)**: Good for most use cases
- **Fast (0.5s)**: For active debugging (may increase CPU usage)
- **Slow (5s)**: For monitoring many containers (reduces overhead)

### 2. Terminal Size

- **Minimum**: 80x24 characters
- **Recommended**: 120x30 or larger for best experience
- **Full screen**: Resize terminal to full screen for optimal layout

### 3. Keyboard Navigation

- Use `j`/`k` (Vim-style) for faster navigation
- Use number keys (1-3) for quick tab switching
- Press `r` if data seems stale

### 4. Performance

- TUI uses minimal resources
- Fetches only running container metrics
- Efficiently updates only changed data

### 5. Multiple Monitors

- Open multiple terminals for monitoring different aspects
- Use one TUI for overview, one for specific container metrics
- Combine with CLI commands for comprehensive monitoring

## Troubleshooting

### TUI Won't Start

**Issue**: Terminal doesn't switch to fullscreen
**Solution**:

- Ensure your terminal supports alternate screen
- Try a different terminal (Windows Terminal, iTerm2, etc.)
- Check that Docker daemon is running

### Garbled Output

**Issue**: Display looks corrupted
**Solution**:

- Resize terminal window
- Press `r` to refresh
- Restart the TUI
- Check terminal color support

### Slow Performance

**Issue**: TUI feels sluggish
**Solution**:

- Increase refresh interval: `--interval 3`
- Reduce number of running containers
- Check system resources
- Update to latest version

### Keys Not Working

**Issue**: Keyboard shortcuts don't respond
**Solution**:

- Ensure terminal has focus
- Check if terminal intercepts keys
- Try alternative keys (arrows vs j/k)
- Restart TUI

## Architecture

### Component Structure

```
tui/
├── mod.rs              # Entry point, event loop
├── app.rs              # Application state management
├── ui.rs               # UI rendering logic
└── terminal_manager.rs # Terminal setup/restore
```

### Data Flow

```
User Input → Event Loop → App State → UI Rendering → Terminal
     ↑                         ↓
     └──── Auto-refresh Timer ─┘
```

### Key Components

1. **Event Loop** (`mod.rs`): Handles keyboard input and refresh timing
2. **App State** (`app.rs`): Manages container data, metrics, and navigation
3. **UI Renderer** (`ui.rs`): Draws the interface using Ratatui widgets
4. **Terminal Manager** (`terminal_manager.rs`): Sets up/restores terminal state

## Development

### Building

```bash
cd docker-ui
cargo build --release
```

### Testing

```bash
# Run with Docker daemon
docker run -d --name test-nginx nginx
cargo run --release -- monitor

# Test with no containers
docker stop $(docker ps -aq)
cargo run --release -- monitor
```

### Debugging

Enable debug output:

```rust
// In tui/mod.rs, add logging
eprintln!("Debug: {}", message);
```

### Customization

#### Change Colors

Edit `tui/ui.rs`:

```rust
// Example: Change header color
Style::default().fg(Color::Magenta)  // Instead of Cyan
```

#### Add New Tab

1. Add tab to `app.rs`:

```rust
pub enum Tab {
    Overview,
    Containers,
    Metrics,
    NewTab,  // Add new tab
}
```

2. Implement rendering in `ui.rs`:

```rust
Tab::NewTab => draw_new_tab(f, app, chunks[2]),
```

3. Add keyboard shortcut in `mod.rs`:

```rust
KeyCode::Char('4') => app.select_tab(3),
```

## Comparison with Dashboard

### TUI Monitor

- ✅ Full-screen interface
- ✅ Interactive navigation
- ✅ Three organized tabs
- ✅ Visual gauges and charts
- ✅ Better for focused monitoring

### Dashboard Command

- ✅ Terminal-friendly output
- ✅ Scrollable history
- ✅ Copy/paste friendly
- ✅ Better for logging
- ✅ Script-friendly

**Use TUI** when you want an interactive, visual monitoring experience.

**Use Dashboard** when you want scrollable output or need to pipe data.

## Future Enhancements

Potential features for future versions:

- [ ] Log viewing tab
- [ ] Container action menu (start/stop/restart)
- [ ] Network graph visualization
- [ ] Historical data charts
- [ ] Search/filter containers
- [ ] Mouse support
- [ ] Customizable themes
- [ ] Split-screen view
- [ ] Export metrics to file

## Credits

Built with:

- [Ratatui](https://ratatui.rs/) - TUI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [docker-monitor](../docker-monitor) - Docker monitoring library

## License

MIT License - See LICENSE file for details
