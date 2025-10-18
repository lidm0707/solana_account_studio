//! # SurfDesk Desktop Application
//!
//! Professional desktop application for Solana account management.
//!
//! ## Development Status
//!
//! This codebase is currently in active development. Many components, functions,
//! and imports are work-in-progress and will be fully implemented in future
//! iterations. Warning suppression is enabled to facilitate rapid development
//! and iterative feature implementation.

use anyhow::Result;
use clap::Parser;
use dioxus::prelude::*;
use dioxus_desktop::launch;
use log::{error, info, LevelFilter};
use std::sync::Arc;

mod keyboard;
mod pages;
mod surfpool;

// Simple notification component
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
}

#[component]
fn NotificationCenter(children: Element) -> Element {
    rsx! {
        div { class: "notification-center", {children} }
    }
}

// Import core components
use pages::{AccountsPage, DashboardPage, SurfPoolPage};
use surfdesk_core::components::{Button, Card, Loading, Size};
use surfpool::{SurfPoolConfig, SurfPoolManager};

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[command(
    name = "surfdesk-desktop",
    about = "SurfDesk Desktop - Professional Solana Account Studio",
    version,
    author
)]
struct Args {
    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Theme preference (light, dark, auto)
    #[arg(long, default_value = "auto")]
    theme: String,
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current theme
    pub theme: Signal<Theme>,
    /// Current page/view
    pub current_view: Signal<DesktopView>,
    /// SurfPool manager
    pub surfpool_manager: Arc<SurfPoolManager>,
    /// Application settings
    pub settings: Signal<AppSettings>,
    /// Notification system
    pub notifications: Signal<Vec<Notification>>,
}

/// Application theme
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Desktop views/pages
/// Desktop application views
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesktopView {
    Dashboard,
    Accounts,
    Transactions,
    SurfPool,
    Analytics,
    Settings,
}

/// Application settings
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Theme preference
    pub theme: Theme,
    /// Auto-start SurfPool
    pub auto_start_surfpool: bool,
    /// Default network
    pub default_network: String,
    /// Enable notifications
    pub enable_notifications: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Auto,
            auto_start_surfpool: false,
            default_network: "devnet".to_string(),
            enable_notifications: true,
        }
    }
}

/// Navigation item component
#[component]
fn NavigationItem(
    view: DesktopView,
    current_view: DesktopView,
    on_click: EventHandler<DesktopView>,
    icon: String,
    label: String,
) -> Element {
    let is_active = view == current_view;

    rsx! {
        div {
            class: format!("nav-item {}", if is_active { "active" } else { "" }),
            onclick: move |_| on_click(view),
            span { class: "nav-icon", "{icon}" }
            span { class: "nav-label", "{label}" }
        }
    }
}

/// Menu bar component
#[component]
fn MenuBar(
    current_view: Signal<DesktopView>,
    on_view_change: EventHandler<DesktopView>,
) -> Element {
    rsx! {
        div { class: "menu-bar",
            div { class: "menu-section",
                div { class: "menu-item",
                    span { class: "menu-logo", "🏄" }
                    span { class: "menu-title", "SurfDesk" }
                }
            }
        }
    }
}

/// Sidebar component
#[component]
fn Sidebar(
    current_view: Signal<DesktopView>,
    on_view_change: EventHandler<DesktopView>,
) -> Element {
    rsx! {
        div { class: "sidebar",
            NavigationItem {
                view: DesktopView::Dashboard,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "📊".to_string(),
                label: "Dashboard".to_string(),
            }
            NavigationItem {
                view: DesktopView::Accounts,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "🏦".to_string(),
                label: "Accounts".to_string(),
            }
            NavigationItem {
                view: DesktopView::Transactions,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "🔧".to_string(),
                label: "Transactions".to_string(),
            }
            NavigationItem {
                view: DesktopView::SurfPool,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "🌊".to_string(),
                label: "SurfPool".to_string(),
            }
            NavigationItem {
                view: DesktopView::Analytics,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "📈".to_string(),
                label: "Analytics".to_string(),
            }
            NavigationItem {
                view: DesktopView::Settings,
                current_view: current_view(),
                on_click: on_view_change,
                icon: "⚙️".to_string(),
                label: "Settings".to_string(),
            }
        }
    }
}

/// Main application component
#[component]
fn SurfDeskDesktopApp() -> Element {
    // Args are not needed in the component for now
    let surfpool_config = SurfPoolConfig::default();
    let surfpool_manager = Arc::new(SurfPoolManager::new(surfpool_config));

    // Initialize application state
    let mut app_state = AppState {
        theme: use_signal(|| Theme::Auto),
        current_view: use_signal(|| DesktopView::Dashboard),
        surfpool_manager: surfpool_manager.clone(),
        settings: use_signal(AppSettings::default),
        notifications: use_signal(Vec::<Notification>::new),
    };

    // Provide context to child components
    use_context_provider(|| app_state.clone());

    // Apply theme
    let theme_signal = app_state.theme;
    let theme_class = match theme_signal() {
        Theme::Light => "theme-light",
        Theme::Dark => "theme-dark",
        Theme::Auto => "theme-auto",
    };

    rsx! {
        // Include styles from core
        style { {include_str!("../../surfdesk-core/src/styles/styles.css")} }
        style { {surfdesk_core::styles::include_all_styles()} }

        div {
            class: "surfdesk-desktop {theme_class}",

            // Menu bar
            MenuBar {
                current_view: app_state.current_view,
                on_view_change: move |view| {
                    app_state.current_view.set(view);
                },
            }

            // Main content area
            div { class: "desktop-main",

                // Sidebar navigation
                Sidebar {
                    current_view: app_state.current_view,
                    on_view_change: move |view| {
                        app_state.current_view.set(view);
                    },
                }

                // Content area
                div { class: "desktop-content",
                    // Render current view
                    {let current_view_signal = (app_state.current_view)();
                    match current_view_signal {
                        DesktopView::Dashboard => {
                            rsx! { DashboardPage {} }
                        }
                        DesktopView::Accounts => {
                            rsx! {
                                AccountsPage {}
                            }
                        }
                        DesktopView::Transactions => {
                            rsx! {
                                div { class: "placeholder-page",
                                    h2 { "Transactions Page" }
                                    p { "Coming soon..." }
                                }
                            }
                        }
                        DesktopView::SurfPool => {
                            rsx! {
                                SurfPoolPage {}
                            }
                        }
                        DesktopView::Analytics => {
                            rsx! {
                                div { class: "placeholder-page",
                                    h2 { "Analytics Page" }
                                    p { "Coming soon..." }
                                }
                            }
                        }
                        DesktopView::Settings => {
                            rsx! {
                                div { class: "placeholder-page",
                                    h2 { "Settings Page" }
                                    p { "Coming soon..." }
                                }
                            }
                        }
                    }}
                }
            }

            // Notification system
            NotificationCenter {}
        }
    }
}

/// Main entry point
fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = match args.log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();

    info!("Starting SurfDesk Desktop application");
    info!("Arguments: {:?}", args);

    // Initialize core library using a blocking runtime for startup
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        surfdesk_core::init_core().await.map_err(|e| {
            error!("Failed to initialize core library: {}", e);
            e
        })
    })?;

    // Launch Dioxus desktop (manages its own async runtime)
    launch(SurfDeskDesktopApp);

    info!("SurfDesk Desktop application terminated gracefully");
    Ok(())
}
