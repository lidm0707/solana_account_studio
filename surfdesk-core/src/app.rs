//! # Main Application Module
//!
//! This module contains the main application structure and logic for SurfDesk.
//! It provides the core application framework that can be used across all platforms
//! (desktop, web, and terminal) with platform-specific adaptations.

use crate::{error::Result, platform::Platform, state::AppState, types::UIState};
use dioxus::prelude::*;

/// Main SurfDesk application component
///
/// This is the root component that manages the entire application state and
/// provides the platform-agnostic UI structure. Platform-specific components
/// are rendered based on the current platform.
#[component]
pub fn SurfDeskApp() -> Element {
    // Initialize global application state
    let state = use_signal(AppState::new);
    let ui_state = use_signal(UIState::default);

    // Platform detection
    let platform = Platform::current();

    // Effect for handling global state changes
    use_effect(move || {
        let state = state.read();
        let ui_state = ui_state.read();

        // Log state changes for debugging
        log::debug!(
            "App state updated: {} projects, theme: {:?}",
            state.projects.read().len(),
            ui_state.read().theme
        );
    });

    rsx! {
        div {
            class: "app-root {platform_class(platform)}",
            match platform {
                Platform::Desktop => rsx! {
                    DesktopAppShell {
                        state: state,
                        ui_state: ui_state
                    }
                },
                Platform::Web => rsx! {
                    WebAppShell {
                        state: state,
                        ui_state: ui_state
                    }
                },
                Platform::Terminal => rsx! {
                    TerminalAppShell {
                        state: state,
                        ui_state: ui_state
                    }
                }
            }
        }
    }
}

/// Get CSS class for platform
fn platform_class(platform: Platform) -> &'static str {
    match platform {
        Platform::Desktop => "platform-desktop",
        Platform::Web => "platform-web",
        Platform::Terminal => "platform-terminal",
    }
}

/// Desktop application shell
#[component]
fn DesktopAppShell(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "desktop-shell",
            // Header with native controls
            div { class: "header", "SurfDesk Header" }

            // Main content area
            div { class: "main-area",
                Sidebar {
                    state: state,
                    ui_state: ui_state
                }

                MainContent {
                    state: state,
                    ui_state: ui_state
                }
            }

            // Status bar
            StatusBar {
                state: state
            }
        }
    }
}

/// Web application shell
#[component]
fn WebAppShell(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "web-shell",
            // Web navigation
            WebNavigation {}

            // Header
            div { class: "header", "SurfDesk Web Header" }

            // Main content area
            div { class: "main-area",
                Sidebar {
                    state: state,
                    ui_state: ui_state
                }

                MainContent {
                    state: state,
                    ui_state: ui_state
                }
            }

            // Web footer
            WebFooterLinks {}
        }
    }
}

/// Terminal application shell
#[component]
fn TerminalAppShell(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "terminal-shell",
            // Terminal status bar
            TerminalStatusBar {
                state: state
            }

            // Terminal layout
            div { class: "terminal-main",
                TerminalSidebar {
                    state: state,
                    ui_state: ui_state
                }

                TerminalMainContent {
                    state: state,
                    ui_state: ui_state
                }
            }

            // Terminal prompt
            TerminalPrompt {}
        }
    }
}

/// Sidebar component
#[component]
fn Sidebar(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    let expanded = ui_state.read().sidebar.expanded;

    rsx! {
        div {
            class: "sidebar {sidebar_visibility_class(expanded)}",
            SidebarSection {
                title: "Projects",
                children: rsx! {
                    ProjectList {
                        state: state
                    }
                }
            }

            SidebarSection {
                title: "Environments",
                children: rsx! {
                    EnvironmentList {
                        state: state
                    }
                }
            }

            SidebarSection {
                title: "Accounts",
                children: rsx! {
                    AccountExplorer {
                        state: state
                    }
                }
            }

            SidebarSection {
                title: "Programs",
                children: rsx! {
                    ProgramList {
                        state: state
                    }
                }
            }
        }
    }
}

/// Sidebar section component
#[component]
fn SidebarSection(title: String, children: Element) -> Element {
    rsx! {
        div { class: "sidebar-section",
            h3 { class: "sidebar-section-title", "{title}" }
            div { class: "sidebar-section-content",
                {children}
            }
        }
    }
}

/// Web navigation component
#[component]
fn WebNavigation() -> Element {
    rsx! {
        nav { class: "web-navigation",
            div { class: "nav-brand",
                a { href: "/", "SurfDesk" }
            }

            div { class: "nav-links",
                a { href: "/dashboard", "Dashboard" }
                a { href: "/accounts", "Accounts" }
                a { href: "/transactions", "Transactions" }
                a { href: "/programs", "Programs" }
                a { href: "/settings", "Settings" }
            }

            WebUserMenu {}
        }
    }
}

