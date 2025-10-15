//! # Main Application Module (MVP Version)
//!
//! Simplified main application structure for SurfDesk.
//! Focus on compilation success over comprehensive features.
//! MVP-first approach following BREAKTOOL methodology.

use crate::{platform::Platform, state::AppState, types::UIState};
use dioxus::prelude::*;

/// Props for platform shell components
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PlatformShellProps {
    state: Signal<AppState>,
    ui_state: Signal<UIState>,
}

/// Main SurfDesk application component
#[component]
pub fn SurfDeskApp() -> Element {
    // Initialize global application state
    let state = use_signal(AppState::new);
    let ui_state = use_signal(UIState::default);

    // Platform detection
    let platform = Platform::current();

    rsx! {
        div {
            class: "surfdesk-app {platform_class(platform)}",

            // Header
            div { class: "app-header",
                h1 { "SurfDesk - Solana Account Studio" }
                div { class: "platform-info", "Platform: {platform:?}" }
            }

            // Main content area
            div { class: "main-content",

                // Sidebar
                div { class: "sidebar",
                    h3 { "Navigation" }
                    ul {
                        li { "Dashboard" }
                        li { "Account Explorer" }
                        li { "Transaction Builder" }
                        li { "Program Manager" }
                        li { "Settings" }
                    }
                }

                // Content area
                div { class: "content-area",
                    h2 { "Dashboard" }
                    div { class: "dashboard-grid",
                        div { class: "stat-card",
                            h3 { "Projects" }
                            div { class: "stat-value", "0" }
                        }
                        div { class: "stat-card",
                            h3 { "Accounts" }
                            div { class: "stat-value", "0" }
                        }
                        div { class: "stat-card",
                            h3 { "Transactions" }
                            div { class: "stat-value", "0" }
                        }
                        div { class: "stat-card",
                            h3 { "Programs" }
                            div { class: "stat-value", "0" }
                        }
                    }

                    div { class: "welcome-section",
                        h3 { "Welcome to SurfDesk!" }
                        p { "Your comprehensive Solana account management studio." }
                        div { class: "action-buttons",
                            button { class: "btn-primary", "Create Project" }
                            button { class: "btn-secondary", "Import Account" }
                            button { class: "btn-secondary", "Deploy Program" }
                        }
                    }
                }
            }

            // Footer
            div { class: "app-footer",
                p { "SurfDesk v0.1.0 - MVP Version" }
            }
        }
    }
}

/// Get platform-specific CSS class
fn platform_class(platform: Platform) -> &'static str {
    match platform {
        Platform::Desktop => "platform-desktop",
        Platform::Web => "platform-web",
        Platform::Terminal => "platform-terminal",
    }
}

/// Dashboard component
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "dashboard-view",
            h2 { "Dashboard" }
            p { "Dashboard functionality coming soon..." }
        }
    }
}

/// Account Explorer component
#[component]
pub fn AccountExplorer() -> Element {
    rsx! {
        div { class: "account-explorer-view",
            h2 { "Account Explorer" }
            p { "Account explorer functionality coming soon..." }
        }
    }
}

/// Transaction Builder component
#[component]
pub fn TransactionBuilder() -> Element {
    rsx! {
        div { class: "transaction-builder-view",
            h2 { "Transaction Builder" }
            p { "Transaction builder functionality coming soon..." }
        }
    }
}

/// Program Manager component
#[component]
pub fn ProgramManager() -> Element {
    rsx! {
        div { class: "program-manager-view",
            h2 { "Program Manager" }
            p { "Program management functionality coming soon..." }
        }
    }
}

/// Settings component
#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "settings-view",
            h2 { "Settings" }
            p { "Settings functionality coming soon..." }
        }
    }
}

/// Initialize application state
pub async fn init_app() -> crate::error::Result<()> {
    log::info!("Initializing SurfDesk application");

    // Initialize core services
    // TODO: Add actual initialization logic

    Ok(())
}

/// Run the application
pub fn run_app() {
    log::info!("Starting SurfDesk application");
    // The actual running logic would be platform-specific
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_class() {
        assert_eq!(platform_class(Platform::Desktop), "platform-desktop");
        assert_eq!(platform_class(Platform::Web), "platform-web");
        assert_eq!(platform_class(Platform::Terminal), "platform-terminal");
    }

    #[test]
    fn test_app_structure() {
        // Basic smoke test - if we can create the app, the structure works
        let _app = SurfDeskApp;
    }

    async fn test_init_app() {
        let result = init_app().await;
        assert!(result.is_ok());
    }

    fn test_run_app() {
        run_app(); // Should not panic
    }
}
