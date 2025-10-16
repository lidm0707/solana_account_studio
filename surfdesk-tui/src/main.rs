//! # SurfDesk Terminal UI Application
//!
//! This is the main entry point for the SurfDesk terminal UI application.
//! It provides a command-line interface for the Solana account studio
//! using ratatui and crossterm for cross-platform terminal UI development.

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info, LevelFilter};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};
use surfdesk_core::{current_platform, init_core};

/// Command line arguments for the TUI application
#[derive(Parser, Debug)]
#[command(
    name = "surfdesk-tui",
    about = "SurfDesk Terminal UI - Multi-platform Solana Account Studio",
    version,
    author
)]
struct Args {
    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Configuration file path
    #[arg(short, long, default_value = "./config/.env")]
    config: String,

    /// Enable mouse support
    #[arg(short, long)]
    mouse: bool,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Refresh rate in milliseconds
    #[arg(long, default_value = "100")]
    refresh_rate: u64,
}

/// Application state
struct App {
    /// Current selected menu item
    selected: usize,
    /// Menu items
    items: Vec<String>,
    /// Connection status
    connection_status: String,
    /// Progress value (0-100)
    progress: u16,
    /// Input text
    input: String,
    /// Input mode
    input_mode: bool,
    /// Messages
    messages: Vec<String>,
}

impl App {
    /// Create a new application instance
    fn new() -> Self {
        Self {
            selected: 0,
            items: vec![
                "🔗 Connect to Solana".to_string(),
                "🏦 Browse Accounts".to_string(),
                "🔧 Build Transaction".to_string(),
                "🤖 AI Assistant".to_string(),
                "⚙️  Settings".to_string(),
                "❌ Quit".to_string(),
            ],
            connection_status: "Disconnected".to_string(),
            progress: 0,
            input: String::new(),
            input_mode: false,
            messages: vec![
                "Welcome to SurfDesk Terminal UI!".to_string(),
                "Use arrow keys to navigate, Enter to select.".to_string(),
                "Press 'q' to quit at any time.".to_string(),
            ],
        }
    }

    /// Move selection up
    fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move selection down
    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    /// Get current selected item
    fn selected_item(&self) -> &str {
        &self.items[self.selected]
    }

    /// Add a message
    fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
    }
}

/// Main application component
fn ui(f: &mut Frame, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("🏄 SurfDesk Terminal UI")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("Header"));
    f.render_widget(header, chunks[0]);

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // Left panel - Menu
    let menu_items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let menu = List::new(menu_items).block(Block::default().borders(Borders::ALL).title("Menu"));
    f.render_widget(menu, main_chunks[0]);

    // Right panel - Content
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(main_chunks[1]);

    // Status bar
    let status_text = format!(
        "Status: {} | Platform: {}",
        app.connection_status,
        current_platform()
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, content_chunks[0]);

    // Progress bar
    let progress = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Gray))
        .percent(app.progress);
    f.render_widget(progress, content_chunks[1]);

    // Messages area
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|msg| ListItem::new(msg.as_str()))
        .collect();
    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(messages_list, content_chunks[2]);

    // Input area (if in input mode)
    if app.input_mode {
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3)])
            .split(content_chunks[3]);

        let input_text = Paragraph::new(app.input.as_str())
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(Clear, input_area[0]); // Clear the area
        f.render_widget(input_text, input_area[0]);
    }

    // Footer
    let footer_text = match app.input_mode {
        true => "Enter text | Esc to exit input mode",
        false => "↑↓ Navigate | Enter Select | q Quit | i Input mode",
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(footer, chunks[2]);
}

