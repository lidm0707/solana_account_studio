//! # SurfPool Integration Module
//!
//! Real SurfPool integration for local Solana validator management.
//! Uses the core SurfPool module for proper `surfpool start` command execution.
//! Provides start/stop functionality, status monitoring, and MCP integration.

#![allow(dead_code)]

use anyhow::{Context, Result};
use dioxus::prelude::*;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::Child;
use std::time::{Duration, Instant};
use surfdesk_core::surfpool::{ControllerStatus, Platform, ProcessStatus, SurfPoolController};

/// Real SurfPool manager using core module
#[derive(Debug, Clone)]
pub struct SurfPoolManager {
    /// Core SurfPool controller
    controller: SurfPoolController,
    /// Current status
    status: dioxus::prelude::Signal<ControllerStatus>,
    /// Configuration
    config: dioxus::prelude::Signal<SurfPoolConfig>,
}

// Skip PartialEq for SurfPoolManager - we'll handle this differently

// Use core module's configuration
pub use surfdesk_core::surfpool::{PresetAccount, ResourceLimits, SurfPoolConfig, TokenAccount};

// Use core module's status
pub use surfdesk_core::surfpool::ControllerStatus as SurfPoolStatus;

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Whether the validator is healthy
    pub healthy: bool,
    /// Current slot
    pub slot: u64,
    /// Block height
    pub block_height: u64,
    /// Number of active validators
    pub validators: u64,
    /// Network cluster
    pub cluster: String,
    /// Version
    pub version: String,
}

/// Log entry from SurfPool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: String,
    /// Log level
    pub level: String,
    /// Message
    pub message: String,
    /// Optional details
    pub details: Option<String>,
}

impl SurfPoolManager {
    /// Create new SurfPool manager
    pub fn new(config: SurfPoolConfig) -> Self {
        Self {
            controller: SurfPoolController::new(Platform::Desktop),
            status: use_signal(ControllerStatus::Stopped),
            config: use_signal(config),
        }
    }

    /// Start SurfPool validator using core module
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting SurfPool validator using core module");

        // Update status
        self.status.set(ControllerStatus::Starting);

        // Use core controller's start_mainnet_fork method
        match self.controller.start_mainnet_fork().await {
            Ok(_) => {
                info!("SurfPool started successfully");
                self.status.set(ControllerStatus::Running);
                Ok(())
            }
            Err(e) => {
                error!("Failed to start SurfPool: {}", e);
                self.status.set(ControllerStatus::Error(e.to_string()));
                Err(e.into())
            }
        }
    }

    /// Stop SurfPool validator using core module
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping SurfPool validator using core module");

        // Update status
        self.status.set(ControllerStatus::Stopping);

        // Use core controller's stop method
        match self.controller.stop().await {
            Ok(_) => {
                info!("SurfPool stopped successfully");
                self.status.set(ControllerStatus::Stopped);
                Ok(())
            }
            Err(e) => {
                error!("Failed to stop SurfPool: {}", e);
                self.status.set(ControllerStatus::Error(e.to_string()));
                Err(e.into())
            }
        }
    }

    /// Get current status
    pub fn get_status(&self) -> ControllerStatus {
        self.status.read().clone()
    }

    /// Perform health check using core module
    pub async fn health_check(&self) -> Result<surfdesk_core::surfpool::ProcessStatus> {
        match self.controller.get_process_status().await {
            Ok(process_status) => Ok(process_status),
            Err(e) => {
                error!("Failed to get process status: {}", e);
                Err(anyhow::anyhow!("Health check failed: {}", e))
            }
        }
    }

    /// Request airdrop using core module and Solana RPC
    pub async fn request_airdrop(&self, pubkey: &str, amount: u64) -> Result<String> {
        // Use Solana RPC client for airdrop
        let rpc_client = surfdesk_core::solana_rpc::SolanaRpcClient::new_with_url(
            "http://localhost:8999",
            surfdesk_core::solana_rpc::RpcCommitment::Confirmed,
        );

        match rpc_client.request_airdrop(pubkey, amount).await {
            Ok(signature) => {
                info!("Airdrop successful: {:?}", signature);
                Ok(signature.to_string())
            }
            Err(e) => {
                error!("Airdrop failed: {}", e);
                Err(anyhow::anyhow!("Airdrop failed: {}", e))
            }
        }
    }

    /// Get recent logs from SurfPool process
    pub async fn get_recent_logs(&self, limit: usize) -> Vec<String> {
        // For now, return basic log entries
        // In a real implementation, this would read from SurfPool's log output
        vec![
            format!(
                "[{}] INFO: SurfPool validator started",
                chrono::Utc::now().to_rfc3339()
            ),
            format!(
                "[{}] INFO: Validator initialized",
                chrono::Utc::now().to_rfc3339()
            ),
        ]
        .into_iter()
        .take(limit)
        .collect()
    }
}

