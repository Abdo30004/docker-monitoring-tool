use colored::*;
use tabled::{ Table, Tabled, settings::Style };
use docker_monitor::{ ContainerInfo, ContainerMetrics, ContainerStatus };

/// Print the application header
pub fn print_header() {
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_blue()
    );
    println!(
        "{}",
        "║            🐳 Docker Container Monitor CLI 🐳              ║".bright_blue().bold()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_blue()
    );
    println!();
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

/// Print a section header
pub fn print_section(title: &str) {
    println!();
    println!("{}", format!("━━━ {} ━━━", title).bright_cyan().bold());
    println!();
}

/// Format container status with color
pub fn format_status(status: &ContainerStatus) -> ColoredString {
    match status {
        ContainerStatus::Running => "Running".green().bold(),
        ContainerStatus::Stopped => "Stopped".red(),
        ContainerStatus::Paused => "Paused".yellow(),
        ContainerStatus::Restarting => "Restarting".yellow().bold(),
        ContainerStatus::Dead => "Dead".red().bold(),
        ContainerStatus::Created => "Created".cyan(),
        ContainerStatus::Exited => "Exited".red(),
        ContainerStatus::Unknown => "Unknown".white().dimmed(),
    }
}

/// Format bytes to human-readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Format percentage with color coding
pub fn format_percentage(
    value: f64,
    threshold_warning: f64,
    threshold_critical: f64
) -> ColoredString {
    let formatted = format!("{:.2}%", value);

    if value >= threshold_critical {
        formatted.red().bold()
    } else if value >= threshold_warning {
        formatted.yellow()
    } else {
        formatted.green()
    }
}

/// Container data for table display
#[derive(Tabled)]
pub struct ContainerRow {
    #[tabled(rename = "ID")]
    pub id: String,

    #[tabled(rename = "Name")]
    pub name: String,

    #[tabled(rename = "Image")]
    pub image: String,

    #[tabled(rename = "Status")]
    pub status: String,

    #[tabled(rename = "State")]
    pub state: String,
}

impl From<&ContainerInfo> for ContainerRow {
    fn from(container: &ContainerInfo) -> Self {
        Self {
            id: container.id[..12].to_string(),
            name: container.name.clone(),
            image: container.image.clone(),
            status: format!("{}", container.status),
            state: container.state.clone(),
        }
    }
}

/// Display containers in a table
pub fn display_containers_table(containers: &[ContainerInfo]) {
    if containers.is_empty() {
        print_warning("No containers found");
        return;
    }

    let rows: Vec<ContainerRow> = containers.iter().map(ContainerRow::from).collect();
    let mut table = Table::new(rows);
    table.with(Style::rounded());

    println!("{}", table);
    println!();
    print_info(&format!("Total: {} container(s)", containers.len()));
}

/// Display containers in compact format
pub fn display_containers_compact(containers: &[ContainerInfo]) {
    if containers.is_empty() {
        print_warning("No containers found");
        return;
    }

    for container in containers {
        let status_colored = format_status(&container.status);
        println!(
            "{} {} {} {}",
            container.id[..12].bright_black(),
            container.name.bright_white().bold(),
            container.image.cyan(),
            status_colored
        );
    }

    println!();
    print_info(&format!("Total: {} container(s)", containers.len()));
}

/// Display detailed container information
pub fn display_container_details(container: &ContainerInfo) {
    print_section("Container Information");

    println!("{:<20} {}", "ID:".bright_white().bold(), container.id);
    println!("{:<20} {}", "Name:".bright_white().bold(), container.name.bright_green());
    println!("{:<20} {}", "Image:".bright_white().bold(), container.image.cyan());
    println!("{:<20} {}", "Status:".bright_white().bold(), format_status(&container.status));
    println!("{:<20} {}", "State:".bright_white().bold(), container.state);
    println!(
        "{:<20} {}",
        "Created:".bright_white().bold(),
        container.created.format("%Y-%m-%d %H:%M:%S")
    );

    if !container.labels.is_empty() {
        print_section("Labels");
        for (key, value) in &container.labels {
            println!("  {}: {}", key.yellow(), value);
        }
    }

    if !container.ports.is_empty() {
        print_section("Port Mappings");
        for port in &container.ports {
            if let Some(public) = port.public_port {
                println!(
                    "  {} {} → {} {}",
                    port.private_port.to_string().cyan(),
                    port.port_type.dimmed(),
                    public.to_string().green().bold(),
                    port.port_type.dimmed()
                );
            } else {
                println!("  {} {}", port.private_port.to_string().cyan(), port.port_type.dimmed());
            }
        }
    }
}

