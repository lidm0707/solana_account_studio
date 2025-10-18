//! # Account Monitoring Components
//!
//! This module provides UI components for the account monitoring feature,
//! including monitoring controls, alert configuration, and real-time status displays.

use crate::components::{Button, Color, Size};
use crate::state::AppState;
use crate::types::*;
use dioxus::prelude::*;

/// Account monitoring dashboard component
#[component]
pub fn MonitoringDashboard() -> Element {
    let app_state = use_context::<AppState>();
    let monitoring_stats = app_state.monitoring_stats();

    rsx! {
        div { class: "monitoring-dashboard",
            // Header
            div { class: "monitoring-header",
                h2 { "Account Monitoring" }
                Button {
                    onclick: move |_| {
                        // TODO: Implement add account functionality
                    },
                    color: Color::Primary,
                    size: Size::Small,
                    "Add Account to Monitor"
                }
            }

            // Statistics Overview
            MonitoringStatsOverview { stats: monitoring_stats }

            // Monitored Accounts List
            MonitoredAccountsList {
                on_refresh: move |_| {
                    spawn(async move {
                        // TODO: Implement refresh functionality
                    });
                }
            }
        }
    }
}

/// Monitoring statistics overview component
#[component]
fn MonitoringStatsOverview(stats: MonitoringStats) -> Element {
    rsx! {
        div { class: "monitoring-stats",
            div { class: "stats-grid",
                div { class: "stat-card",
                    div { class: "stat-value", "{stats.total_accounts}" }
                    div { class: "stat-label", "Total Accounts" }
                }
                div { class: "stat-card",
                    div { class: "stat-value", "{stats.active_accounts}" }
                    div { class: "stat-label", "Active Monitoring" }
                }
                div { class: "stat-card",
                    div { class: "stat-value", "{stats.total_checks}" }
                    div { class: "stat-label", "Total Checks" }
                }
                div { class: "stat-card",
                    div { class: "stat-value", "{stats.total_alerts}" }
                    div { class: "stat-label", "Alerts Triggered" }
                }
            }

            if let Some(_last_check) = stats.last_check {
                div { class: "last-check",
                    span { "Last check: Available" }
                }
            }
        }
    }
}

/// Monitored accounts list component
#[component]
fn MonitoredAccountsList(on_refresh: EventHandler<()>) -> Element {
    let accounts = use_signal(Vec::<(SolanaPubkey, AccountMonitoring)>::new);
    let loading = use_signal(|| false);

    rsx! {
        div { class: "monitored-accounts",
            div { class: "accounts-header",
                h3 { "Monitored Accounts" }
                Button {
                    onclick: move |_| on_refresh(()),
                    color: Color::Secondary,
                    size: Size::Small,
                    "Refresh"
                }
            }

            if loading() {
                div { class: "loading", "Loading monitored accounts..." }
            } else if accounts().is_empty() {
                div { class: "empty-state",
                    p { "No accounts are currently being monitored." }
                    p { "Add an account to start monitoring its activity." }
                }
            } else {
                div { class: "accounts-list",
                    for (pubkey, monitoring) in accounts() {
                        MonitoredAccountItem {
                            pubkey: pubkey.clone(),
                            monitoring: monitoring.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Individual monitored account item
#[component]
fn MonitoredAccountItem(pubkey: SolanaPubkey, monitoring: AccountMonitoring) -> Element {
    let mut show_details = use_signal(|| false);
    let mut show_alerts = use_signal(|| false);

    rsx! {
        div { class: "account-item",
            div { class: "account-header",
                div { class: "account-info",
                    span { class: "account-pubkey", "{pubkey}" }
                    span { class: "monitoring-status",
                        class: if monitoring.enabled { "enabled" } else { "disabled" },
                        if monitoring.enabled { "Monitoring Active" } else { "Monitoring Paused" }
                    }
                }
                div { class: "account-actions",
                    Button {
                        onclick: move |_| show_details.set(!show_details()),
                        color: Color::Secondary,
                        size: Size::Small,
                        "Details"
                    }
                    Button {
                        onclick: move |_| show_alerts.set(!show_alerts()),
                        color: Color::Secondary,
                        size: Size::Small,
                        "Alerts ({monitoring.alerts.len()})"
                    }
                }
            }

            if show_details() {
                AccountMonitoringDetails {
                    monitoring: monitoring.clone(),
                }
            }

            if show_alerts() {
                AccountAlertsList {
                    alerts: monitoring.alerts,
                }
            }
        }
    }
}

/// Account monitoring details component
#[component]
fn AccountMonitoringDetails(monitoring: AccountMonitoring) -> Element {
    rsx! {
        div { class: "monitoring-details",
            div { class: "detail-row",
                span { "Status: " }
                span {
                    class: if monitoring.enabled { "status-enabled" } else { "status-disabled" },
                    if monitoring.enabled { "Enabled" } else { "Disabled" }
                }
            }
            div { class: "detail-row",
                span { "Check Interval: " }
                span { "{monitoring.interval_seconds} seconds" }
            }
            div { class: "detail-row",
                span { "Total Checks: " }
                span { "{monitoring.check_count}" }
            }
            if let Some(_last_checked) = monitoring.last_checked {
                div { class: "detail-row",
                    span { "Last Checked: " }
                    span { "Available" }
                }
            }
            div { class: "detail-row",
                span { "Event History: " }
                span { "{monitoring.history.len()} events" }
            }
        }
    }
}

/// Account alerts list component
#[component]
fn AccountAlertsList(alerts: Vec<AlertConfig>) -> Element {
    rsx! {
        div { class: "alerts-section",
            div { class: "alerts-header",
                h4 { "Alert Configurations" }
                Button {
                    onclick: move |_| {
                        // TODO: Implement add alert functionality
                    },
                    color: Color::Primary,
                    size: Size::Small,
                    "Add Alert"
                }
            }

            if alerts.is_empty() {
                div { class: "empty-alerts",
                    p { "No alerts configured for this account." }
                }
            } else {
                div { class: "alerts-list",
                    for alert in alerts {
                        AlertItem {
                            alert: alert.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Individual alert item component
#[component]
fn AlertItem(alert: AlertConfig) -> Element {
    let mut active = use_signal(|| alert.active);

    rsx! {
        div { class: "alert-item",
            div { class: "alert-header",
                div { class: "alert-info",
                    span { class: "alert-name", "{alert.name}" }
                    span { class: "alert-type", "{alert.alert_type.display_name()}" }
                }
                div { class: "alert-controls",
                    label { class: "toggle-switch",
                        input {
                            r#type: "checkbox",
                            checked: active(),
                            onchange: move |evt| {
                                let is_active = evt.value() == "true";
                                active.set(is_active);
                                // TODO: Update alert in service
                            }
                        }
                        span { class: "toggle-slider" }
                    }
                    Button {
                        onclick: move |_| {
                            // TODO: Remove alert functionality
                        },
                        color: Color::Error,
                        size: Size::Small,
                        "Remove"
                    }
                }
            }
            div { class: "alert-details",
                p { "Cooldown: {alert.cooldown_seconds} seconds" }
                if let Some(_last_triggered) = alert.last_triggered {
                    p { "Last triggered: Available" }
                }
                p { "Trigger count: {alert.trigger_count}" }
            }
        }
    }
}
