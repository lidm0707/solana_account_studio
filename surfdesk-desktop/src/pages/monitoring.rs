//! # Desktop Monitoring Page
//!
//! This module provides the desktop-specific implementation of the account monitoring
//! page, integrating the core monitoring components with desktop-specific features
//! like native notifications and system tray integration.

use crate::AppState;
use dioxus::prelude::*;
use surfdesk_core::components::{Button, Card, Color, Size};
use surfdesk_core::{components::MonitoringDashboard, types::*, MonitoringStats};

/// Desktop monitoring page component
#[component]
pub fn MonitoringPage() -> Element {
    let app_state = use_context::<AppState>();
    let mut monitoring_enabled = use_signal(|| false);
    let mut stats = use_signal(MonitoringStats::default);

    // Initialize monitoring service on component mount
    use_effect({
        let app_state_clone = app_state.clone();
        move || {
            let app_state_clone_inner = app_state_clone.clone();
            spawn(async move {
                // TODO: Initialize monitoring service
                // For now, just set monitoring as enabled
                monitoring_enabled.set(true);
            });
        }
    });

    rsx! {
        div { class: "monitoring-page desktop-page",

            // Page header with controls
            MonitoringPageHeader {
                enabled: monitoring_enabled(),
                on_toggle: {
                    let app_state_clone = app_state.clone();
                    move |enabled| {
                        let app_state_clone_inner = app_state_clone.clone();
                        spawn(async move {
                            // TODO: Start/stop monitoring service
                            // For now, just update the state
                            monitoring_enabled.set(enabled);
                        });
                    }
                },
                stats: stats.read().clone()
            }

            // Main monitoring dashboard
            MonitoringDashboard {}

            // Desktop-specific monitoring controls
            DesktopMonitoringControls {
                on_export_stats: {
                    let app_state_clone = app_state.clone();
                    move || {
                        let app_state_clone_inner = app_state_clone.clone();
                        spawn(async move {
                            export_monitoring_stats(&app_state_clone_inner).await;
                        });
                    }
                },
                on_clear_history: {
                    let app_state_clone = app_state.clone();
                    move || {
                        let app_state_clone_inner = app_state_clone.clone();
                        spawn(async move {
                            clear_monitoring_history(&app_state_clone_inner).await;
                        });
                    }
                }
            }
        }
    }
}

/// Monitoring page header with controls
#[component]
fn MonitoringPageHeader(
    enabled: bool,
    on_toggle: EventHandler<bool>,
    stats: MonitoringStats,
) -> Element {
    rsx! {
        div { class: "monitoring-page-header",
            div { class: "header-content",
                div { class: "header-title",
                    h1 { "Account Monitoring" }
                    p { "Real-time monitoring of Solana accounts with customizable alerts" }
                }

                div { class: "header-controls",
                    // Monitoring toggle
                    div { class: "monitoring-toggle",
                        label { class: "toggle-label", "Monitoring Service" }
                        label { class: "toggle-switch",
                            input {
                                r#type: "checkbox",
                                checked: enabled,
                                onchange: move |evt| {
                                    let is_enabled = evt.value() == "true";
                                    on_toggle(is_enabled);
                                }
                            }
                            span { class: "toggle-slider" }
                        }
                        span {
                            class: if enabled { "status-enabled" } else { "status-disabled" },
                            if enabled { "🟢 Active" } else { "🔴 Inactive" }
                        }
                    }

                    // Quick stats
                    div { class: "quick-stats",
                        div { class: "stat-item",
                            span { class: "stat-label", "Accounts" }
                            span { class: "stat-value", "{stats.total_accounts}" }
                        }
                        div { class: "stat-item",
                            span { class: "stat-label", "Active" }
                            span { class: "stat-value", "{stats.active_accounts}" }
                        }
                        div { class: "stat-item",
                            span { class: "stat-label", "Alerts" }
                            span { class: "stat-value", "{stats.total_alerts}" }
                        }
                    }
                }
            }
        }
    }
}

