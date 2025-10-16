//! # SurfPool Integration Module
//!
//! Comprehensive SurfPool integration for local Solana validator management.
//! Provides start/stop functionality, status monitoring, and developer tools
//! for local development environments.

use anyhow::{Context, Result};
use dioxus::prelude::*;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// SurfPool configuration and management
pub struct SurfPoolManager {
    /// Current process instance
    process: Arc<Mutex<Option<Child>>>,
    /// Current status
    status: Arc<Mutex<SurfPoolStatus>>,
    /// Configuration
    config: SurfPoolConfig,
    /// Last status update time
    last_update: Arc<Mutex<Instant>>,
}

/// SurfPool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfPoolConfig {
    /// Port for the validator (default: 8999)
    pub port: u16,
    /// Whether to fork mainnet
    pub fork_mainnet: bool,
    /// RPC URL to fork from
    pub rpc_url: String,
    /// Ledger directory
    pub ledger_path: String,
    /// Account directory
    pub account_path: String,
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: String,
}

impl Default for SurfPoolConfig {
    fn default() -> Self {
        Self {
            port: 8999,
            fork_mainnet: true,
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            ledger_path: "./surfpool-ledger".to_string(),
            account_path: "./surfpool-accounts".to_string(),
            enable_logging: true,
            log_level: "info".to_string(),
        }
    }
}

