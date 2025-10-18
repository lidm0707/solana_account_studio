//! # SurfPool Page Component
//!
//! Enhanced SurfPool integration with real-time controls, status monitoring,
//! live metrics, log viewing, and configuration management for local Solana validator.

use crate::surfpool::{SurfPoolConfig, SurfPoolManager, SurfPoolMetrics, SurfPoolStatus};
use dioxus::prelude::*;
use log::{error, info, warn};
use std::time::Duration;
use surfdesk_core::components::{Button, Card, Input, Size, Variant};
use surfdesk_core::solana_rpc::SolanaRpcClient;

// Re-export spawn from tokio for async tasks
use dioxus::prelude::spawn;

/// Enhanced SurfPool control page component
#[component]
pub fn SurfPoolPage() -> Element {
    let mut config = use_signal(SurfPoolConfig::default);
    let mut show_config_modal = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut current_status = use_signal(|| Option::<SurfPoolStatus>::None);
    let mut current_metrics = use_signal(|| Option::<SurfPoolMetrics>::None);
    let mut is_refreshing = use_signal(|| false);

    // Enhanced SurfPool manager with real service integration
    let surfpool_manager = use_signal(|| {
        let cfg = config.read().clone();
        SurfPoolManager::new(cfg)
    });

    // Real RPC client for health checks and validation
    let rpc_client = use_signal(|| {
        SolanaRpcClient::new_with_url(
            "http://localhost:8899", // Default SurfPool port from testing
            surfdesk_core::solana_rpc::RpcCommitment::Confirmed,
        )
    });

    // Enhanced periodic status and metrics update
    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut error_signal = error_message;
        let mut status_signal = current_status;
        let mut metrics_signal = current_metrics;
        let mut refreshing_signal = is_refreshing;
        let manager = surfpool_manager.clone();
        let rpc = rpc_client.clone();

        async move {
            loop {
                // Set refreshing state
                refreshing_signal.set(true);

                // Get SurfPool status
                if let Ok(manager_ref) = manager.try_read() {
                    match manager_ref.get_status().await {
                        Ok(status) => {
                            status_signal.set(Some(status));
                            error_signal.set(None);
                        }
                        Err(e) => {
                            warn!("Failed to get SurfPool status: {}", e);
                            error_signal.set(Some(format!("Status check failed: {}", e)));
                        }
                    }

                    // Get SurfPool metrics
                    match manager_ref.get_metrics().await {
                        Ok(metrics) => {
                            metrics_signal.set(Some(metrics));
                        }
                        Err(e) => {
                            warn!("Failed to get SurfPool metrics: {}", e);
                        }
                    }
                }

                // Periodic RPC health check
                if let Ok(health) = rpc.read().get_health().await {
                    if health {
                        info!("RPC health check passed");
                    } else {
                        warn!("RPC health check failed");
                        error_signal.set(Some("RPC health check failed".to_string()));
                    }
                } else {
                    warn!("Unable to connect to RPC");
                    error_signal.set(Some("Unable to connect to RPC".to_string()));
                }

                refreshing_signal.set(false);
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    });

    // Handle configuration change
    let handle_config_change = move |_| {
        show_config_modal.set(true);
    };

    // Handle manual refresh
    let handle_refresh = move |_| {
        let manager = surfpool_manager.clone();
        let mut status_signal = current_status;
        let mut metrics_signal = current_metrics;
        let mut refreshing_signal = is_refreshing;

        spawn(async move {
            refreshing_signal.set(true);

            if let Ok(manager_ref) = manager.try_read() {
                // Refresh status
                match manager_ref.get_status().await {
                    Ok(status) => {
                        status_signal.set(Some(status));
                        info!("SurfPool status refreshed successfully");
                    }
                    Err(e) => {
                        error!("Failed to refresh SurfPool status: {}", e);
                    }
                }

                // Refresh metrics
                match manager_ref.get_metrics().await {
                    Ok(metrics) => {
                        metrics_signal.set(Some(metrics));
                        info!("SurfPool metrics refreshed successfully");
                    }
                    Err(e) => {
                        error!("Failed to refresh SurfPool metrics: {}", e);
                    }
                }
            }

            refreshing_signal.set(false);
        });
    };

    // Handle start SurfPool
    let handle_start_surfpool = move |_| {
        let manager = surfpool_manager.clone();
        let mut error_signal = error_message;

        spawn(async move {
            if let Ok(mut manager_ref) = manager.try_write() {
                match manager_ref.start().await {
                    Ok(_) => {
                        info!("SurfPool started successfully");
                    }
                    Err(e) => {
                        error!("Failed to start SurfPool: {}", e);
                        error_signal.set(Some(format!("Failed to start SurfPool: {}", e)));
                    }
                }
            }
        });
    };

    // Handle stop SurfPool
    let handle_stop_surfpool = move |_| {
        let manager = surfpool_manager.clone();
        let mut error_signal = error_message;

        spawn(async move {
            if let Ok(mut manager_ref) = manager.try_write() {
                match manager_ref.stop().await {
                    Ok(_) => {
                        info!("SurfPool stopped successfully");
                    }
                    Err(e) => {
                        error!("Failed to stop SurfPool: {}", e);
                        error_signal.set(Some(format!("Failed to stop SurfPool: {}", e)));
                    }
                }
            }
        });
    };

    // Handle configuration save
    let handle_save_config = move |new_config: SurfPoolConfig| {
        let mut manager_signal = surfpool_manager.clone();
        let mut config_signal = config.clone();
        let mut show_modal = show_config_modal.clone();

        spawn(async move {
            if let Ok(mut manager) = manager_signal.try_write() {
                match manager.update_config(new_config.clone()).await {
                    Ok(_) => {
                        info!("SurfPool configuration updated successfully");
                        config_signal.set(new_config);
                        show_modal.set(false);
                    }
                    Err(e) => {
                        error!("Failed to update SurfPool configuration: {}", e);
                    }
                }
            }
        });
    };

    // Handle export logs
    let handle_export_logs = move |_| {
        let manager = surfpool_manager.clone();

        spawn(async move {
            if let Ok(manager_ref) = manager.try_read() {
                let logs = manager_ref.get_recent_logs(1000).await;

                // Create log content
                let log_content = logs
                    .iter()
                    .map(|log| {
                        format!(
                            "[{}] [{}] [{}]: {}",
                            log.timestamp,
                            log.level,
                            log.source.as_deref().unwrap_or("Unknown"),
                            log.message
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                // In a real implementation, this would save to a file
                info!("Exported {} log entries", logs.len());
            }
        });
    };

    rsx! {
        div { class: "surfpool-page page-container",

            // Header
            div { class: "page-header",
                div { class: "page-title-section",
                    h1 { class: "page-title", "🌊 SurfPool Control" }
                    p { class: "page-subtitle",
                        "Manage your local Solana validator with real-time monitoring and control" }
                }
                div { class: "page-actions",
                    Button {
                        variant: Variant::Secondary,
                        size: Size::Medium,
                        onclick: handle_config_change,
                        children: rsx! {
                            "⚙️ Configuration"
                        }
                    }
                    Button {
                        variant: Variant::Secondary,
                        size: Size::Medium,
                        onclick: handle_export_logs,
                        children: rsx! {
                            "📥 Export Logs"
                        }
                    }
                }
            }

            // Error display
            if let Some(error) = error_message.read().as_ref() {
                Card {
                    variant: Variant::Error,
                    elevated: false,
                    children: rsx! {
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl", "⚠️" }
                            div { class: "flex-1",
                                h3 { class: "font-semibold text-error", "Connection Error" }
                                p { class: "text-sm text-error/80", "{error}" }
                            }
                            Button {
                                variant: Variant::Ghost,
                                size: Size::Small,
                                onclick: move |_| error_message.set(None),
                                children: rsx! { "×" }
                            }
                        }
                    }
                }
            }

            // Enhanced Status Display
            Card {
                title: Some("📊 Real-Time Status".to_string()),
                size: Size::Large,
                elevated: true,
                children: rsx! {
                    div { class: "surfpool-status-display",
                        // Status indicator
                        div { class: "status-header",
                            div { class: "flex items-center gap-3",
                                div { class: format!("status-indicator {}",
                                    match current_status.read().as_ref().map(|s| &s.service_status) {
                                        Some(crate::surfpool::ServiceStatus::Running) => "status-online",
                                        Some(crate::surfpool::ServiceStatus::Starting) => "status-starting",
                                        Some(crate::surfpool::ServiceStatus::Stopping) => "status-stopping",
                                        Some(crate::surfpool::ServiceStatus::Stopped) => "status-offline",
                                        Some(crate::surfpool::ServiceStatus::Error(_)) => "status-error",
                                        None => "status-offline"
                                    })
                                }
                                span { class: "status-text",
                                    {current_status.read().as_ref()
                                        .map(|s| format!("{:?}", s.service_status))
                                        .unwrap_or_else(|| "Unknown".to_string())}
                                }
                                if is_refreshing() {
                                    span { class: "text-xs text-muted ml-2", "Refreshing..." }
                                }
                            }
                            Button {
                                variant: Variant::Ghost,
                                size: Size::Small,
                                onclick: handle_refresh,
                                disabled: is_refreshing(),
                                children: rsx! {
                                    "🔄 Refresh"
                                }
                            }
                        }

                        // Control buttons
                        div { class: "surfpool-actions mt-4",
                            Button {
                                variant: Variant::Primary,
                                size: Size::Medium,
                                onclick: handle_start_surfpool,
                                disabled: current_status.read().as_ref()
                                    .map(|s| matches!(s.service_status, crate::surfpool::ServiceStatus::Running))
                                    .unwrap_or(false),
                                children: rsx! {
                                    "▶️ Start SurfPool"
                                }
                            }
                            Button {
                                variant: Variant::Secondary,
                                size: Size::Medium,
                                onclick: handle_stop_surfpool,
                                disabled: current_status.read().as_ref()
                                    .map(|s| !matches!(s.service_status, crate::surfpool::ServiceStatus::Running))
                                    .unwrap_or(true),
                                children: rsx! {
                                    "⏹️ Stop SurfPool"
                                }
                            }
                        }

                        // Status details
                        if let Some(status) = current_status.read().as_ref() {
                            div { class: "status-details mt-6 grid grid-cols-2 md:grid-cols-4 gap-4",
                                div { class: "metric-item",
                                    span { class: "metric-label", "Current Slot" }
                                    span { class: "metric-value", "{status.current_slot}" }
                                }
                                div { class: "metric-item",
                                    span { class: "metric-label", "Block Height" }
                                    span { class: "metric-value", "{status.block_height}" }
                                }
                                div { class: "metric-item",
                                    span { class: "metric-label", "Connected Peers" }
                                    span { class: "metric-value", "{status.connected_peers}" }
                                }
                                div { class: "metric-item",
                                    span { class: "metric-label", "Network" }
                                    span { class: "metric-value", "{status.network}" }
                                }
                            }
                        }

                        // Metrics display
                        if let Some(metrics) = current_metrics.read().as_ref() {
                            div { class: "metrics-section mt-6",
                                h4 { class: "font-semibold mb-3", "📈 Performance Metrics" }
                                div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
                                    div { class: "metric-item",
                                        span { class: "metric-label", "TPS" }
                                        span { class: "metric-value", "{metrics.tps}" }
                                    }
                                    div { class: "metric-item",
                                        span { class: "metric-label", "Validators" }
                                        span { class: "metric-value", "{metrics.validators}" }
                                    }
                                    div { class: "metric-item",
                                        span { class: "metric-label", "Memory" }
                                        span { class: "metric-value", "{metrics.memory_mb}MB" }
                                    }
                                    div { class: "metric-item",
                                        span { class: "metric-label", "CPU" }
                                        span { class: "metric-value", "{metrics.cpu_percent}%" }
                                    }
                                    div { class: "metric-item",
                                        span { class: "metric-label", "Latency" }
                                        span { class: "metric-value", "{metrics.network_latency_ms}ms" }
                                    }
                                    div { class: "metric-item",
                                        span { class: "metric-label", "Accounts" }
                                        span { class: "metric-value", "{metrics.active_accounts}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Enhanced SurfPool Controls
            Card {
                title: Some("🎛️ Advanced Controls".to_string()),
                size: Size::Large,
                elevated: true,
                children: rsx! {
                    // Use the enhanced SurfPoolControls component
                    crate::surfpool::SurfPoolControls {
                        manager: std::sync::Arc::new(surfpool_manager.read().clone())
                    }
                }
            }

            // Service Information
            Card {
                title: Some("📋 Service Information".to_string()),
                size: Size::Medium,
                elevated: true,
                children: rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div { class: "space-y-3",
                            h4 { class: "font-semibold text-primary", "🔗 Connection Details" }
                            div { class: "space-y-2 text-sm",
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "RPC URL:" }
                                    span { class: "font-mono text-accent", "http://localhost:8899" }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "WebSocket:" }
                                    span { class: "font-mono text-accent", "ws://localhost:8900" }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "Studio UI:" }
                                    span { class: "font-mono text-accent", "http://localhost:18488" }
                                }
                            }
                        }
                        div { class: "space-y-3",
                            h4 { class: "font-semibold text-primary", "🛠️ Development Tools" }
                            div { class: "space-y-2 text-sm",
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "Solana CLI:" }
                                    code { class: "bg-surface px-2 py-1 rounded text-xs",
                                        "// solana config set --url http://127.0.0.1:8899" }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "Network Type:" }
                                    span { class: "font-semibold text-success", "Mainnet Fork" }
                                }
                                div { class: "flex justify-between",
                                    span { class: "text-muted", "Version:" }
                                    span { class: "font-mono text-info", "SurfPool 0.10.7" }
                                }
                            }
                        }
                    }
                }
            }

            // Quick Actions
            Card {
                title: Some("⚡ Quick Actions".to_string()),
                size: Size::Medium,
                elevated: true,
                children: rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        Button {
                            variant: Variant::Outline,
                            size: Size::Medium,
                            onclick: move |_| {
                                spawn(async move {
                                    // Open browser to Studio UI
                                    info!("Opening Studio UI in browser");
                                });
                            },
                            children: rsx! {
                                div { class: "flex items-center gap-2",
                                    span { "🖥️" }
                                    span { "Open Studio UI" }
                                }
                            }
                        }
                        Button {
                            variant: Variant::Outline,
                            size: Size::Medium,
                            onclick: move |_| {
                                let rpc = rpc_client.clone();
                                let mut error_signal = error_message;

                                spawn(async move {
                                    match rpc.read().get_health().await {
                                        Ok(healthy) => {
                                            if healthy {
                                                info!("RPC connection test successful");
                                            } else {
                                                warn!("RPC connection test failed - unhealthy");
                                                error_signal.set(Some("RPC connection unhealthy".to_string()));
                                            }
                                        }
                                        Err(e) => {
                                            error!("RPC connection test failed: {}", e);
                                            error_signal.set(Some(format!("RPC test failed: {}", e)));
                                        }
                                    }
                                });
                            },
                            children: rsx! {
                                div { class: "flex items-center gap-2",
                                    span { "🔍" }
                                    span { "Test Connection" }
                                }
                            }
                        }
                        Button {
                            variant: Variant::Outline,
                            size: Size::Medium,
                            onclick: move |_| {
                                let rpc = rpc_client.clone();
                                let mut error_signal = error_message;

                                spawn(async move {
                                    match rpc.read().get_version().await {
                                        Ok(version) => {
                                            info!("Validator version: {}", version);
                                        }
                                        Err(e) => {
                                            error!("Failed to get validator info: {}", e);
                                            error_signal.set(Some(format!("Failed to get validator info: {}", e)));
                                        }
                                    }
                                });
                            },
                            children: rsx! {
                                div { class: "flex items-center gap-2",
                                    span { "📊" }
                                    span { "Validator Info" }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Configuration Modal
        if show_config_modal() {
            ConfigModal {
                config: config.read().clone(),
                on_save: handle_save_config,
                on_cancel: move |_| show_config_modal.set(false),
            }
        }
    }
}

/// Configuration modal component
#[component]
fn ConfigModal(
    config: SurfPoolConfig,
    on_save: EventHandler<SurfPoolConfig>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut local_config = use_signal(|| config);
    let mut errors = use_signal(Vec::<String>::new);

    let handle_save = move |_| {
        let new_config = local_config.read().clone();
        on_save.call(new_config);
    };

    let handle_port_change = move |port: u16| {
        local_config.write().rpc_port = port;
    };

    let handle_ws_port_change = move |port: u16| {
        local_config.write().ws_port = port;
    };

    let handle_fork_url_change = move |url: String| {
        local_config.write().fork_url = url;
    };

    rsx! {
        div { class: "modal-overlay fixed inset-0 bg-black/50 flex items-center justify-center z-50",
            div { class: "modal-content bg-surface border border-surface rounded-xl shadow-2xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto",
                div { class: "modal-header p-6 border-b border-surface",
                    h2 { class: "text-xl font-bold text-primary", "⚙️ SurfPool Configuration" }
                    p { class: "text-sm text-muted mt-1",
                        "Customize your local Solana validator settings" }
                }

                div { class: "modal-body p-6 space-y-6",
                    // RPC Port Configuration
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "RPC Port" }
                        Input {
                            value: local_config.read().rpc_port.to_string(),
                            placeholder: "8899",
                            oninput: move |e| {
                                if let Ok(port) = e.value().parse() {
                                    handle_port_change(port);
                                }
                            },
                            variant: Variant::Default,
                            size: Size::Medium,
                        }
                        p { class: "text-xs text-muted",
                            "Port for the JSON-RPC API (default: 8899)" }
                    }

                    // WebSocket Port Configuration
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "WebSocket Port" }
                        Input {
                            value: local_config.read().ws_port.to_string(),
                            placeholder: "8900",
                            oninput: move |e| {
                                if let Ok(port) = e.value().parse() {
                                    handle_ws_port_change(port);
                                }
                            },
                            variant: Variant::Default,
                            size: Size::Medium,
                        }
                        p { class: "text-xs text-muted",
                            "Port for WebSocket connections (default: 8900)" }
                    }

                    // Fork URL Configuration
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Fork URL" }
                        Input {
                            value: local_config.read().fork_url.clone(),
                            placeholder: "https://api.mainnet-beta.solana.com",
                            oninput: move |e| handle_fork_url_change(e.value()),
                            variant: Variant::Default,
                            size: Size::Medium,
                        }
                        p { class: "text-xs text-muted",
                            "URL of the network to fork from" }
                    }

                    // MCP Integration
                    div { class: "flex items-center gap-3",
                        input {
                            r#type: "checkbox",
                            checked: local_config.read().enable_mcp,
                            onchange: move |e| {
                                local_config.write().enable_mcp = e.checked();
                            },
                            class: "w-4 h-4 text-primary border-surface rounded focus:ring-accent"
                        }
                        label { class: "text-sm font-medium text-primary",
                            "Enable MCP Integration" }
                    }
                    p { class: "text-xs text-muted ml-7",
                        "Enable Model Context Protocol for enhanced tooling integration" }
                }

                div { class: "modal-footer p-6 border-t border-surface flex justify-end gap-3",
                    Button {
                        variant: Variant::Secondary,
                        size: Size::Medium,
                        onclick: move |_| on_cancel.call(()),
                        children: rsx! { "Cancel" }
                    }
                    Button {
                        variant: Variant::Primary,
                        size: Size::Medium,
                        onclick: handle_save,
                        children: rsx! { "💾 Save Configuration" }
                    }
                }
            }
        }
    }
}
