//! # SurfDesk Desktop Application - Enhanced Edition
//!
//! Professional desktop application for Solana account management.
//! Features enhanced UI/UX, SurfPool integration, and desktop-specific capabilities.
//!
//! ## Features
//! - Professional design system with dark/light themes
//! - SurfPool local validator management
//! - Advanced account management and analytics
//! - Native desktop integrations (menu bar, system tray, shortcuts)
//! - Component-based architecture with reusable UI components

use anyhow::Result;
use clap::Parser;
use dioxus::prelude::*;
use log::{debug, error, info, warn, LevelFilter};
use std::sync::Arc;

mod keyboard;
mod pages;

use components::*;
use surfpool::SurfPoolManager;

/// Command line arguments for the enhanced desktop application
#[derive(Parser, Debug)]
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

    /// Configuration file path
    #[arg(short, long, default_value = "./config/.env")]
    config: String,

    /// Enable development mode
    #[arg(short, long)]
    dev: bool,

    /// Window width
    #[arg(long, default_value = "1400")]
    width: u32,

    /// Window height
    #[arg(long, default_value = "900")]
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

    /// Start with SurfPool enabled
    #[arg(long)]
    start_surfpool: bool,

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
#[derive(Debug, Clone, PartialEq)]
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
    /// Window settings
    pub window_settings: WindowSettings,
}

/// Window settings
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// Remember window position
    pub remember_position: bool,
    /// Remember window size
    pub remember_size: bool,
    /// Start maximized
    pub start_maximized: bool,
    /// Show in system tray
    pub show_in_tray: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Auto,
            auto_start_surfpool: false,
            default_network: "devnet".to_string(),
            enable_notifications: true,
            window_settings: WindowSettings {
                remember_position: true,
                remember_size: true,
                start_maximized: false,
                show_in_tray: true,
            },
        }
    }
}

/// Notification for the user
#[derive(Debug, Clone)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether it's read
    pub read: bool,
}

/// Notification types
#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// Main application component
#[component]
fn SurfDeskDesktopApp() -> Element {
    let args = use_context::<Args>();
    let surfpool_config = SurfPoolConfig::default();
    let surfpool_manager = Arc::new(SurfPoolManager::new(surfpool_config));

    // Initialize application state
    let app_state = AppState {
        theme: use_signal(|| match args.theme.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }),
        current_view: use_signal(|| DesktopView::Dashboard),
        surfpool_manager: surfpool_manager.clone(),
        settings: use_signal(AppSettings::default),
        notifications: use_signal(Vec::<Notification>::new),
    };

    // Provide context to child components
    use_context_provider(|| app_state.clone());

    // Apply theme
    let theme_class = match app_state.theme() {
        Theme::Light => "theme-light",
        Theme::Dark => "theme-dark",
        Theme::Auto => "theme-auto",
    };

    rsx! {
        // Include styles from core
        style { {include_str!("../assets/styles.css")} }
        style { {surfdesk_core::styles::include_all_styles()} }

        div {
            class: "surfdesk-desktop {theme_class}",

            // Menu bar
            MenuBar {
                current_view: app_state.current_view,
                on_view_change: move |view| {
                    app_state.current_view.set(view);
                },
                surfpool_manager: app_state.surfpool_manager.clone(),
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
                    match app_state.current_view() {
                        DesktopView::Dashboard => {
                            DashboardPage {
                                surfpool_manager: app_state.surfpool_manager.clone(),
                            }
                        }
                        DesktopView::Accounts => {
                            rsx! {
                                div { class: "placeholder-page",
                                    h2 { "Accounts Page" }
                                    p { "Coming soon..." }
                                    LoadingSpinner {
                                        size: Size::Medium,
                                        text: Some("Loading accounts...".to_string()),
                                        variant: LoadingVariant::Dots,
                                    }
                                }
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
                                div { class: "placeholder-page",
                                    h2 { "SurfPool Page" }
                                    p { "Coming soon..." }
                                }
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
                    }
                }
            }

            // Notification system
            NotificationCenter {
                notifications: app_state.notifications,
            }
        }
    }
}

