#![allow(dead_code)]
//! # Status Bar Component Module
//!
//! This module provides a status bar component for displaying application
//! status, connection information, and system indicators in the SurfDesk application.

use super::combine_classes;
use crate::{state::AppState, types::UIState};
use dioxus::prelude::*;

/// Status bar component properties
#[derive(Props, Clone, PartialEq)]
pub struct StatusBarProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Application state
    pub state: dioxus::prelude::Signal<AppState>,
    /// UI state
    pub ui_state: dioxus::prelude::Signal<UIState>,
    /// Show connection status
    #[props(default)]
    pub show_connection: bool,
    /// Show network information
    #[props(default)]
    pub show_network: bool,
    /// Show account count
    #[props(default)]
    pub show_accounts: bool,
}

/// Status bar component
#[component]
pub fn StatusBar(props: StatusBarProps) -> Element {
    let mut classes = vec!["status-bar"];

    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    // Get current status information (simplified for now)
    let connection_status = "Connected".to_string();
    let active_accounts = 0;
    let current_network = Some("Devnet".to_string());

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,

            // Left section - Connection status
            div { class: "status-bar-left",
                if props.show_connection {
                    div { class: "status-item",
                        span { class: "status-label", "Connection:" }
                        span { class: "status-value", "{connection_status}" }
                    }
                }

                if props.show_network {
                    if let Some(network) = current_network {
                        div { class: "status-item",
                            span { class: "status-label", "Network:" }
                            span { class: "status-value", "{network}" }
                        }
                    }
                }
            }

            // Center section - Status messages
            div { class: "status-bar-center",
                div { class: "status-message",
                    "Ready"
                }
            }

            // Right section - Account information
            div { class: "status-bar-right",
                if props.show_accounts {
                    div { class: "status-item",
                        span { class: "status-label", "Accounts:" }
                        span { class: "status-value", "{active_accounts}" }
                    }
                }

                // Loading indicator
                if props.state.read().is_loading() {
                    div { class: "status-item loading",
                        span { class: "loading-spinner" }
                        span { class: "status-label", "Loading..." }
                    }
                }
            }
        }
    }
}

/// Simple status bar with default settings
#[component]
pub fn SimpleStatusBar() -> Element {
    let state = use_signal(AppState::default);
    let ui_state = use_signal(UIState::default);

    rsx! {
        StatusBar {
            state: state,
            ui_state: ui_state,
            show_connection: true,
            show_network: true,
            show_accounts: true,
        }
    }
}

/// Compact status bar for mobile views
#[component]
pub fn CompactStatusBar(props: StatusBarProps) -> Element {
    let mut classes = vec!["status-bar", "compact"];

    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,

            // Only show essential information
            div { class: "status-bar-compact",
                span { class: "status-connection",
                    "Connected"
                }
                span { class: "status-separator", "•" }
                span { class: "status-accounts",
                    "0 accounts"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        // Test that status bar can be created with default props
        let props = StatusBarProps {
            class: Some("test-class".to_string()),
            id: Some("test-status-bar".to_string()),
            state: use_signal(AppState::default),
            ui_state: use_signal(UIState::default),
            show_connection: true,
            show_network: true,
            show_accounts: true,
        };

        // Verify props are set correctly
        assert_eq!(props.class, Some("test-class".to_string()));
        assert_eq!(props.id, Some("test-status-bar".to_string()));
        assert!(props.show_connection);
        assert!(props.show_network);
        assert!(props.show_accounts);
    }

    #[test]
    fn test_status_bar_classes() {
        let mut classes = vec!["status-bar"];
        classes.push("custom-class");

        let combined = super::combine_classes(&classes);
        assert!(combined.contains("status-bar"));
        assert!(combined.contains("custom-class"));
    }
}
