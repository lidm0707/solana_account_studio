//! # SurfDesk Desktop Application
//!
//! This is the main entry point for the SurfDesk desktop application.
//! It provides a native desktop experience for the Solana account studio
//! using Dioxus for cross-platform GUI development.

use anyhow::Result;
use clap::Parser;
use dioxus::prelude::*;
use log::{error, info, LevelFilter};

use surfdesk_core::{current_platform, init_core};

/// Command line arguments for the desktop application
#[derive(Parser, Debug)]
#[command(
    name = "surfdesk-desktop",
    about = "SurfDesk Desktop - Multi-platform Solana Account Studio",
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

    /// Enable development mode
    #[arg(short, long)]
    dev: bool,

    /// Window width
    #[arg(long, default_value = "1200")]
    width: u32,

    /// Window height
    #[arg(long, default_value = "800")]
    height: u32,

    /// Enable fullscreen mode
    #[arg(long)]
    fullscreen: bool,

    /// Enable always on top
    #[arg(long)]
    always_on_top: bool,

    /// Disable window resizing
    #[arg(long)]
    no_resize: bool,
}

/// Main application component
#[component]
fn SurfDeskDesktopApp() -> Element {
    let mut count = use_signal(|| 0);
    let platform_info = use_signal(|| current_platform().to_string());

    rsx! {
        style { {include_str!("../assets/styles.css")} }

        div { class: "desktop-app",
            div { class: "header",
                h1 { "SurfDesk Desktop" }
                p { "Multi-platform Solana Account Studio" }
                div { class: "platform-info",
                    span { "Platform: {platform_info}" }
                }
            }

            div { class: "main-content",
                div { class: "welcome-card",
                    h2 { "Welcome to SurfDesk!" }
                    p { "Your comprehensive Solana account studio is ready to use." }

                    div { class: "feature-grid",
                        div { class: "feature-card",
                            h3 { "🔗 Solana Integration" }
                            p { "Connect to Solana mainnet, devnet, testnet, or local validator" }
                        }

                        div { class: "feature-card",
                            h3 { "🏦 Account Management" }
                            p { "Explore, analyze, and manage Solana accounts and programs" }
                        }

                        div { class: "feature-card",
                            h3 { "🔧 Transaction Builder" }
                            p { "Build, simulate, and send transactions with confidence" }
                        }

                        div { class: "feature-card",
                            h3 { "🤖 AI-Powered Testing" }
                            p { "Generate intelligent test cases and analyze results" }
                        }
                    }

                    div { class: "demo-section",
                        h3 { "Interactive Demo" }
                        p { "Click count: {count}" }
                        button {
                            onclick: move |_| count += 1,
                            "Click me!"
                        }
                        button {
                            onclick: move |_| count.set(0),
                            "Reset"
                        }
                    }
                }

                div { class: "status-panel",
                    h3 { "System Status" }
                    div { class: "status-grid",
                        div { class: "status-item",
                            span { class: "status-label", "Core Library:" }
                            span { class: "status-value status-ok", "✓ Initialized" }
                        }
                        div { class: "status-item",
                            span { class: "status-label", "Platform:" }
                            span { class: "status-value", "{current_platform()}" }
                        }
                        div { class: "status-item",
                            span { class: "status-label", "GUI:" }
                            span { class: "status-value status-ok", "✓ Dioxus Desktop" }
                        }
                        div { class: "status-item",
                            span { class: "status-label", "Features:" }
                            span { class: "status-value", "Desktop + Solana + Database" }
                        }
                    }
                }
            }

            div { class: "footer",
                p { "SurfDesk v{surfdesk_core::VERSION} - Built with Dioxus & Rust" }
                div { class: "footer-links",
                    a { href: "https://github.com/your-org/surfdesk", "GitHub" }
                    span { " • " }
                    a { href: "https://docs.surfdesk.dev", "Documentation" }
                    span { " • " }
                    a { href: "https://discord.gg/surfdesk", "Discord" }
                }
            }
        }
    }
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
    // Load environment variables from .env file if it exists
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

    info!("Starting SurfDesk Desktop application...");
    info!("Platform: {}", current_platform());
    info!("Version: {}", surfdesk_core::VERSION);

    // Initialize core library
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        init_core().await.map_err(|e| {
            error!("Failed to initialize core library: {}", e);
            e
        })
    })?;

    // Configure Dioxus desktop
    dioxus::launch(SurfDeskDesktopApp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        use clap::Parser;

        let args = Args::try_parse_from([
            "surfdesk-desktop",
            "--log-level",
            "debug",
            "--width",
            "1024",
            "--height",
            "768",
        ])
        .unwrap();

        assert_eq!(args.log_level, "debug");
        assert_eq!(args.width, 1024);
        assert_eq!(args.height, 768);
    }

    #[test]
    fn test_logging_init() {
        let result = init_logging("info");
        assert!(result.is_ok());
    }
}