/// Menu bar component
#[component]
fn MenuBar(
    current_view: Signal<DesktopView>,
    on_view_change: EventHandler<DesktopView>,
    surfpool_manager: Arc<SurfPoolManager>,
) -> Element {
    let surfpool_status = use_signal(|| surfpool_manager.get_status());

    // Update SurfPool status periodically
    use_coroutine(|_| {
        let manager = Arc::clone(&surfpool_manager);
        let status = surfpool_status.clone();

        async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
            loop {
                interval.tick().await;
                status.set(manager.get_status());
            }
        }
    });

    rsx! {
        div { class: "menu-bar",

            // App menu
            div { class: "menu-section",
                div { class: "menu-item",
                    span { class: "menu-logo", "🏄" }
                    span { class: "menu-title", "SurfDesk" }
                }
            }

            // Navigation menu
            div { class: "menu-section nav-menu",

                NavigationItem {
                    view: DesktopView::Dashboard,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "📊",
                    label: "Dashboard",
                }

                NavigationItem {
                    view: DesktopView::Accounts,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "🏦",
                    label: "Accounts",
                }

                NavigationItem {
                    view: DesktopView::Transactions,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "🔧",
                    label: "Transactions",
                }

                NavigationItem {
                    view: DesktopView::SurfPool,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "🌊",
                    label: "SurfPool",
                    badge: match surfpool_status() {
                        surfpool::SurfPoolStatus::Running { .. } => Some("●".to_string()),
                        _ => None,
                    },
                }

                NavigationItem {
                    view: DesktopView::Analytics,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "📈",
                    label: "Analytics",
                }

                NavigationItem {
                    view: DesktopView::Settings,
                    current_view: current_view(),
                    on_click: on_view_change,
                    icon: "⚙️",
                    label: "Settings",
                }
            }

            // System menu
            div { class: "menu-section system-menu",

                // SurfPool status indicator
                div { class: "surfpool-indicator",
                    span { class: format!("surfpool-status surfpool-status-{:?}", surfpool_status()),
                        match surfpool_status() {
                            surfpool::SurfPoolStatus::Running { .. } => "🟢",
                            surfpool::SurfPoolStatus::Starting => "🟡",
                            surfpool::SurfPoolStatus::Stopping => "🟡",
                            surfpool::SurfPoolStatus::Stopped => "⚪",
                            surfpool::SurfPoolStatus::Error { .. } => "🔴",
                        }
                    }
                }


                // Window controls
                div { class: "window-controls",
                    button { class: "window-control minimize", "─" }
                    button { class: "window-control maximize", "□" }
                    button { class: "window-control close", "✕" }
                }
            }
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
    badge: Option<String>,
) -> Element {
    let is_active = current_view == view;

    rsx! {
        button {
            class: format!("nav-item {}", if is_active { "active" } else { "" }),
            onclick: move |_| on_click.call(view),

            span { class: "nav-icon", "{icon}" }
            span { class: "nav-label", "{label}" }

            if let Some(badge_text) = badge {
                span { class: "nav-badge", "{badge_text}" }
            }
        }
    }
}

/// Sidebar navigation component
#[component]
fn Sidebar(
    current_view: Signal<DesktopView>,
    on_view_change: EventHandler<DesktopView>,
) -> Element {
    rsx! {
            aside { class: "sidebar",

                // Quick actions
                div { class: "sidebar-section",
                    h3 { class: "sidebar-title", "Quick Actions" }

                    div { class: "quick-actions",
                        Button {
                            variant: ButtonVariant::Primary,
                            size: Size::Small,
                            full_width: true,
                            icon: Some("🚀".to_string()),
                            onclick: move |_| {
                                // Quick account creation
                            },
                            "Create Account"
                        }

                        Button {
                            variant: ButtonVariant::Secondary,
                            size: Size::Small,
                            full_width: true,
                            icon: Some("📥".to_string()),
                            onclick: move |_| {
                                // Import account
                            },
                            "Import Account"
                        }

                        Button {
                            variant: ButtonVariant::Tertiary,
                            size: Size::Small,
                            full_width: true,
                            icon: Some("💰".to_string()),
                            onclick: move |_| {
                                // Request airdrop
                            },
                            "Request Airdrop"
                        }
                    }
                // }

                // Recent activity
                div { class: "sidebar-section",
                    h3 { class: "sidebar-title", "Recent Activity" }

                    div { class: "activity-list",
                        // Mock activity items
                        div { class: "activity-item",
                            span { class: "activity-icon", "✅" }
                            div { class: "activity-content",
                                span { class: "activity-title", "Account Created" }
                                span { class: "activity-time", "2 min ago" }
                            }
                        }

                        div { class: "activity-item",
                            span { class: "activity-icon", "💸" }
                            div { class: "activity-content",
                                span { class: "activity-title", "Transaction Sent" }
                                span { class: "activity-time", "5 min ago" }
                            }
                        }

                        div { class: "activity-item",
                            span { class: "activity-icon", "🔄" }
                            div { class: "activity-content",
                                span { class: "activity-title", "Balance Updated" }
                                span { class: "activity-time", "10 min ago" }
                            }
                        }
                    }
                }
            }
        }
    }

    rsx! {
        div { class: "status-bar",

            // Left side status
            div { class: "status-left",
                span { class: "status-item",
                    "Network: Devnet"
                }

                span { class: "status-item",
                    "Accounts: 3"
                }

                span { class: "status-item",
                    "SurfPool: Ready"
                }
            }

            // Right side status
            div { class: "status-right",
                span { class: "status-item",
                    "Version: {surfdesk_core::VERSION}"
                }

                span { class: "status-item",
                    "12:34:56"
                }
            }
        }
    }
}

// /// Notification center component
// #[component]
// fn NotificationCenter(notifications: Signal<Vec<Notification>>) -> Element {
//     let show_notifications = use_signal(|| false);
//     let unread_count = use_signal(|| notifications().iter().filter(|n| !n.read).count());

//     rsx! {
//         div { class: "notification-center",

//             // Notification bell
//             button {
//                 class: "notification-bell",
//                 onclick: move |_| show_notifications.set(!show_notifications()),

//                 span { class: "notification-icon", "🔔" }

//                 if unread_count() > 0 {
//                     span { class: "notification-badge", "{unread_count()}" }
//                 }
//             }

//             // Notification dropdown
//             if show_notifications() {
//                 div { class: "notification-dropdown",
//                     h3 { "Notifications" }

//                     div { class: "notification-list",
//                         for notification in notifications() {
//                             div { class: format!("notification-item {}", if notification.read { "read" } else { "unread" }),
//                                 div { class: "notification-header",
//                                     span { class: "notification-title", "{notification.title}" }
//                                     span { class: "notification-time",
//                                         "12:34"
//                                     }
//                                 }
//                                 p { class: "notification-message", "{notification.message}" }
//                             }
//                         }
//                     }

//                     if notifications().is_empty() {
//                         div { class: "notification-empty",
//                             span { "No notifications" }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

/// Initialize logging with enhanced formatting
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
        .format_module_path(false)
        .format_target(false)
        .init();

    info!("SurfDesk Desktop logging initialized at level: {}", level);
    Ok(())
}

