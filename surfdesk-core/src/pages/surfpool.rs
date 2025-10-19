//! Surfpool Manager Page Component
//!
//! This page provides the interface for managing the Surfpool local Solana
//! simulation network, including starting/stopping the service, monitoring
//! status, and configuring network parameters.

use dioxus::prelude::*;

/// Surfpool Manager page component
#[component]
pub fn SurfpoolManager() -> Element {
    let is_running = use_signal(|| false);
    let status_message = use_signal(|| "Surfpool is stopped".to_string());
    let port = use_signal(|| 8999u16);
    let network_status = use_signal(|| "Disconnected".to_string());
    let logs = use_signal(|| Vec::<String>::new());

    rsx! {
        div { class: "surfpool-manager-page",
            div { class: "page-header",
                h1 { "Surfpool Manager" }
                p { "Manage your local Solana simulation network" }
            }

            // Status Section
            div { class: "status-section",
                StatusCard {
                    is_running: is_running(),
                    status_message: status_message(),
                    port: port(),
                    network_status: network_status()
                }
            }

            // Control Panel
            div { class: "control-panel",
                h2 { "Control Panel" }
                div { class: "control-buttons",
                    if !is_running() {
                        button {
                            class: "btn btn-primary btn-large",
                            onclick: move |_| {
                                is_running.set(true);
                                status_message.set("Starting Surfpool...");
                                logs.write().push("Surfpool start initiated".to_string());
                                // TODO: Implement actual surfpool start logic
                                spawn(async move {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                    status_message.set("Surfpool is running");
                                    network_status.set("Connected - Local Simulation");
                                    logs.write().push("Surfpool successfully started on port 8999".to_string());
                                });
                            },
                            "🚀 Start Surfpool"
                        }
                    } else {
                        button {
                            class: "btn btn-danger btn-large",
                            onclick: move |_| {
                                is_running.set(false);
                                status_message.set("Stopping Surfpool...");
                                logs.write().push("Surfpool stop initiated".to_string());
                                // TODO: Implement actual surfpool stop logic
                                spawn(async move {
                                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                                    status_message.set("Surfpool is stopped");
                                    network_status.set("Disconnected");
                                    logs.write().push("Surfpool successfully stopped".to_string());
                                });
                            },
                            "🛑 Stop Surfpool"
                        }
                    }

                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            logs.write().push("Network status refreshed".to_string());
                            // TODO: Implement network status refresh
                        },
                        "🔄 Refresh Status"
                    }

                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            logs.write().push("Configuration opened".to_string());
                            // TODO: Open configuration dialog
                        },
                        "⚙️ Configure"
                    }
                }
            }

            // Network Information
            div { class: "network-info-section",
                h2 { "Network Information" }
                div { class: "info-grid",
                    InfoCard {
                        label: "Network Type",
                        value: "Local Simulation (Mainnet Fork)"
                    }
                    InfoCard {
                        label: "Port",
                        value: "{port()}"
                    }
                    InfoCard {
                        label: "RPC Endpoint",
                        value: "http://localhost:8999"
                    }
                    InfoCard {
                        label: "WebSocket",
                        value: "ws://localhost:8999"
                    }
                }
            }

            // Configuration Section
            div { class: "config-section",
                h2 { "Configuration" }
                div { class: "config-form",
                    div { class: "form-group",
                        label { "RPC Port" }
                        input {
                            r#type: "number",
                            value: "{port()}",
                            class: "form-control",
                            onchange: move |evt| {
                                if let Ok(new_port) = evt.value().parse::<u16>() {
                                    port.set(new_port);
                                }
                            }
                        }
                    }

                    div { class: "form-group",
                        label { "Auto-start on Launch" }
                        input {
                            r#type: "checkbox",
                            class: "form-checkbox"
                        }
                    }

                    div { class: "form-group",
                        label { "Log Level" }
                        select {
                            class: "form-control",
                            option { "Info" }
                            option { "Debug" }
                            option { "Error" }
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            logs.write().push("Configuration saved".to_string());
                            // TODO: Save configuration
                        },
                        "💾 Save Configuration"
                    }
                }
            }

            // Logs Section
            div { class: "logs-section",
                div { class: "logs-header",
                    h2 { "Activity Logs" }
                    button {
                        class: "btn btn-secondary btn-small",
                        onclick: move |_| {
                            logs.write().clear();
                        },
                        "🗑️ Clear Logs"
                    }
                }

                div { class: "logs-container",
                    if logs().is_empty() {
                        div { class: "logs-empty",
                            p { "No activity logs yet. Start Surfpool to see logs." }
                        }
                    } else {
                        for (index, log_entry) in logs().iter().enumerate() {
                            div {
                                key: "{index}",
                                class: "log-entry",
                                span { class: "log-timestamp",
                                    "[{chrono::Utc::now().format(\"%H:%M:%S\")}]"
                                }
                                span { class: "log-message", "{log_entry}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Status card component for displaying surfpool status
#[component]
fn StatusCard(
    is_running: bool,
    status_message: String,
    port: u16,
    network_status: String,
) -> Element {
    rsx! {
        div { class: "surfpool-status-card",
            div { class: "status-header",
                div {
                    class: "status-indicator {if is_running { 'status-running' } else { 'status-stopped' }}",
                    div { class: "status-dot" }
                    span { class: "status-text", "{status_message}" }
                }
            }

            div { class: "status-details",
                div { class: "status-item",
                    span { class: "status-label", "Port:" }
                    span { class: "status-value", "{port}" }
                }
                div { class: "status-item",
                    span { class: "status-label", "Network:" }
                    span { class: "status-value", "{network_status}" }
                }
            }

            div { class: "status-features",
                h3 { "Features" }
                ul {
                    li { "Mainnet fork to local simulation" }
                    li { "Full RPC compatibility" }
                    li { "WebSocket support" }
                    li { "Account state persistence" }
                    li { "Transaction history" }
                }
            }
        }
    }
}

/// Information card component
#[component]
fn InfoCard(label: String, value: String) -> Element {
    rsx! {
        div { class: "info-card",
            div { class: "info-label", "{label}" }
            div { class: "info-value", "{value}" }
        }
    }
}