/// Web user menu component
#[component]
fn WebUserMenu() -> Element {
    rsx! {
        div { class: "web-user-menu",
            button { class: "user-menu-button", "User" }
            div { class: "user-menu-dropdown",
                a { href: "/profile", "Profile" }
                a { href: "/settings", "Settings" }
                a { href: "/logout", "Logout" }
            }
        }
    }
}

/// Web footer links component
#[component]
fn WebFooterLinks() -> Element {
    rsx! {
        footer { class: "web-footer",
            div { class: "footer-links",
                a { href: "/docs", "Documentation" }
                a { href: "/support", "Support" }
                a { href: "/github", "GitHub" }
            }

            div { class: "footer-info",
                p { "© 2024 SurfDesk. All rights reserved." }
            }
        }
    }
}

/// Terminal status bar component
#[component]
fn TerminalStatusBar(state: Signal<AppState>) -> Element {
    let connection_status = state.read().connection_status();
    let current_network = state.read().current_network();

    rsx! {
        div { class: "terminal-status-bar",
            span { class: "status-connection", "{connection_status}" }
            span { class: "status-network", "{current_network}" }
            span { class: "status-time", "12:34:56" }
        }
    }
}

/// Terminal sidebar component
#[component]
fn TerminalSidebar(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "terminal-sidebar",
            div { class: "terminal-nav",
                button { class: "nav-item active", "[Dashboard]" }
                button { class: "nav-item", "[Accounts]" }
                button { class: "nav-item", "[Transactions]" }
                button { class: "nav-item", "[Programs]" }
                button { class: "nav-item", "[Settings]" }
            }
        }
    }
}

/// Terminal main content component
#[component]
fn TerminalMainContent(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "terminal-main-content",
            div { class: "terminal-content",
                h2 { "Dashboard" }
                p { "Welcome to SurfDesk Terminal Interface" }

                div { class: "terminal-stats",
                    div { "Projects: 0" }
                    div { "Environments: 0" }
                    div { "Accounts: 0" }
                    div { "Transactions: 0" }
                }
            }
        }
    }
}

/// Terminal prompt component
#[component]
fn TerminalPrompt() -> Element {
    rsx! {
        div { class: "terminal-prompt",
            span { class: "prompt-symbol", "$" }
            input {
                class: "prompt-input",
                placeholder: "Enter command...",
                type: "text"
            }
        }
    }
}

/// Status bar component
#[component]
fn StatusBar(state: Signal<AppState>) -> Element {
    let connection_status = state.read().connection_status();
    let current_network = state.read().current_network();
    let is_loading = state.read().is_loading();

    rsx! {
        div { class: "status-bar",
            div { class: "status-left",
                span { class: "status-connection {state.read().connection_status.read().css_class()}",
                       "{connection_status}" }
                span { class: "status-network", "{current_network}" }
            }

            div { class: "status-center",
                if is_loading {
                    span { class: "status-loading", "Loading..." }
                }
            }

            div { class: "status-right",
                span { class: "status-memory", "Memory: {get_memory_usage()}MB" }
            }
        }
    }
}

/// Project list component
#[component]
fn ProjectList(state: Signal<AppState>) -> Element {
    let projects = state.read().projects.read();

    rsx! {
        div { class: "project-list",
            for project in projects.iter() {
                div {
                    class: "project-item",
                    key: "{project.id}",
                    onclick: move |_| {
                        // Handle project selection
                    },

                    div { class: "project-name", "{project.name}" }
                    div { class: "project-description",
                          "{project.description.as_deref().unwrap_or(\"No description\")}" }
                }
            }

            if projects.is_empty() {
                div { class: "empty-state", "No projects found" }
            }
        }
    }
}

/// Environment list component
#[component]
fn EnvironmentList(state: Signal<AppState>) -> Element {
    let environments = state.read().environments.read();

    rsx! {
        div { class: "environment-list",
            for environment in environments.iter() {
                div {
                    class: "environment-item",
                    key: "{environment.id}",

                    div { class: "environment-name", "{environment.name}" }
                    div { class: "environment-network", "{environment.network.display_name()}" }
                }
            }

            if environments.is_empty() {
                div { class: "empty-state", "No environments found" }
            }
        }
    }
}

/// Account explorer component
#[component]
fn AccountExplorer(state: Signal<AppState>) -> Element {
    let accounts = state.read().accounts.read();

    rsx! {
        div { class: "account-explorer",
            for account in accounts.iter() {
                div {
                    class: "account-item",
                    key: "{account.id}",

                    div { class: "account-pubkey", "{account.pubkey.to_string()}" }
                    div { class: "account-balance", "{account.balance} SOL" }
                }
            }

            if accounts.is_empty() {
                div { class: "empty-state", "No accounts found" }
            }
        }
    }
}