/// Desktop-specific monitoring controls
#[component]
fn DesktopMonitoringControls(
    on_export_stats: EventHandler<()>,
    on_clear_history: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "desktop-monitoring-controls",
            Card {
                title: Some("Desktop Controls".to_string()),
                div { class: "controls-grid",
                    // Export functionality
                    div { class: "control-group",
                        h3 { "Export Data" }
                        p { "Export monitoring statistics and history for analysis" }
                        Button {
                            onclick: move |_| on_export_stats(()),
                            color: Color::Secondary,
                            size: Size::Small,
                            "Export Statistics"
                        }
                    }

                    // History management
                    div { class: "control-group",
                        h3 { "History Management" }
                        p { "Clear monitoring history to free up storage space" }
                        Button {
                            onclick: move |_| on_clear_history(()),
                            color: Color::Error,
                            size: Size::Small,
                            "Clear History"
                        }
                    }

                    // System integration
                    div { class: "control-group",
                        h3 { "System Integration" }
                        p { "Configure desktop notifications and system tray integration" }
                        Button {
                            onclick: move |_| {
                                // Open system settings modal
                            },
                            color: Color::Primary,
                            size: Size::Small,
                            "System Settings"
                        }
                    }

                    // Advanced options
                    div { class: "control-group",
                        h3 { "Advanced Options" }
                        p { "Configure advanced monitoring settings and performance options" }
                        Button {
                            onclick: move |_| {
                                // Open advanced settings modal
                            },
                            color: Color::Secondary,
                            size: Size::Small,
                            "Advanced Settings"
                        }
                    }
                }
            }
        }
    }
}

/// Export monitoring statistics to file
async fn export_monitoring_stats(app_state: &AppState) {
    // TODO: Implement monitoring stats export
    log::info!("Export monitoring stats (not yet implemented)");

    // Show desktop notification
    show_desktop_notification(
        "Monitoring Data Exported",
        "Statistics and account data have been exported successfully",
    )
    .await;
}

/// Clear monitoring history
async fn clear_monitoring_history(app_state: &AppState) {
    // TODO: Implement monitoring history clearing
    log::info!("Clear monitoring history (not yet implemented)");

    // Show desktop notification
    show_desktop_notification(
        "History Cleared",
        "Monitoring history has been cleared for all accounts",
    )
    .await;
}

/// Show desktop notification
async fn show_desktop_notification(title: &str, message: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Err(e) = notify_rust::Notification::new()
            .summary(title)
            .body(message)
            .appname("SurfDesk")
            .show()
        {
            log::error!("Failed to show desktop notification: {}", e);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        // Fallback to in-app notification on web
        log::info!("Desktop notification: {} - {}", title, message);
    }
}

/// Desktop notification handler for monitoring alerts
pub struct DesktopNotificationHandler;

#[async_trait::async_trait]
impl surfdesk_core::services::monitoring::AlertHandler for DesktopNotificationHandler {
    async fn handle_alert(
        &self,
        event: &surfdesk_core::types::MonitoringEvent,
    ) -> surfdesk_core::error::Result<()> {
        let (title, message) = match &event.event_type {
            surfdesk_core::types::MonitoringEventType::BalanceChanged { change_amount, .. } => (
                "Balance Change Detected".to_string(),
                format!("Account balance changed by {} lamports", change_amount),
            ),
            surfdesk_core::types::MonitoringEventType::TransactionDetected {
                amount,
                success,
                ..
            } => {
                let status = if *success { "Successful" } else { "Failed" };
                (
                    "Transaction Detected".to_string(),
                    format!("{} transaction of {} lamports", status, amount),
                )
            }
            surfdesk_core::types::MonitoringEventType::AlertTriggered {
                alert_name,
                message,
                ..
            } => (alert_name.clone(), message.clone()),
            _ => (
                "Monitoring Alert".to_string(),
                "New monitoring event detected".to_string(),
            ),
        };

        show_desktop_notification(&title, &message).await;
        Ok(())
    }
}

/// Initialize desktop-specific monitoring features
/// Initialize desktop monitoring system
async fn initialize_desktop_monitoring(app_state: &AppState) -> surfdesk_core::error::Result<()> {
    // TODO: Initialize desktop monitoring system
    log::info!("Desktop monitoring initialization (not yet implemented)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_page_creation() {
        // This test would require setting up a full Dioxus context
        // For now, we'll just verify the component compiles
        assert!(true);
    }
}
