//! # SurfPool Page Component
//!
//! Real SurfPool integration with start/stop controls, status monitoring,
//! log viewing, and configuration management for local Solana validator.

#![allow(dead_code)]

use crate::surfpool::{SurfPoolConfig, SurfPoolManager, SurfPoolStatus as DesktopSurfPoolStatus};
use dioxus::prelude::*;
use std::sync::Arc;
use surfdesk_core::components::{Button, Card, Input, Size, Variant};
use surfdesk_core::solana_rpc::{SolanaNetwork, SolanaRpcClient};

/// SurfPool control page component
#[component]
pub fn SurfPoolPage() -> Element {
    let surfpool_status = use_signal(|| DesktopSurfPoolStatus::Stopped);
    let logs = use_signal(Vec::<String>::new);
    let config = use_signal(SurfPoolConfig::default);
    let show_config_modal = use_signal(|| false);
    let loading = use_signal(|| false);
    let error_message = use_signal(|| Option::<String>::None);

    // Real SurfPool manager (no Arc for Dioxus compatibility)
    let surfpool_manager = use_signal(|| {
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
        let manager = surfpool_manager.read().clone();
        let rpc = rpc_client.read().clone();

        async move {
            loop {
                // Update status from manager
                let current_status = manager.get_status();
                status_signal.set(current_status.clone());

                // Collect logs from process
                let recent_logs = manager.get_recent_logs(50);
                let log_strings: Vec<String> = recent_logs
                    .into_iter()
                    .map(|log| format!("[{}] {}: {}", log.timestamp, log.level, log.message))
                    .collect();
                logs_signal.set(log_strings);

                // Perform health check if running
                if let DesktopSurfPoolStatus::Running { .. } = &current_status {
                    match manager.health_check().await {
                        Ok(health) => {
                            log::info!("Health check passed: slot {}", health.slot);
                            error_signal.set(None);
                        }
                        Err(e) => {
                            log::warn!("Health check failed: {}", e);
                            error_signal.set(Some(format!("Health check failed: {}", e)));
                        }
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    });

    // Handle start SurfPool
    let handle_start = {
        let manager = surfpool_manager.read().clone();
        let mut loading_signal = loading;
        let mut error_signal = error_message;
        let current_config = config.read().clone();

        move |_| {
            let manager = manager.clone();
            let mut loading = loading_signal.clone();
            let mut error = error_signal.clone();
            let config = current_config.clone();

            spawn(async move {
                loading.set(true);
                error.set(None);

                // Update manager config
                let new_manager = SurfPoolManager::new(config.read().clone());

                match new_manager.start().await {
                    Ok(_) => {
                        log::info!("SurfPool started successfully");
                        *surfpool_manager.write() = new_manager;
                    }
                    Err(e) => {
                        log::error!("Failed to start SurfPool: {}", e);
                        error.set(Some(format!("Failed to start SurfPool: {}", e)));
                    }
                }

                loading.set(false);
            });
        }
    };

    // Handle stop SurfPool
    let handle_stop = {
        let manager = surfpool_manager.read().clone();
        let mut loading_signal = loading;
        let mut error_signal = error_message;

        move |_| {
            let manager = manager.clone();
            let mut loading = loading_signal.clone();
            let mut error = error_signal.clone();

            spawn(async move {
                loading.set(true);
                error.set(None);

                match manager.stop().await {
                    Ok(_) => {
                        log::info!("SurfPool stopped successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to stop SurfPool: {}", e);
                        error.set(Some(format!("Failed to stop SurfPool: {}", e)));
                    }
                }

                loading.set(false);
            });
        }
    };

    // Handle restart SurfPool
    let handle_restart = {
        let manager = surfpool_manager.read().clone();
        let mut loading_signal = loading;
        let mut error_signal = error_message;

        move |_| {
            let manager = manager.clone();
            let mut loading = loading_signal.clone();
            let mut error = error_signal.clone();

            spawn(async move {
                loading.set(true);
                error.set(None);

                // Stop first
                if let Err(e) = manager.stop().await {
                    error.set(Some(format!("Failed to stop SurfPool: {}", e)));
                    loading.set(false);
                    return;
                }

                // Wait a moment
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // Start again
                match manager.start().await {
                    Ok(_) => {
                        log::info!("SurfPool restarted successfully");
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to restart SurfPool: {}", e)));
                    }
                }

                loading.set(false);
            });
        }
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
                                DesktopSurfPoolStatus::Running { pid, uptime, port, rpc_url, .. } => rsx! {
                                    div { class: "status-running",
                                        div { class: "status-indicator running" }
                                        div { class: "status-details",
                                            h3 { "SurfPool Running" }
                                            div { class: "status-info",
                                                div { class: "info-item",
                                                    span { class: "info-label", "Process ID:" }
                                                    span { class: "info-value", "{pid}" }
                                                }
                                                div { class: "info-item",
                                                    span { class: "info-label", "Port:" }
                                                    span { class: "info-value", "{port}" }
                                                }
                                                div { class: "info-item",
                                                    span { class: "info-label", "RPC URL:" }
                                                    span { class: "info-value", "{rpc_url}" }
                                                }
                                                div { class: "info-item",
                                                    span { class: "info-label", "Uptime:" }
                                                    span { class: "info-value", "{uptime}s" }
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
                                DesktopSurfPoolStatus::Running { .. } => rsx! {
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
    let mut port = use_signal(config.port.to_string());
    let mut fork_mainnet = use_signal(config.fork_mainnet);
    let mut rpc_url = use_signal(config.rpc_url.clone());
    let mut ledger_path = use_signal(config.ledger_path.clone());
    let mut account_path = use_signal(config.account_path.clone());
    let mut enable_logging = use_signal(config.enable_logging);
    let mut log_level = use_signal(config.log_level.clone());

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
                        label { "Port" }
                        Input {
                            value: port.read().clone(),
                            on_input: move |s| port.set(s),
                            placeholder: "8999",
                            r#type: "number",
                        }
                    }

                    div { class: "form-group",
                        input {
                            r#type: "checkbox",
                            id: "fork-mainnet",
                            checked: fork_mainnet(),
                            oninput: move |e| {
                                if let Some(checked) = e.value().as_bool() {
                                    fork_mainnet.set(checked);
                                }
                            }
                        }
                        label { r#for: "fork-mainnet", "Fork Mainnet" }
                    }

                    if fork_mainnet() {
                        div { class: "form-group",
                            label { "RPC URL" }
                            Input {
                                value: rpc_url.read().clone(),
                                on_input: move |s| rpc_url.set(s),
                                placeholder: "https://api.mainnet-beta.solana.com",
                            }
                        }
                    }

                    div { class: "form-group",
                        label { "Ledger Path" }
                        Input {
                            value: ledger_path.read().clone(),
                            on_input: move |s| ledger_path.set(s),
                            placeholder: "./surfpool-ledger",
                        }
                    }

                    div { class: "form-group",
                        label { "Account Path" }
                        Input {
                            value: account_path.read().clone(),
                            on_input: move |s| account_path.set(s),
                            placeholder: "./surfpool-accounts",
                        }
                    }

                    div { class: "form-group",
                        input {
                            r#type: "checkbox",
                            id: "enable-logging",
                            checked: enable_logging(),
                            oninput: move |e| {
                                if let Some(checked) = e.value().as_bool() {
                                    enable_logging.set(checked);
                                }
                            }
                        }
                        label { r#for: "enable-logging", "Enable Logging" }
                    }

                    if enable_logging() {
                        div { class: "form-group",
                            label { "Log Level" }
                            select {
                                onchange: move |e| log_level.set(e.value()),
                                option { value: "debug", "Debug" }
                                option { value: "info", "Info" }
                                option { value: "warn", "Warn" }
                                option { value: "error", "Error" }
                            }
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
                                port: port.read().parse().unwrap_or(8999),
                                fork_mainnet: fork_mainnet(),
                                rpc_url: rpc_url.read().clone(),
                                ledger_path: ledger_path.read().clone(),
                                account_path: account_path.read().clone(),
                                enable_logging: enable_logging(),
                                log_level: log_level.read().clone(),
                            };
                            on_save.call(new_config);
                            on_close.call(().into());
                        },
                        "Save Configuration"
                    }
                }
            }
        }
    }
}