/// Handle keyboard events
fn handle_key_events(key_event: crossterm::event::KeyEvent, app: &mut App) -> Result<bool> {
    if app.input_mode {
        if let KeyEventKind::Press = key_event.kind {
            match key_event.code {
                KeyCode::Enter => {
                    if !app.input.is_empty() {
                        app.add_message(format!("Input: {}", app.input));
                        app.input.clear();
                    }
                    app.input_mode = false;
                }
                KeyCode::Esc => app.input_mode = false,
                KeyCode::Char(c) => app.input.push(c),
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
        }
    }

    if let KeyEventKind::Press = key_event.kind {
        match key_event.code {
            KeyCode::Up => app.previous(),
            KeyCode::Down => app.next(),
            KeyCode::Enter => match app.selected_item() {
                "🔗 Connect to Solana" => {
                    app.connection_status = "Connecting...".to_string();
                    app.add_message("Connecting to Solana network...".to_string());
                    app.progress = 100;
                    app.connection_status = "Connected".to_string();
                    app.add_message("✅ Connected to Solana devnet".to_string());
                }
                "🏦 Browse Accounts" => app.add_message("Opening account browser...".to_string()),
                "🔧 Build Transaction" => {
                    app.add_message("Opening transaction builder...".to_string())
                }
                "🤖 AI Assistant" => app.add_message("Opening AI assistant...".to_string()),
                "⚙️  Settings" => app.add_message("Opening settings...".to_string()),
                "❌ Quit" => return Ok(true),
                _ => {}
            },
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('i') => app.input_mode = true,
            KeyCode::Char('r') => {
                app.progress = 0;
                app.connection_status = "Disconnected".to_string();
                app.add_message("Reset connection status".to_string());
            }
            _ => {}
        }
    }

    Ok(false)
}

/// Initialize logging
fn init_logging(level: &str) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .format_timestamp_secs()
        .init();

    info!("Logging initialized at level: {}", level);
    Ok(())
}

/// Load configuration from file
fn load_config(config_path: &str) -> Result<()> {
    if std::path::Path::new(config_path).exists() {
        dotenvy::from_filename(config_path)?;
        info!("Configuration loaded from: {}", config_path);
    } else {
        info!(
            "Configuration file not found: {} (using defaults)",
            config_path
        );
    }
    Ok(())
}

/// Main function
fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging first
    init_logging(&args.log_level)?;

    // Load configuration
    load_config(&args.config)?;

    info!("Starting SurfDesk Terminal UI application...");
    info!("Platform: {}", current_platform());
    info!("Version: {}", surfdesk_core::VERSION);

    // Initialize core library
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        if let Err(e) = init_core().await {
            error!("Failed to initialize core: {}", e);
            return Err(e);
        }
        Ok(())
    })?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create application
    let mut app = App::new();

    // Run the application
    let mut should_quit = false;
    while !should_quit {
        // Draw UI
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle events
        if event::poll(Duration::from_millis(args.refresh_rate))? {
            if let Event::Key(key) = event::read()? {
                should_quit = handle_key_events(key, &mut app)?;
            }
        }

        // Simulate some progress animation
        if app.connection_status == "Connecting..." && app.progress < 100 {
            app.progress = (app.progress + 1).min(100);
            if app.progress == 100 {
                app.connection_status = "Connected".to_string();
                app.add_message("✅ Connected to Solana network!".to_string());
            }
        }
    }

    // Restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    info!("SurfDesk Terminal UI application exited gracefully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.selected, 0);
        assert_eq!(app.items.len(), 6);
        assert_eq!(app.connection_status, "Disconnected");
    }

    #[test]
    fn test_app_navigation() {
        let mut app = App::new();
        app.next();
        assert_eq!(app.selected, 1);
        app.previous();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_args_parsing() {
        use clap::Parser;

        let args = Args::try_parse_from([
            "surfdesk-tui",
            "--log-level",
            "debug",
            "--mouse",
            "--refresh-rate",
            "200",
        ])
        .unwrap();

        assert_eq!(args.log_level, "debug");
        assert!(args.mouse);
        assert_eq!(args.refresh_rate, 200);
    }

    #[test]
    fn test_logging_init() {
        let result = init_logging("info");
        assert!(result.is_ok());
    }
}
