use super::app::{ App, Tab };
use ratatui::{
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Line, Span },
    widgets::{ Block, Borders, Cell, Gauge, List, ListItem, Paragraph, Row, Table, Tabs, Wrap },
    Frame,
};
use chrono::Local;
use docker_monitor::ContainerStatus;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(0), // Content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    draw_header(f, chunks[0]);
    draw_tabs(f, app, chunks[1]);

    match app.current_tab {
        Tab::Overview => draw_overview(f, app, chunks[2]),
        Tab::Containers => draw_containers(f, app, chunks[2]),
        Tab::Metrics => draw_metrics(f, app, chunks[2]),
    }

    draw_footer(f, app, chunks[3]);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let header = Paragraph::new(
        vec![
            Line::from(
                vec![
                    Span::styled("🐳 ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        "Docker Monitor TUI",
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    ),
                    Span::raw("  "),
                    Span::styled(time, Style::default().fg(Color::Gray))
                ]
            )
        ]
    )
        .block(
            Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
        )
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tabs = Tab::all();
    let titles: Vec<Line> = tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.title().split_at(1);
            Line::from(
                vec![
                    Span::styled(
                        first,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)
                    ),
                    Span::styled(rest, Style::default().fg(Color::White))
                ]
            )
        })
        .collect();

    let selected_index = tabs
        .iter()
        .position(|&t| t == app.current_tab)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Navigation"))
        .select(selected_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn draw_overview(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Stats boxes
            Constraint::Min(0), // Container list
        ])
        .split(area);

    // Stats boxes
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[0]);

    // Total containers
    let total_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled("Total", Style::default().fg(Color::Gray))]),
        Line::from(
            vec![
                Span::styled(
                    app.total_containers.to_string(),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                )
            ]
        )
    ];
    let total_block = Paragraph::new(total_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("📦 Containers")
                .border_style(Style::default().fg(Color::Cyan))
        )
        .alignment(Alignment::Center);
    f.render_widget(total_block, stats_chunks[0]);

    // Running containers
    let running_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled("Running", Style::default().fg(Color::Gray))]),
        Line::from(
            vec![
                Span::styled(
                    app.running_containers.to_string(),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                )
            ]
        )
    ];
    let running_block = Paragraph::new(running_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("✓ Active")
                .border_style(Style::default().fg(Color::Green))
        )
        .alignment(Alignment::Center);
    f.render_widget(running_block, stats_chunks[1]);

    // Stopped containers
    let stopped = app.total_containers - app.running_containers;
    let stopped_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled("Stopped", Style::default().fg(Color::Gray))]),
        Line::from(
            vec![
                Span::styled(
                    stopped.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                )
            ]
        )
    ];
    let stopped_block = Paragraph::new(stopped_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("■ Inactive")
                .border_style(Style::default().fg(Color::Red))
        )
        .alignment(Alignment::Center);
    f.render_widget(stopped_block, stats_chunks[2]);

    // Container summary list
    if app.error_message.is_some() {
        draw_error(f, app, chunks[1]);
    } else {
        let items: Vec<ListItem> = app.containers
            .iter()
            .map(|container| {
                let status_color = if matches!(container.status, ContainerStatus::Running) {
                    Color::Green
                } else {
                    Color::Red
                };

                let content = vec![
                    Line::from(
                        vec![
                            Span::styled("● ", Style::default().fg(status_color)),
                            Span::styled(
                                &container.name,
                                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                            ),
                            Span::raw("  "),
                            Span::styled(&container.image, Style::default().fg(Color::Blue)),
                            Span::raw("  "),
                            Span::styled(
                                container.status.to_string(),
                                Style::default().fg(status_color)
                            )
                        ]
                    )
                ];

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Container Summary"))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, chunks[1]);
    }
}

fn draw_containers(f: &mut Frame, app: &App, area: Rect) {
    if app.error_message.is_some() {
        draw_error(f, app, area);
        return;
    }

    if app.containers.is_empty() {
        let empty = Paragraph::new("No containers found")
            .block(Block::default().borders(Borders::ALL).title("Containers"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, area);
        return;
    }

    let header = Row::new(vec!["Status", "Name", "Image", "ID", "Ports"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .height(1);

    let rows: Vec<Row> = app.containers
        .iter()
        .enumerate()
        .map(|(i, container)| {
            let status_symbol = if matches!(container.status, ContainerStatus::Running) {
                "✓"
            } else {
                "■"
            };

            let status_color = if matches!(container.status, ContainerStatus::Running) {
                Color::Green
            } else {
                Color::Red
            };

            let ports_str = container.ports
                .iter()
                .map(|p| {
                    if let Some(public) = p.public_port {
                        format!("{}:{}", public, p.private_port)
                    } else {
                        p.private_port.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            let row_style = if i == app.selected_index {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(
                vec![
                    Cell::from(
                        Span::styled(
                            format!("{} {}", status_symbol, container.status),
                            Style::default().fg(status_color)
                        )
                    ),
                    Cell::from(container.name.as_str()),
                    Cell::from(
                        Span::styled(container.image.as_str(), Style::default().fg(Color::Blue))
                    ),
                    Cell::from(&container.id[..12]),
                    Cell::from(ports_str)
                ]
            ).style(row_style)
        })
        .collect();

    let table = Table::new(rows, [
        Constraint::Length(15),
        Constraint::Length(20),
        Constraint::Length(25),
        Constraint::Length(14),
        Constraint::Min(15),
    ])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Containers (↑/↓ to navigate)"))
        .column_spacing(1);

    f.render_widget(table, area);
}

fn draw_metrics(f: &mut Frame, app: &App, area: Rect) {
    if app.error_message.is_some() {
        draw_error(f, app, area);
        return;
    }

    let selected = app.get_selected_container();
    let metrics = app.get_selected_metrics();

    if selected.is_none() || metrics.is_none() {
        let msg = if app.containers.is_empty() {
            "No containers found"
        } else {
            "No metrics available - Container may not be running"
        };

        let empty = Paragraph::new(msg)
            .block(Block::default().borders(Borders::ALL).title("Metrics"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, area);
        return;
    }

    let container = selected.unwrap();
    let metrics = metrics.unwrap();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Container name
            Constraint::Length(4), // CPU
            Constraint::Length(4), // Memory
            Constraint::Length(6), // Network
            Constraint::Min(0), // Disk
        ])
        .split(area);

    // Container name
    let title = Paragraph::new(
        vec![
            Line::from(
                vec![
                    Span::styled("Container: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        &container.name,
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    ),
                    Span::raw("  "),
                    Span::styled(&container.image, Style::default().fg(Color::Blue))
                ]
            )
        ]
    )
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // CPU
    let cpu_percent = metrics.cpu_stats.usage_percent.clamp(0.0, 100.0) as u16;
    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("🔥 CPU Usage"))
        .gauge_style(
            Style::default().fg(
                if cpu_percent > 80 {
                    Color::Red
                } else if cpu_percent > 50 {
                    Color::Yellow
                } else {
                    Color::Green
                }
            )
        )
        .percent(cpu_percent)
        .label(format!("{:.2}%", metrics.cpu_stats.usage_percent));
    f.render_widget(cpu_gauge, chunks[1]);

    // Memory
    let mem_percent = metrics.memory_stats.usage_percent.clamp(0.0, 100.0) as u16;
    let mem_label = format!(
        "{:.2}% ({} / {})",
        metrics.memory_stats.usage_percent,
        format_bytes(metrics.memory_stats.usage),
        format_bytes(metrics.memory_stats.limit)
    );
    let mem_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("💾 Memory Usage"))
        .gauge_style(
            Style::default().fg(
                if mem_percent > 80 {
                    Color::Red
                } else if mem_percent > 50 {
                    Color::Yellow
                } else {
                    Color::Green
                }
            )
        )
        .percent(mem_percent)
        .label(mem_label);
    f.render_widget(mem_gauge, chunks[2]);

    // Network
    let network_text = vec![
        Line::from(""),
        Line::from(
            vec![
                Span::styled("⬇ RX: ", Style::default().fg(Color::Green)),
                Span::raw(format_bytes(metrics.network_stats.rx_bytes)),
                Span::raw("  "),
                Span::styled("⬆ TX: ", Style::default().fg(Color::Blue)),
                Span::raw(format_bytes(metrics.network_stats.tx_bytes))
            ]
        ),
        Line::from(
            vec![
                Span::styled("Packets: ", Style::default().fg(Color::Gray)),
                Span::raw(
                    format!(
                        "↓{} ↑{}",
                        metrics.network_stats.rx_packets,
                        metrics.network_stats.tx_packets
                    )
                )
            ]
        )
    ];
    let network_block = Paragraph::new(network_text)
        .block(Block::default().borders(Borders::ALL).title("🌐 Network I/O"))
        .alignment(Alignment::Center);
    f.render_widget(network_block, chunks[3]);

    // Disk
    let disk_text = vec![
        Line::from(""),
        Line::from(
            vec![
                Span::styled("📖 Read: ", Style::default().fg(Color::Cyan)),
                Span::raw(format_bytes(metrics.disk_stats.read_bytes)),
                Span::raw("  "),
                Span::styled("📝 Write: ", Style::default().fg(Color::Magenta)),
                Span::raw(format_bytes(metrics.disk_stats.write_bytes))
            ]
        ),
        Line::from(
            vec![
                Span::styled("Operations: ", Style::default().fg(Color::Gray)),
                Span::raw(
                    format!("R:{} W:{}", metrics.disk_stats.read_ops, metrics.disk_stats.write_ops)
                )
            ]
        )
    ];
    let disk_block = Paragraph::new(disk_text)
        .block(Block::default().borders(Borders::ALL).title("💿 Disk I/O"))
        .alignment(Alignment::Center);
    f.render_widget(disk_block, chunks[4]);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let help_text = vec![
        Line::from(
            vec![
                Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("/"),
                Span::styled(
                    "ESC",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
                Span::raw(": Quit  "),
                Span::styled(
                    "TAB",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
                Span::raw(": Switch Tab  "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("/"),
                Span::styled("jk", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(": Navigate  "),
                Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(": Refresh  "),
                Span::styled(
                    "1-3",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
                Span::raw(": Quick Tab  "),
                Span::styled(
                    format!("⟳ {}s", app.refresh_interval),
                    Style::default().fg(Color::Green)
                )
            ]
        )
    ];

    let footer = Paragraph::new(help_text)
        .block(
            Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
        )
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}

fn draw_error(f: &mut Frame, app: &App, area: Rect) {
    if let Some(error) = &app.error_message {
        let error_text = vec![
            Line::from(""),
            Line::from(
                vec![
                    Span::styled(
                        "⚠ Error",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    )
                ]
            ),
            Line::from(""),
            Line::from(Span::styled(error, Style::default().fg(Color::Red))),
            Line::from(""),
            Line::from(Span::styled("Press 'r' to retry", Style::default().fg(Color::Gray)))
        ];

        let error_block = Paragraph::new(error_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .border_style(Style::default().fg(Color::Red))
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(error_block, area);
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
