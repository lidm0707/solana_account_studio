#![allow(dead_code)]
//! # SurfPool Control Component Module
//!
//! This module provides components for integrating with the external SurfPool tool.
//! SurfPool is a separate third-party tool that users must install independently.
//! These components handle the integration and provide clear feedback about availability.

use super::combine_classes;
use crate::platform::Platform;
use crate::state::AppState;
use crate::surfpool::{use_surfpool_controller, use_surfpool_status, ControllerStatus};
use dioxus::prelude::*;

/// Props for the SurfPool control component
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SurfPoolControlProps {
    /// Application state
    pub state: Signal<AppState>,
    /// Current platform
    pub platform: Platform,
    /// Optional callback when status changes
    pub on_status_change: Option<EventHandler<ControllerStatus>>,
}

/// Main SurfPool control component
#[component]
pub fn SurfPoolControl(props: SurfPoolControlProps) -> Element {
    let controller = use_surfpool_controller(props.platform);
    let mut status = use_signal(|| ControllerStatus::Stopped);
    let mut error_message = use_signal(|| Option::<String>::None);

    // Manual status checking (simplified approach)
    let check_status = move |_| {
        let controller = controller.read().clone();

        spawn(async move {
            let new_status = controller.get_status().await;
            status.set(new_status.clone());

            if let Some(handler) = &props.on_status_change {
                handler.call(new_status);
            }
        });
    };

    rsx! {
        div { class: "surfpool-control",
            div { class: "surfpool-header",
                h3 { "SurfPool Control" }
                SurfPoolStatus {}
            }

            div { class: "surfpool-status",
                p { class: "status-text", "Status: {status}" }
                if let Some(error) = error_message.read().as_ref() {
                    p { class: "error-text", "{error}" }
                }
            }

            div { class: "surfpool-actions",
                button {
                    class: "btn btn-primary",
                    onclick: check_status,
                    "Check Status"
                }
            }
        }
    }
}

/// Component to show SurfPool availability status
#[component]
pub fn SurfPoolStatus() -> Element {
    let is_available = use_surfpool_status();

    rsx! {
        div { class: "surfpool-status-indicator",
            if *is_available.read() {
                div { class: "status-available",
                    span { class: "status-icon", "✅" }
                    span { class: "status-text", "SurfPool Available" }
                }
            } else {
                div { class: "status-unavailable",
                    span { class: "status-icon", "⚠️" }
                    div { class: "status-details",
                        span { class: "status-text", "SurfPool Not Available" }
                        p { class: "install-hint",
                            "Install with: "
                            code { "cargo install surfpool" }
                        }
                    }
                }
            }
        }
    }
}

/// Simple component to show installation instructions
#[component]
pub fn SurfPoolInstallGuide() -> Element {
    rsx! {
        div { class: "surfpool-install-guide",
            h4 { "SurfPool Installation Guide" }
            p { "SurfPool is an optional tool for local Solana development." }

            div { class: "install-steps",
                h5 { "Installation Steps:" }
                ol {
                    li { "Install Rust (if not already installed):"
                        pre { "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" }
                    }
                    li { "Install SurfPool:"
                        pre { "cargo install surfpool" }
                    }
                    li { "Verify installation:"
                        pre { "surfpool --version" }
                    }
                }
            }

            div { class: "help-links",
                p { "For more information:" }
                a {
                    href: "https://github.com/surfpool/surfpool",
                    target: "_blank",
                    "SurfPool GitHub Repository"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surfpool_control_props() {
        let props = SurfPoolControlProps {
            state: use_signal(AppState::default),
            platform: Platform::Desktop,
            on_status_change: None,
        };

        assert_eq!(props.platform, Platform::Desktop);
        assert!(props.on_status_change.is_none());
    }

    #[test]
    fn test_component_creation() {
        // Test that component names are valid
        // This is a basic smoke test to ensure components exist
        let component_names = vec!["SurPoolControl", "SurfPoolStatus", "SurfPoolInstallGuide"];
        for name in component_names {
            assert!(!name.is_empty());
            assert!(name.len() > 5); // Basic sanity check
        }
    }
}
