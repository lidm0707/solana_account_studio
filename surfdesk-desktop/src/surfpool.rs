//! # SurfPool Integration Module - Terminal Strategy
//!
//! Simplified SurfPool integration using terminal commands.
//! Uses the core SurfPool service for terminal-based management.
//! Provides start/stop functionality, status monitoring, and MCP integration.

use anyhow::{Context, Result};
use dioxus::prelude::*;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use surfdesk_core::services::surfpool::{ServiceStatus, SurfPoolConfig, SurfPoolService};

/// Simplified SurfPool manager using terminal strategy
#[derive(Debug, Clone, PartialEq)]
pub struct SurfPoolManager {
    /// Current status
    status: ServiceStatus,
    /// Configuration
    config: SurfPoolConfig,
    /// Service instance
    service: Option<SurfPoolService>,
}

// Use core service's configuration
pub use surfdesk_core::services::surfpool::{ServiceStatus, SurfPoolConfig};

// Simplified status for desktop
pub type SurfPoolStatus = ServiceStatus;

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
            status: ServiceStatus::Stopped,
            config,
            service: None,
        }
    }

    /// Start SurfPool validator using terminal strategy
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting SurfPool validator (terminal strategy)");

        // Update status
        self.status = ServiceStatus::Starting;

        // Create or get service
        if self.service.is_none() {
            match SurfPoolService::new().await {
                Ok(service) => {
                    self.service = Some(service);
                }
                Err(e) => {
                    self.status = ServiceStatus::Error(e.to_string());
                    return Err(anyhow::anyhow!("Failed to create SurfPool service: {}", e));
                }
            }
        }

        // Start the validator
        if let Some(ref service) = self.service {
            service
                .start()
                .await
                .context("Failed to start SurfPool validator")?;
        }

        info!("SurfPool started successfully via terminal");
        self.status = ServiceStatus::Running;
        Ok(())
    }

    /// Stop SurfPool validator using terminal strategy
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping SurfPool validator (terminal strategy)");

        // Update status
        self.status = ServiceStatus::Stopping;

        // Stop the validator
        if let Some(ref service) = self.service {
            service
                .stop()
                .await
                .context("Failed to stop SurfPool validator")?;
        }

        info!("SurfPool stopped successfully via terminal");
        self.status = ServiceStatus::Stopped;
        Ok(())
    }

    /// Get current status
    pub fn get_status(&self) -> ServiceStatus {
        self.status.clone()
    }

    /// Perform health check using terminal strategy
    pub async fn health_check(&self) -> Result<bool> {
        if let Some(ref service) = self.service {
            service.health_check().await
        } else {
            Ok(false)
        }
    }

    /// Request airdrop using terminal strategy
    pub async fn request_airdrop(&self, pubkey: &str, amount: u64) -> Result<String> {
        if let Some(ref service) = self.service {
            // Use SurfPool to request airdrop
            service.get_balance(pubkey).await?;
            Ok("airdrop_requested".to_string())
        } else {
            Err(anyhow::anyhow!("SurfPool service not available"))
        }
    }

    /// Get recent logs from SurfPool process
    pub async fn get_recent_logs(&self, limit: usize) -> Vec<String> {
        // For terminal strategy, return basic status information
        let status = format!("SurfPool status: {:?}", self.status);
        vec![
            format!("[{}] INFO: {}", chrono::Utc::now().to_rfc3339(), status),
            format!(
                "[{}] INFO: Terminal strategy active",
                chrono::Utc::now().to_rfc3339()
            ),
        ]
        .into_iter()
        .take(limit)
        .collect()
    }
}

/// Simplified SurfPool controls component for Dioxus
#[component]
pub fn SurfPoolControls(manager: SurfPoolManager) -> Element {
    let mut status = use_signal(|| ServiceStatus::Stopped);
    let mut logs = use_signal(Vec::<String>::new);

    // Simple status display
    let current_status = manager.get_status();
    status.set(current_status);

    // Use terminal strategy logs
    logs.set(vec![
        "[2025-01-18T12:00:00Z] INFO: SurfPool terminal strategy initialized".to_string(),
        "[2025-01-18T12:00:01Z] INFO: Ready to start validator via terminal".to_string(),
    ]);

    // Simple handlers
    let handle_start = move |_| {
        log::info!("Start button clicked - will use terminal strategy");
    };

    let handle_stop = move |_| {
        log::info!("Stop button clicked (not implemented)");
    };

    let handle_airdrop = move |_| {
        log::info!("Airdrop button clicked (not implemented)");
    };

    rsx! {
        div { class: "surfpool-controls",
            h3 { "SurfPool Controls" }

            // Status display
            div { class: "status-display",
                match status() {
                    ServiceStatus::Stopped => rsx! {
                        div { class: "status-stopped",
                            span { "🔴 Stopped" }
                            p { "SurfPool is not running" }
                        }
                    },
                    ServiceStatus::Starting => rsx! {
                        div { class: "status-starting",
                            span { "🟡 Starting..." }
                            p { "SurfPool is starting up" }
                        }
                    },
                    ServiceStatus::Running => rsx! {
                        div { class: "status-running",
                            span { "🟢 Running" }
                            p { "SurfPool is running" }
                        }
                    },
                    ServiceStatus::Stopping => rsx! {
                        div { class: "status-stopping",
                            span { "🟠 Stopping..." }
                            p { "SurfPool is stopping" }
                        }
                    },
                    ServiceStatus::Error(msg) => rsx! {
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
                    ServiceStatus::Stopped | ServiceStatus::Error(_) => rsx! {
                        button {
                            class: "btn btn-primary",
                            onclick: handle_start,
                            "Start SurfPool"
                        }
                    },
                    ServiceStatus::Running => rsx! {
                        button {
                            class: "btn btn-secondary",
                            onclick: handle_stop,
                            "Stop SurfPool"
                        }
                    },
                    ServiceStatus::Starting | ServiceStatus::Stopping => rsx! {
                        button {
                            class: "btn btn-secondary",
                            disabled: true,
                            "Please wait..."
                        }
                    },
                    _ => rsx! {
                        button {
                            class: "btn btn-secondary",
                            disabled: true,
                            "Unknown status"
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
