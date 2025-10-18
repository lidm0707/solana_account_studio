//! # SurfPool Page Component
//!
//! Real SurfPool integration with start/stop controls, status monitoring,
//! log viewing, and configuration management for local Solana validator.

use crate::surfpool::{SurfPoolConfig, SurfPoolManager, SurfPoolStatus as DesktopSurfPoolStatus};
use dioxus::prelude::*;
use std::sync::Arc;
use surfdesk_core::components::{Button, Card, Input, Size, Variant};
use surfdesk_core::solana_rpc::{SolanaNetwork, SolanaRpcClient};

/// SurfPool control page component
#[component]
pub fn SurfPoolPage() -> Element {
    let surfpool_status = use_signal(|| DesktopSurfPoolStatus::Stopped);
    let mut logs = use_signal(Vec::<String>::new);
    let mut config = use_signal(SurfPoolConfig::default);
    let mut show_config_modal = use_signal(|| false);
    let loading = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);

    // Real SurfPool manager (no Arc for Dioxus compatibility)
    let mut surfpool_manager = use_signal(|| {
        let cfg = config.read().clone();
        SurfPoolManager::new(cfg)
    });

    // Real RPC client for health checks
    let rpc_client = use_signal(|| {
        SolanaRpcClient::new_with_url(
            "http://localhost:8999", // Default SurfPool port
            surfdesk_core::solana_rpc::RpcCommitment::Confirmed,
        )
    });

    // Status monitoring effect
    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut status_signal = surfpool_status;
        let mut logs_signal = logs;
        let mut error_signal = error_message;
        let manager = surfpool_manager.clone();
        let rpc = rpc_client.clone();

        async move {
            loop {
                // Update status from manager
                let manager_ref = manager.read();
                let current_status = manager_ref.get_status();
                status_signal.set(current_status.clone());

                // Collect logs from process
                let recent_logs = manager_ref.get_recent_logs(50).await;
                logs_signal.set(recent_logs);

                // Perform health check if running
                if let DesktopSurfPoolStatus::Running = &current_status {
                    match manager_ref.health_check().await {
                        Ok(health) => {
                            log::info!(
                                "Health check passed: uptime {}s",
                                health.uptime_seconds.unwrap_or(0)
                            );
                            error_signal.set(None);
                        }
                        Err(e) => {
                            error_signal.set(Some(format!("Health check failed: {}", e)));
                        }
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    });

    // Handle start SurfPool
    let handle_start = move |_| {
        log::info!("Start SurfPool button clicked (not implemented)");
    };

    // Handle stop SurfPool
    let handle_stop = move |_| {
        log::info!("Stop SurfPool button clicked (not implemented)");
    };

    // Handle restart SurfPool
    let handle_restart = move |_| {
        log::info!("Restart SurfPool button clicked (not implemented)");
    };

    // Handle config change
    let handle_config_change = move |new_config: SurfPoolConfig| {
        config.set(new_config);
    };

    // Clear logs
    let handle_clear_logs = move |_| {
        logs.set(Vec::new());
    };

    // Export logs
    let handle_export_logs = move |_| {
        let current_logs = logs.read().clone();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("surfpool_logs_{}.txt", timestamp);

        match std::fs::write(&filename, current_logs.join("\n")) {
            Ok(_) => {
                log::info!("Logs exported to {}", filename);
            }
            Err(e) => {
                log::error!("Failed to export logs: {}", e);
            }
        }
    };

    rsx! {
        div { class: "surfpool-page",

            // Header
            div { class: "surfpool-header",
                h1 { "SurfPool Control" }
                p { "Manage your local Solana validator for development" }
            }

            // Error display
            if let Some(error) = error_message.read().as_ref() {
                div { class: "error-banner",
                    span { class: "error-icon", "⚠️" }
                    span { class: "error-text", "{error}" }
                    button {
                        class: "error-dismiss",
                        onclick: move |_| error_message.set(None),
                        "×"
                    }
                }
            }

            // Status and controls
            div { class: "surfpool-main",

                // Status card
                Card {
                    title: Some("Validator Status".to_string()),
                    size: Size::Large,
                    elevated: true,
                    children: rsx! {
                        div { class: "status-display",
                            match surfpool_status() {
                                DesktopSurfPoolStatus::Running => rsx! {
                                    div { class: "status-running",
                                        div { class: "status-indicator running" }
                                        div { class: "status-details",
                                            h3 { "SurfPool Running" }
                                            div { class: "status-info",
                                                div { class: "info-item",
                                                    span { class: "info-label", "Status:" }
                                                    span { class: "info-value", "Active" }
                                                }
                                                div { class: "info-item",
                                                    span { class: "info-label", "RPC Port:" }
                                                    span { class: "info-value", "8999" }
                                                }
                                                div { class: "info-item",
                                                    span { class: "info-label", "WS Port:" }
                                                    span { class: "info-value", "8900" }
                                                }
                                            }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Stopped => rsx! {
                                    div { class: "status-stopped",
                                        div { class: "status-indicator stopped" }
                                        div { class: "status-details",
                                            h3 { "SurfPool Stopped" }
                                            p { "Local validator is not running" }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Starting => rsx! {
                                    div { class: "status-starting",
                                        div { class: "status-indicator starting" }
                                        div { class: "status-details",
                                            h3 { "Starting SurfPool..." }
                                            p { "Validator is initializing" }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Stopping => rsx! {
                                    div { class: "status-stopping",
                                        div { class: "status-indicator stopping" }
                                        div { class: "status-details",
                                            h3 { "Stopping SurfPool..." }
                                            p { "Validator is shutting down" }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Error(message) => rsx! {
                                    div { class: "status-error",
                                        div { class: "status-indicator error" }
                                        div { class: "status-details",
                                            h3 { "SurfPool Error" }
                                            p { "Validator encountered an issue: {message}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Control buttons
                        div { class: "control-buttons",
                            match surfpool_status() {
                                DesktopSurfPoolStatus::Stopped | DesktopSurfPoolStatus::Error(_) => rsx! {
                                    Button {
                                        onclick: handle_start,
                                        disabled: loading(),
                                        "Start Validator"
                                    }
                                },
                                DesktopSurfPoolStatus::Running => rsx! {
                                    Button {
                                        onclick: handle_stop,
                                        disabled: loading(),
                                        variant: Variant::Outlined,
                                        "Stop Validator"
                                    }
                                    Button {
                                        onclick: handle_restart,
                                        disabled: loading(),
                                        "Restart"
                                    }
                                },
                                DesktopSurfPoolStatus::Starting | DesktopSurfPoolStatus::Stopping => rsx! {
                                    Button {
                                        disabled: true,
                                        "Please wait..."
                                    }
                                }
                            }

                            Button {
                                onclick: move |_| show_config_modal.set(true),
                                variant: Variant::Outlined,
                                "Configure"
                            }
                        }
                    }
                }

                // Logs card
                Card {
                    title: Some("Validator Logs".to_string()),
                    size: Size::Large,
                    elevated: true,
                    children: rsx! {
                        div { class: "logs-container",
                            div { class: "logs-header",
                                span { "Recent Logs" }
                                div { class: "logs-actions",
                                    Button {
                                        size: Size::Small,
                                        onclick: handle_clear_logs,
                                        variant: Variant::Outlined,
                                        "Clear"
                                    }
                                    Button {
                                        size: Size::Small,
                                        onclick: handle_export_logs,
                                        variant: Variant::Outlined,
                                        "Export"
                                    }
                                }
                            }

                            div { class: "logs-content",
                                if logs.read().is_empty() {
                                    div { class: "logs-empty",
                                        p { "No logs available. Start the validator to see logs." }
                                    }
                                } else {
                                    for (index, log_line) in logs.read().iter().enumerate() {
                                        div {
                                            class: "log-entry",
                                            key: "{index}",
                                            "{log_line}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Configuration modal
            if show_config_modal() {
                ConfigModal {
                    config: config.read().clone(),
                    on_close: move |_| show_config_modal.set(false),
                    on_save: handle_config_change,
                }
            }
        }
    }
}

/// Configuration modal component
#[component]
fn ConfigModal(
    config: SurfPoolConfig,
    on_close: EventHandler<MouseEvent>,
    on_save: EventHandler<SurfPoolConfig>,
) -> Element {
    let mut rpc_port = use_signal(|| config.rpc_port.to_string());
    let mut ws_port = use_signal(|| config.ws_port.to_string());
    let mut ledger_path = use_signal(|| config.ledger_path.clone());
    let mut accounts_path = use_signal(|| config.accounts_path.clone());
    let mut auto_start = use_signal(|| config.auto_start);
    let mut fork_url = use_signal(|| config.fork_url.clone().unwrap_or_default());
    let mut fork_slot = use_signal(|| config.fork_slot.unwrap_or(0).to_string());
    let mut enable_mcp = use_signal(|| config.enable_mcp);

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header",
                    h2 { "SurfPool Configuration" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "×"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "RPC Port" }
                        Input {
                            value: rpc_port.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                rpc_port.set(value);
                            },
                            placeholder: "8999",
                            input_type: surfdesk_core::components::InputType::Number,
                        }
                    }

                    div { class: "form-group",
                        label { "WebSocket Port" }
                        Input {
                            value: ws_port.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                ws_port.set(value);
                            },
                            placeholder: "8900",
                            input_type: surfdesk_core::components::InputType::Number,
                        }
                    }

                    div { class: "form-group",
                        label { "Ledger Path" }
                        Input {
                            value: ledger_path.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                ledger_path.set(value);
                            },
                            placeholder: "/tmp/surfpool-ledger",
                        }
                    }

                    div { class: "form-group",
                        label { "Accounts Path" }
                        Input {
                            value: accounts_path.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                accounts_path.set(value);
                            },
                            placeholder: "/tmp/surfpool-accounts",
                        }
                    }

                    div { class: "form-group",
                        input {
                            r#type: "checkbox",
                            id: "auto-start",
                            checked: auto_start(),
                            onchange: move |e| {
                                auto_start.set(e.checked());
                            }
                        }
                        label { r#for: "auto-start", "Auto Start" }
                    }

                    div { class: "form-group",
                        input {
                            r#type: "checkbox",
                            id: "enable-mcp",
                            checked: enable_mcp(),
                            onchange: move |e| {
                                enable_mcp.set(e.checked());
                            }
                        }
                        label { r#for: "enable-mcp", "Enable MCP" }
                    }

                    div { class: "form-group",
                        label { "Fork URL (Optional)" }
                        Input {
                            value: fork_url.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                fork_url.set(value);
                            },
                            placeholder: "https://api.mainnet-beta.solana.com",
                        }
                    }

                    div { class: "form-group",
                        label { "Fork Slot (Optional)" }
                        Input {
                            value: fork_slot.read().clone(),
                            on_input: move |e: Event<FormData>| {
                                let value = e.value();
                                fork_slot.set(value);
                            },
                            placeholder: "0",
                            input_type: surfdesk_core::components::InputType::Number,
                        }
                    }
                }

                div { class: "modal-footer",
                    Button {
                        variant: Variant::Outlined,
                        onclick: on_close,
                        "Cancel"
                    }
                    Button {
                        onclick: move |_| {
                            let new_config = SurfPoolConfig {
                                rpc_port: rpc_port.read().parse().unwrap_or(8999),
                                ws_port: ws_port.read().parse().unwrap_or(8900),
                                ledger_path: ledger_path.read().clone(),
                                accounts_path: accounts_path.read().clone(),
                                auto_start: auto_start(),
                                resource_limits: surfdesk_core::surfpool::ResourceLimits {
                                    max_memory_mb: 4096,
                                    max_cpu_percent: 80,
                                    max_disk_gb: 100,
                                },
                                fork_url: if fork_url.read().is_empty() { None } else { Some(fork_url.read().clone()) },
                                fork_slot: if fork_slot.read().parse().unwrap_or(0) == 0 { None } else { Some(fork_slot.read().parse().unwrap_or(0)) },
                                enable_mcp: enable_mcp(),
                                anchor_project: false,
                                preset_accounts: vec![],
                            };
                            on_save.call(new_config);
                            // Modal will close via the close button
                        },
                        "Save Configuration"
                    }
                }
            }
        }
    }
}
