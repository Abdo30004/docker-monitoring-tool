mod commands;
mod display;
mod tui;

use clap::{ Parser, Subcommand };
use anyhow::Result;

#[derive(Parser)]
#[command(name = "docker-ui")]
#[command(author = "Docker UI Team")]
#[command(version = "0.1.0")]
#[command(about = "A beautiful CLI interface for Docker container monitoring", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all running containers
    #[command(alias = "ls")]
    List {
        /// Show all containers (including stopped)
        #[arg(short, long)]
        all: bool,

        /// Output format: table, json, compact
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Show detailed information about a container
    #[command(alias = "inspect")]
    Info {
        /// Container ID or name
        container: String,
    },

    /// Display real-time metrics for a container
    #[command(alias = "stats")]
    Metrics {
        /// Container ID or name
        container: String,

        /// Continuous monitoring (refresh every N seconds)
        #[arg(short, long)]
        follow: bool,

        /// Refresh interval in seconds (only with --follow)
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Monitor metrics for all running containers
    Dashboard {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Start a stopped container
    Start {
        /// Container ID or name
        container: String,
    },

    /// Stop a running container
    Stop {
        /// Container ID or name
        container: String,
    },

    /// Monitor Docker events in real-time
    Events {
        /// Filter by event type (start, stop, die, etc.)
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Show system-wide Docker information
    System,

    /// Launch interactive TUI monitor
    Monitor {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "1")]
        interval: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print header
    display::print_header();

    match cli.command {
        Commands::List { all, format } => {
            commands::list_containers(all, &format).await?;
        }
        Commands::Info { container } => {
            commands::container_info(&container).await?;
        }
        Commands::Metrics { container, follow, interval } => {
            commands::container_metrics(&container, follow, interval).await?;
        }
        Commands::Dashboard { interval } => {
            commands::dashboard(interval).await?;
        }
        Commands::Start { container } => {
            commands::start_container(&container).await?;
        }
        Commands::Stop { container } => {
            commands::stop_container(&container).await?;
        }
        Commands::Events { filter } => {
            commands::monitor_events(filter).await?;
        }
        Commands::System => {
            commands::system_info().await?;
        }
        Commands::Monitor { interval } => {
            // Don't print header for TUI mode
            tui::run_tui(interval).await?;
            return Ok(());
        }
    }

    Ok(())
}