/// Load configuration from file and environment
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

    // Log configuration
    info!("SurfPool configuration: port=8999, fork_mainnet=true");
    info!("Desktop features: menu_bar, system_tray, notifications");

    Ok(())
}

/// Main function - application entry point
fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging first
    init_logging(&args.log_level)?;

    // Load configuration
    load_config(&args.config)?;

    info!("🏄 Starting SurfDesk Desktop Enhanced Application...");
    info!("Platform: {}", surfdesk_core::current_platform());
    info!("Version: {}", surfdesk_core::VERSION);
    info!("Window: {}x{}", args.width, args.height);
    info!("Theme: {}", args.theme);

    // Initialize core library
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        surfdesk_core::init_core().await.map_err(|e| {
            error!("Failed to initialize core library: {}", e);
            e
        })
    })?;

    // Configure and launch Dioxus desktop
    dioxus::launch_cfg(
        SurfDeskDesktopApp,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("SurfDesk Desktop - Professional Solana Account Studio")
                .with_inner_size(dioxus_desktop::LogicalSize::new(args.width, args.height))
                .with_min_inner_size(dioxus_desktop::LogicalSize::new(1200, 800))
                .with_resizable(!args.no_resize)
                .with_always_on_top(args.always_on_top)
                .with_fullscreen(args.fullscreen),
        ),
    );

    info!("SurfDesk Desktop application terminated gracefully");
    Ok(())
}