/// Display container metrics
pub fn display_metrics(metrics: &ContainerMetrics) {
    print_section(&format!("Metrics for {}", metrics.container_name.bright_green()));

    println!(
        "{:<20} {}",
        "Timestamp:".bright_white().bold(),
        metrics.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().dimmed()
    );

    // CPU Stats
    println!();
    println!("{}", "CPU Usage:".bright_yellow().bold());
    println!(
        "  {:<18} {}",
        "Usage:",
        format_percentage(metrics.cpu_stats.usage_percent, 70.0, 90.0)
    );
    println!(
        "  {:<18} {}",
        "Total Usage:",
        format!("{} ns", metrics.cpu_stats.total_usage).dimmed()
    );
    println!("  {:<18} {}", "Online CPUs:", metrics.cpu_stats.online_cpus.to_string().cyan());

    // Memory Stats
    println!();
    println!("{}", "Memory Usage:".bright_magenta().bold());
    println!(
        "  {:<18} {} ({})",
        "Usage:",
        format_bytes(metrics.memory_stats.usage).bright_white(),
        format_percentage(metrics.memory_stats.usage_percent, 70.0, 90.0)
    );
    println!("  {:<18} {}", "Limit:", format_bytes(metrics.memory_stats.limit).dimmed());
    println!("  {:<18} {}", "Max Usage:", format_bytes(metrics.memory_stats.max_usage).dimmed());

    // Network Stats
    println!();
    println!("{}", "Network I/O:".bright_cyan().bold());
    println!(
        "  {:<18} {} ⬇  {} ⬆",
        "Bytes:",
        format_bytes(metrics.network_stats.rx_bytes).green(),
        format_bytes(metrics.network_stats.tx_bytes).blue()
    );
    println!(
        "  {:<18} {} ⬇  {} ⬆",
        "Packets:",
        metrics.network_stats.rx_packets.to_string().dimmed(),
        metrics.network_stats.tx_packets.to_string().dimmed()
    );
    if metrics.network_stats.rx_errors > 0 || metrics.network_stats.tx_errors > 0 {
        println!(
            "  {:<18} {} ⬇  {} ⬆",
            "Errors:",
            metrics.network_stats.rx_errors.to_string().red(),
            metrics.network_stats.tx_errors.to_string().red()
        );
    }

    // Disk Stats
    println!();
    println!("{}", "Disk I/O:".bright_green().bold());
    println!(
        "  {:<18} {} 📖  {} 📝",
        "Bytes:",
        format_bytes(metrics.disk_stats.read_bytes).cyan(),
        format_bytes(metrics.disk_stats.write_bytes).yellow()
    );
    println!(
        "  {:<18} {} 📖  {} 📝",
        "Operations:",
        metrics.disk_stats.read_ops.to_string().dimmed(),
        metrics.disk_stats.write_ops.to_string().dimmed()
    );
}

/// Metrics data for table display
#[derive(Tabled)]
pub struct MetricsRow {
    #[tabled(rename = "Container")]
    pub name: String,

    #[tabled(rename = "CPU %")]
    pub cpu: String,

    #[tabled(rename = "Memory")]
    pub memory: String,

    #[tabled(rename = "Memory %")]
    pub memory_pct: String,

    #[tabled(rename = "Net I/O")]
    pub network: String,

    #[tabled(rename = "Disk I/O")]
    pub disk: String,
}

impl From<&ContainerMetrics> for MetricsRow {
    fn from(metrics: &ContainerMetrics) -> Self {
        Self {
            name: metrics.container_name.clone(),
            cpu: format!("{:.2}%", metrics.cpu_stats.usage_percent),
            memory: format_bytes(metrics.memory_stats.usage),
            memory_pct: format!("{:.2}%", metrics.memory_stats.usage_percent),
            network: format!(
                "⬇ {} / ⬆ {}",
                format_bytes(metrics.network_stats.rx_bytes),
                format_bytes(metrics.network_stats.tx_bytes)
            ),
            disk: format!(
                "📖 {} / 📝 {}",
                format_bytes(metrics.disk_stats.read_bytes),
                format_bytes(metrics.disk_stats.write_bytes)
            ),
        }
    }
}

/// Display multiple container metrics in a table
pub fn display_metrics_table(metrics_list: &[ContainerMetrics]) {
    if metrics_list.is_empty() {
        print_warning("No metrics available");
        return;
    }

    let rows: Vec<MetricsRow> = metrics_list.iter().map(MetricsRow::from).collect();
    let mut table = Table::new(rows);
    table.with(Style::rounded());

    println!("{}", table);
}

/// Display a progress bar for operations
pub fn create_spinner(message: &str) -> indicatif::ProgressBar {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner
}