/// SurfPool status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurfPoolStatus {
    /// Not running
    Stopped,
    /// Starting up
    Starting,
    /// Running and healthy
    Running {
        /// Process ID
        pid: u32,
        /// Uptime in seconds
        uptime: u64,
        /// Port it's running on
        port: u16,
        /// RPC URL
        rpc_url: String,
        /// Last health check
        last_health_check: String,
    },
    /// Stopping
    Stopping,
    /// Error state
    Error {
        /// Error message
        message: String,
        /// Timestamp of error
        timestamp: String,
    },
}

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
            process: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(SurfPoolStatus::Stopped)),
            config,
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Start SurfPool validator
    pub async fn start(&self) -> Result<()> {
        info!("Starting SurfPool validator on port {}", self.config.port);

        // Update status to starting
        {
            let mut status = self.status.lock().unwrap();
            *status = SurfPoolStatus::Starting;
        }

        // Build command
        let mut cmd = Command::new("surfpool");

        // Add arguments
        cmd.arg("--port")
            .arg(self.config.port.to_string())
            .arg("--ledger")
            .arg(&self.config.ledger_path)
            .arg("--account-dir")
            .arg(&self.config.account_path);

        if self.config.fork_mainnet {
            cmd.arg("--fork")
                .arg(&self.config.rpc_url);
        }

        if self.config.enable_logging {
            cmd.arg("--log-level")
                .arg(&self.config.log_level);
        }

        // Set up pipes for stdout/stderr
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Start the process
        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                info!("SurfPool started with PID: {}", pid);

                // Store the process
                {
                    let mut process = self.process.lock().unwrap();
                    *process = Some(child);
                }

                // Update status to running
                {
                    let mut status = self.status.lock().unwrap();
                    let rpc_url = format!("http://localhost:{}", self.config.port);
                    *status = SurfPoolStatus::Running {
                        pid,
                        uptime: 0,
                        port: self.config.port,
                        rpc_url,
                        last_health_check: chrono::Utc::now().to_rfc3339(),
                    };
                }

                // Start background monitoring
                self.start_monitoring().await?;

                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to start SurfPool: {}", e);
                error!("{}", error_msg);

                // Update status to error
                {
                    let mut status = self.status.lock().unwrap();
                    *status = SurfPoolStatus::Error {
                        message: error_msg.clone(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };
                }

                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    /// Stop SurfPool validator
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping SurfPool validator");

        // Update status to stopping
        {
            let mut status = self.status.lock().unwrap();
            *status = SurfPoolStatus::Stopping;
        }

        // Get and stop the process
        {
            let mut process = self.process.lock().unwrap();
            if let Some(mut child) = process.take() {
                debug!("Sending SIGTERM to SurfPool process");
                match child.kill() {
                    Ok(_) => {
                        // Wait for process to exit
                        match child.wait() {
                            Ok(exit_status) => {
                                info!("SurfPool stopped with exit status: {:?}", exit_status);
                            }
                            Err(e) => {
                                warn!("Failed to wait for SurfPool exit: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to kill SurfPool process: {}", e);
                    }
                }
            }
        }

        // Update status to stopped
        {
            let mut status = self.status.lock().unwrap();
            *status = SurfPoolStatus::Stopped;
        }

        Ok(())
    }

    /// Get current status
    pub fn get_status(&self) -> SurfPoolStatus {
        self.status.lock().unwrap().clone()
    }

    /// Perform health check
    pub async fn health_check(&self) -> Result<HealthCheck> {
        let status = self.get_status();

        match status {
            SurfPoolStatus::Running { rpc_url, .. } => {
                // Make HTTP request to health endpoint
                let client = reqwest::Client::new();
                let health_url = format!("{}/health", rpc_url);

                match client.get(&health_url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let health: HealthCheck = response.json().await
                                .context("Failed to parse health check response")?;

                            // Update last health check time
                            {
                                let mut status = self.status.lock().unwrap();
                                if let SurfPoolStatus::Running { .. } = &mut *status {
                                    // This would need to be more sophisticated in real implementation
                                }
                            }

                            Ok(health)
                        } else {
                            Err(anyhow::anyhow!(
                                "Health check returned status: {}",
                                response.status()
                            ))
                        }
                    }
                    Err(e) => {
                        Err(anyhow::anyhow!("Failed to perform health check: {}", e))
                    }
                }
            }
            _ => Err(anyhow::anyhow!("SurfPool is not running"))
        }
    }

    /// Request airdrop on local network
    pub asyncfn request_airdrop(&self, pubkey: &str, amount: u64) -> Result<String> {
        let status = self.get_status();

        match status {
            SurfPoolStatus::Running { rpc_url, .. } => {
                let client = reqwest::Client::new();
                let airdrop_url = format!("{}/airdrop", rpc_url);

                let mut body = serde_json::json!({
                    "pubkey": pubkey,
                    "lamports": amount
                });

                let response = client.post(&airdrop_url)
                    .json(&body)
                    .send()
                    .await
                    .context("Failed to request airdrop")?;

                if response.status().is_success() {
                    let result: serde_json::Value = response.json().await
                        .context("Failed to parse airdrop response")?;

                    Ok(result.to_string())
                } else {
                    Err(anyhow::anyhow!(
                        "Airdrop request failed with status: {}",
                        response.status()
                    ))
                }
            }
            _ => Err(anyhow::anyhow!("SurfPool is not running"))
        }
    }

    /// Get recent logs
    pub fn get_recent_logs(&self, limit: usize) -> Vec<LogEntry> {
        // This would read from log files or stdout buffer
        // For now, return mock data
        vec![
            LogEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "INFO".to_string(),
                message: "SurfPool validator started".to_string(),
                details: Some(format!("Port: {}", self.config.port)),
            },
            LogEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "INFO".to_string(),
                message: "Validator initialized".to_string(),
                details: Some("Fork from mainnet".to_string()),
            },
        ]
        .into_iter()
        .take(limit)
        .collect()
    }

    /// Start background monitoring
    async fn start_monitoring(&self) -> Result<()> {
        let status = Arc::clone(&self.status);
        let process = Arc::clone(&self.process);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;

                // Check if process is still running
                {
                    let mut proc = process.lock().unwrap();
                    if let Some(child) = proc.as_mut() {
                        match child.try_wait() {
                            Ok(Some(exit_status)) => {
                                warn!("SurfPool process exited with status: {:?}", exit_status);

                                // Update status to stopped
                                let mut st = status.lock().unwrap();
                                *st = SurfPoolStatus::Stopped;
                                break;
                            }
                            Ok(None) => {
                                // Process is still running, update uptime
                                if let SurfPoolStatus::Running { uptime, .. } = &mut *status.lock().unwrap() {
                                    *uptime += 5;
                                }
                            }
                            Err(e) => {
                                error!("Error checking SurfPool process: {}", e);
                            }
                        }
                    } else {
                        // Process doesn't exist
                        let mut st = status.lock().unwrap();
                        *st = SurfPoolStatus::Stopped;
                        break;
                    }
                }
            }
        });

        Ok(())
    }
}

/// Dioxus component for SurfPool controls
#[derive(Debug, Clone, Props)]
pub struct SurfPoolControlsProps {
    /// SurfPool manager instance
    pub manager: Arc<SurfPoolManager>,
    /// On status change callback
    pub on_status_change: Option<EventHandler<SurfPoolStatus>>,
}