/// Program list component
#[component]
fn ProgramList(state: Signal<AppState>) -> Element {
    rsx! {
        div { class: "program-list",
            div { class: "empty-state", "No programs deployed" }
        }
    }
}

/// Dashboard component
#[component]
fn Dashboard(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "dashboard",
            h2 { "Dashboard" }

            div { class: "dashboard-stats",
                div { class: "stat-card",
                    h3 { "Projects" }
                    div { class: "stat-value", "{state.read().projects.read().len()}" }
                }

                div { class: "stat-card",
                    h3 { "Environments" }
                    div { class: "stat-value", "{state.read().environments.read().len()}" }
                }

                div { class: "stat-card",
                    h3 { "Accounts" }
                    div { class: "stat-value", "{state.read().accounts.read().len()}" }
                }

                div { class: "stat-card",
                    h3 { "Transactions" }
                    div { class: "stat-value", "{state.read().transactions.read().len()}" }
                }
            }

            div { class: "dashboard-content",
                h3 { "Recent Activity" }
                div { class: "activity-list",
                    div { class: "activity-item", "No recent activity" }
                }
            }
        }
    }
}

/// Main content component
#[component]
fn MainContent(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    let current_view = ui_state.read().main_content.current_view;

    rsx! {
        div { class: "main-content",
            match current_view {
                crate::types::ContentView::Dashboard => rsx! {
                    Dashboard {
                        state: state,
                        ui_state: ui_state
                    }
                },
                crate::types::ContentView::AccountExplorer => rsx! {
                    AccountExplorerView {
                        state: state,
                        ui_state: ui_state
                    }
                },
                crate::types::ContentView::TransactionBuilder => rsx! {
                    TransactionBuilderView {
                        state: state,
                        ui_state: ui_state
                    }
                },
                crate::types::ContentView::ProgramManager => rsx! {
                    ProgramManagerView {
                        state: state,
                        ui_state: ui_state
                    }
                },
                crate::types::ContentView::Settings => rsx! {
                    SettingsView {
                        state: state,
                        ui_state: ui_state
                    }
                }
            }
        }
    }
}

/// Account explorer view component
#[component]
fn AccountExplorerView(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "account-explorer-view",
            h2 { "Account Explorer" }
            AccountExplorer {
                state: state
            }
        }
    }
}

/// Transaction builder view component
#[component]
fn TransactionBuilderView(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "transaction-builder-view",
            h2 { "Transaction Builder" }
            p { "Transaction builder functionality coming soon..." }
        }
    }
}

/// Program manager view component
#[component]
fn ProgramManagerView(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "program-manager-view",
            h2 { "Program Manager" }
            p { "Program management functionality coming soon..." }
        }
    }
}

/// Settings view component
#[component]
fn SettingsView(state: Signal<AppState>, ui_state: Signal<UIState>) -> Element {
    rsx! {
        div { class: "settings-view",
            h2 { "Settings" }
            p { "Settings functionality coming soon..." }
        }
    }
}

/// Toggle sidebar function
fn toggle_sidebar() {
    // This would be handled by state management
    log::debug!("Toggle sidebar requested");
}

/// Get toggle icon based on sidebar state
fn toggle_icon() -> &'static str {
    "☰"
}

/// Get sidebar CSS class
fn sidebar_class() -> &'static str {
    "sidebar"
}

/// Get sidebar visibility CSS class
fn sidebar_visibility_class(expanded: bool) -> &'static str {
    if expanded {
        "sidebar-expanded"
    } else {
        "sidebar-collapsed"
    }
}

/// Get memory usage (mock implementation)
fn get_memory_usage() -> u64 {
    // This would use actual memory monitoring
    42
}

/// Initialize the application
pub async fn init_app() -> Result<()> {
    log::info!("Initializing SurfDesk application");

    // Initialize core services
    crate::init_core().await?;

    // Initialize platform-specific features
    crate::platform::PlatformAdapter::initialize().await?;

    log::info!("SurfDesk application initialized successfully");
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
    fn test_toggle_icon() {
        assert_eq!(toggle_icon(), "☰");
    }

    #[test]
    fn test_sidebar_visibility_class() {
        assert_eq!(sidebar_visibility_class(true), "sidebar-expanded");
        assert_eq!(sidebar_visibility_class(false), "sidebar-collapsed");
    }

    #[test]
    fn test_get_memory_usage() {
        let usage = get_memory_usage();
        assert!(usage > 0);
    }

    #[tokio::test]
    async fn test_init_app() {
        let result = init_app().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_app() {
        run_app();
        // This is a basic test that the function doesn't panic
    }
}