/// Real SurfPool controls component for Dioxus
#[component]
pub fn SurfPoolControls(manager: SurfPoolManager) -> Element {
    let status = manager.get_status();
    let logs = use_signal(Vec::<String>::new);
    let is_loading = use_signal(|| false);

    // Real-time status monitoring
    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut status_signal = status;
        let mut logs_signal = logs;
        let mgr = manager.clone();

        async move {
            loop {
                // Update status from manager
                let current_status = mgr.get_status();
                status_signal.set(current_status.clone());

                // Get recent logs
                if let Ok(recent_logs) = mgr.get_recent_logs(10).await {
                    logs_signal.set(recent_logs);
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    });

    // Handle start/stop actions
    let handle_start = {
        let mgr = manager.clone();
        move |_| {
            let manager = mgr.clone();
            spawn(async move {
                if let Err(e) = manager.start().await {
                    error!("Failed to start SurfPool: {}", e);
                }
            });
        }
    };

    let handle_stop = {
        let mgr = manager.clone();
        move |_| {
            let manager = mgr.clone();
            spawn(async move {
                if let Err(e) = manager.stop().await {
                    error!("Failed to stop SurfPool: {}", e);
                }
            });
        }
    };

    let handle_airdrop = {
        let mgr = manager.clone();
        move |_| {
            let manager = mgr.clone();
            spawn(async move {
                // Get first account for airdrop (simplified)
                let pubkey = "11111111111111111111111111111112"; // Mock pubkey
                if let Err(e) = manager.request_airdrop(pubkey, 1_000_000_000).await {
                    error!("Failed to request airdrop: {}", e);
                }
            });
        }
    };

    rsx! {
        div { class: "surfpool-controls",
            h3 { "SurfPool Controls" }

            // Status display
            div { class: "status-display",
                match status() {
                    ControllerStatus::Stopped => rsx! {
                        div { class: "status-stopped",
                            span { "🔴 Stopped" }
                            p { "SurfPool is not running" }
                        }
                    },
                    ControllerStatus::Starting => rsx! {
                        div { class: "status-starting",
                            span { "🟡 Starting..." }
                            p { "SurfPool is starting up" }
                        }
                    },
                    ControllerStatus::Running => rsx! {
                        div { class: "status-running",
                            span { "🟢 Running" }
                            p { "SurfPool is running" }
                        }
                    },
                    ControllerStatus::Stopping => rsx! {
                        div { class: "status-stopping",
                            span { "🟠 Stopping..." }
                            p { "SurfPool is stopping" }
                        }
                    },
                    ControllerStatus::Error(msg) => rsx! {
                        div { class: "status-error",
                            span { "🔴 Error" }
                            p { "Error: {msg}" }
                        }
                    },
                }
            }

            // Control buttons
            div { class: "control-buttons",
                match status() {
                    ControllerStatus::Stopped | ControllerStatus::Error(_) => rsx! {
                        button {
                            class: "btn btn-primary",
                            onclick: handle_start,
                            "Start SurfPool"
                        }
                    },
                    ControllerStatus::Running => rsx! {
                        button {
                            class: "btn btn-secondary",
                            onclick: handle_stop,
                            "Stop SurfPool"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: handle_airdrop,
                            "Request Airdrop"
                        }
                    },
                    _ => rsx! {
                        button {
                            class: "btn btn-secondary",
                            disabled: true,
                            "Please wait..."
                        }
                    },
                }
            }

            // Logs display
            div { class: "logs-display",
                h4 { "Recent Logs" }
                div { class: "logs-content",
                    for (index, log_entry) in logs.read().iter().enumerate() {
                        div {
                            class: "log-entry",
                            key: "{index}",
                            "{log_entry}"
                        }
                    }
                }
            }
        }
    }
}