/// SurfPool controls component
#[component]
pub fn SurfPoolControls(props: SurfPoolControlsProps) -> Element {
    let mut status = use_signal(|| props.manager.get_status());
    let mut logs = use_signal(Vec::<LogEntry>::new);
    let mut is_loading = use_signal(|| false);

    // Update status periodically
    use_coroutine(|_| {
        let manager = Arc::clone(&props.manager);
        let status_signal = status.clone();

        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                status_signal.set(manager.get_status());
            }
        }
    });

    let start_surfpool = move |_| {
        let manager = Arc::clone(&props.manager);
        let loading = is_loading.clone();

        spawn_local(async move {
            loading.set(true);
            match manager.start().await {
                Ok(_) => {
                    log::info!("SurfPool started successfully");
                }
                Err(e) => {
                    log::error!("Failed to start SurfPool: {}", e);
                }
            }
            loading.set(false);
        });
    };

    let stop_surfpool = move |_| {
        let manager = Arc::clone(&props.manager);
        let loading = is_loading.clone();

        spawn_local(async move {
            loading.set(true);
            match manager.stop().await {
                Ok(_) => {
                    log::info!("SurfPool stopped successfully");
                }
                Err(e) => {
                    log::error!("Failed to stop SurfPool: {}", e);
                }
            }
            loading.set(false);
        });
    };

    let refresh_logs = move |_| {
        let manager = Arc::clone(&props.manager);
        let logs_signal = logs.clone();

        spawn_local(async move {
            let recent_logs = manager.get_recent_logs(50);
            logs_signal.set(recent_logs);
        });
    };

    rsx! {
        div { class: "surfpool-controls",

            // Status display
            div { class: "surfpool-status",
                h3 { "SurfPool Status" }

                match status() {
                    SurfPoolStatus::Stopped => {
                        div { class: "status-stopped",
                            span { class: "status-indicator status-offline" }
                            span { "Stopped" }
                        }
                    }
                    SurfPoolStatus::Starting => {
                        div { class: "status-starting",
                            span { class: "status-indicator status-starting" }
                            span { "Starting..." }
                        }
                    }
                    SurfPoolStatus::Running { pid, uptime, port, rpc_url, .. } => {
                        div { class: "status-running",
                            span { class: "status-indicator status-online" }
                            span { "Running" }

                            div { class: "status-details",
                                p { "PID: {pid}" }
                                p { "Port: {port}" }
                                p { "Uptime: {uptime}s" }
                                p { "RPC: {rpc_url}" }
                            }
                        }
                    }
                    SurfPoolStatus::Stopping => {
                        div { class: "status-stopping",
                            span { class: "status-indicator status-stopping" }
                            span { "Stopping..." }
                        }
                    }
                    SurfPoolStatus::Error { message, .. } => {
                        div { class: "status-error",
                            span { class: "status-indicator status-error" }
                            span { "Error: {message}" }
                        }
                    }
                }
            }

            // Control buttons
            div { class: "surfpool-actions",
                match status() {
                    SurfPoolStatus::Stopped | SurfPoolStatus::Error { .. } => {
                        button {
                            class: "surf-button surf-button--primary",
                            onclick: start_surfpool,
                            disabled: is_loading(),
                            "Start SurfPool"
                        }
                    }
                    SurfPoolStatus::Running { .. } => {
                        button {
                            class: "surf-button surf-button--error",
                            onclick: stop_surfpool,
                            disabled: is_loading(),
                            "Stop SurfPool"
                        }
                    }
                    _ => {
                        button {
                            class: "surf-button surf-button--secondary",
                            disabled: true,
                            "Please wait..."
                        }
                    }
                }

                button {
                    class: "surf-button surf-button--ghost",
                    onclick: refresh_logs,
                    "Refresh Logs"
                }
            }

            // Logs display
            div { class: "surfpool-logs",
                h4 { "Recent Logs" }

                div { class: "logs-container",
                    for log in logs() {
                        div { class: format!("log-entry log-{}", log.level.to_lowercase()),
                            span { class: "log-timestamp", "{log.timestamp}" }
                            span { class: "log-level", "[{log.level}]" }
                            span { class: "log-message", "{log.message}" }
                        }
                    }
                }
            }
        }
    }
}
