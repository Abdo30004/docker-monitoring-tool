mod app;
mod ui;
mod terminal_manager;

pub use app::App;
use anyhow::Result;
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use ratatui::{ backend::CrosstermBackend, Terminal };
use std::io;
use std::time::Duration;

pub async fn run_tui(refresh_interval: u64) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(refresh_interval);

    app.update().await?;

    let result = run_app(&mut terminal, &mut app, refresh_interval).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    refresh_interval: u64
) -> Result<()> {
    let tick_rate = Duration::from_secs(refresh_interval);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return Ok(());
                        }
                        KeyCode::Char('r') => app.update().await?,
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.previous_tab(),
                        KeyCode::Char('1') => app.select_tab(0),
                        KeyCode::Char('2') => app.select_tab(1),
                        KeyCode::Char('3') => app.select_tab(2),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update().await?;
            last_tick = std::time::Instant::now();
        }
    }
}
