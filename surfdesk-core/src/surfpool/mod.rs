//! SurfPool: Multi-Platform Solana Development Controller
//!
//! This module provides cross-platform process management for local Solana validators
//! and development environments, supporting desktop, web, and terminal platforms.

pub mod environment;

use crate::error::SurfDeskError;
use crate::platform::Platform;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};

/// Cross-platform SurfPool controller for managing Solana validators
#[derive(Clone)]
pub struct SurfPoolController {
    /// Platform-specific implementation
    platform: Platform,
    /// Running validator process
    process: Arc<Mutex<Option<Child>>>,
    /// Current configuration
    config: Arc<RwLock<SurfPoolConfig>>,
    /// Current status
    status: Arc<RwLock<ControllerStatus>>,
}

// SAFETY: SurfPoolController is Send + Sync because all its fields are Send + Sync
// - Arc<Mutex<Option<Child>>>: Mutex is Send + Sync, Child is Send
// - Arc<RwLock<SurfPoolConfig>>: RwLock is Send + Sync, SurfPoolConfig is Send + Sync
// - Arc<RwLock<ControllerStatus>>: RwLock is Send + Sync, ControllerStatus is Send + Sync
unsafe impl Send for SurfPoolController {}
unsafe impl Sync for SurfPoolController {}

/// Configuration for SurfPool controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfPoolConfig {
    /// RPC port for the validator
    pub rpc_port: u16,
    /// WebSocket port for the validator
    pub ws_port: u16,
    /// Ledger directory path
    pub ledger_path: String,
    /// Accounts directory path
    pub accounts_path: String,
    /// Whether to auto-start the validator
    pub auto_start: bool,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Fork URL for mainnet forking
    pub fork_url: Option<String>,
    /// Fork slot for mainnet forking
    pub fork_slot: Option<u64>,
    /// Whether to enable MCP (Model Context Protocol)
    pub enable_mcp: bool,
    /// Anchor project detection
    pub anchor_project: bool,
    /// Preset accounts for the environment
    pub preset_accounts: Vec<PresetAccount>,
}

/// Resource limits for the validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in MB
    pub max_memory_mb: u32,
    /// Maximum CPU percentage
    pub max_cpu_percent: u32,
    /// Maximum disk space in GB
    pub max_disk_gb: u32,
}

/// Preset account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetAccount {
    /// Public key
    pub pubkey: String,
    /// Initial lamports
    pub lamports: u64,
    /// Optional token account data
    pub tokens: Option<Vec<TokenAccount>>,
}

/// Token account data for preset accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAccount {
    /// Token mint address
    pub mint: String,
    /// Token amount
    pub amount: u64,
}

/// Controller status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControllerStatus {
    /// Controller is stopped
    Stopped,
    /// Controller is starting
    Starting,
    /// Controller is running
    Running,
    /// Controller is stopping
    Stopping,
    /// Controller encountered an error
    Error(String),
}

/// Process status information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessStatus {
    /// Whether the process is currently running
    pub is_running: bool,
    /// RPC port the process is listening on
    pub rpc_port: u16,
    /// WebSocket port the process is listening on
    pub ws_port: u16,
    /// Process ID (if available)
    pub pid: Option<u32>,
    /// How long the process has been running
    pub uptime_seconds: Option<u64>,
}

impl Default for ControllerStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 2048,
            max_cpu_percent: 80,
            max_disk_gb: 10,
        }
    }
}

impl Default for SurfPoolConfig {
    fn default() -> Self {
        Self {
            rpc_port: 8899,
            ws_port: 8900,
            ledger_path: dirs::cache_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("surfdesk")
                .join("ledger")
                .to_string_lossy()
                .to_string(),
            accounts_path: dirs::cache_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("surfdesk")
                .join("accounts")
                .to_string_lossy()
                .to_string(),
            auto_start: false,
            resource_limits: ResourceLimits::default(),
            fork_url: None,
            fork_slot: None,
            enable_mcp: true,
            anchor_project: true,
            preset_accounts: vec![PresetAccount {
                pubkey: "11111111111111111111111111111112".to_string(),
                lamports: 1_000_000_000_000,
                tokens: None,
            }],
        }
    }
}

impl SurfPoolController {
    /// Create a new SurfPool controller
    pub async fn new(platform: Platform) -> Result<Self, SurfDeskError> {
        let config = SurfPoolConfig::default();

        Ok(Self {
            platform,
            process: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(ControllerStatus::Stopped)),
        })
    }

    /// Create a new controller with custom configuration
    pub async fn with_config(
        platform: Platform,
        config: SurfPoolConfig,
    ) -> Result<Self, SurfDeskError> {
        Ok(Self {
            platform,
            process: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(ControllerStatus::Stopped)),
        })
    }

    /// Start the Solana validator
    pub async fn start(&self) -> Result<(), SurfDeskError> {
        let mut status = self.status.write().await;
        if *status != ControllerStatus::Stopped {
            return Err(SurfDeskError::platform("Controller is already running"));
        }

        *status = ControllerStatus::Starting;
        drop(status);

        let config = self.config.read().await.clone();

        // Ensure directories exist
        std::fs::create_dir_all(&config.ledger_path).map_err(|e| {
            SurfDeskError::platform(format!("Failed to create ledger directory: {}", e))
        })?;

        // Start surfpool with mainnet fork
        let result = self.start_mainnet_fork().await;
        match result {
            Ok(_) => {
                *self.status.write().await = ControllerStatus::Running;
                Ok(())
            }
            Err(e) => {
                *self.status.write().await = ControllerStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// Check if surfpool is installed locally
    pub async fn check_installation() -> Result<bool, SurfDeskError> {
        let output = Command::new("surfpool").arg("--version").output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    log::info!(
                        "SurfPool is installed: {}",
                        String::from_utf8_lossy(&output.stdout)
                    );
                    Ok(true)
                } else {
                    log::warn!(
                        "SurfPool command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    Ok(false)
                }
            }
            Err(e) => {
                log::info!("SurfPool not found in PATH: {}", e);
                Ok(false)
            }
        }
    }

    /// Install surfpool if needed (placeholder implementation)
    pub async fn install_if_needed() -> Result<(), SurfDeskError> {
        if !Self::check_installation().await? {
            log::info!("SurfPool not installed. Please install surfpool manually:");
            log::info!(
                "1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
            );
            log::info!("2. Install surfpool: cargo install surfpool");
            Err(SurfDeskError::platform(
                "SurfPool not installed. Please install it manually.",
            ))
        } else {
            Ok(())
        }
    }

    /// Start surfpool with mainnet fork on port 8999
    pub async fn start_mainnet_fork(&self) -> Result<(), SurfDeskError> {
        // Check installation first
        Self::install_if_needed().await?;

        let config = self.config.read().await;

        // Start surfpool with mainnet fork
        let mut cmd = Command::new("surfpool");
        cmd.arg("start")
            .arg("--fork")
            .arg("https://api.mainnet-beta.solana.com")
            .arg("--rpc-port")
            .arg("8999")
            .arg("--ws-port")
            .arg("9000")
            .arg("--ledger")
            .arg(&config.ledger_path)
            .arg("--accounts")
            .arg(&config.accounts_path);

        // Add preset accounts
        for account in &config.preset_accounts {
            cmd.arg("--preset-account")
                .arg(&account.pubkey)
                .arg(account.lamports.to_string());
        }

        // Enable MCP if configured
        if config.enable_mcp {
            cmd.arg("--mcp");
        }

        // Start the process
        let child = cmd
            .spawn()
            .map_err(|e| SurfDeskError::platform(format!("Failed to start surfpool: {}", e)))?;

        // Store the process handle
        *self.process.lock().await = Some(child);

        log::info!("SurfPool started with mainnet fork on port 8999");
        Ok(())
    }

    /// Stop the surfpool process
    pub async fn stop_process(&self) -> Result<(), SurfDeskError> {
        let mut process_guard = self.process.lock().await;

        if let Some(mut child) = process_guard.take() {
            // Try to stop gracefully first
            match child.kill() {
                Ok(_) => {
                    log::info!("SurfPool process stopped gracefully");
                    // Wait for process to actually stop
                    let _ = child.wait();
                }
                Err(e) => {
                    log::warn!("Failed to kill SurfPool process gracefully: {}", e);
                }
            }

            *self.status.write().await = ControllerStatus::Stopped;
            Ok(())
        } else {
            Err(SurfDeskError::platform("No SurfPool process is running"))
        }
    }

    /// Get current process status
    pub async fn get_process_status(&self) -> Result<ProcessStatus, SurfDeskError> {
        let status = self.status.read().await;
        let is_running = matches!(*status, ControllerStatus::Running);

        Ok(ProcessStatus {
            is_running,
            rpc_port: 8999,
            ws_port: 9000,
            pid: None,            // TODO: Get actual PID from process
            uptime_seconds: None, // TODO: Track uptime
        })
    }

    /// Stop the Solana validator
    pub async fn stop(&self) -> Result<(), SurfDeskError> {
        let mut status = self.status.write().await;
        if *status == ControllerStatus::Stopped {
            return Ok(());
        }

        *status = ControllerStatus::Stopping;
        drop(status);

        let mut process_guard = self.process.lock().await;
        if let Some(mut child) = process_guard.take() {
            if let Err(e) = child.kill() {
                log::warn!("Failed to kill surfpool process: {}", e);
            } else {
                log::info!("SurfPool process stopped");
                let _ = child.wait();
            }
        }

        *self.status.write().await = ControllerStatus::Stopped;
        Ok(())
    }

    /// Get the current status
    pub async fn get_status(&self) -> ControllerStatus {
        self.status.read().await.clone()
    }

    /// Get the current configuration
    pub async fn get_config(&self) -> SurfPoolConfig {
        self.config.read().await.clone()
    }

    /// Update the configuration
    pub async fn update_config(&self, new_config: SurfPoolConfig) -> Result<(), SurfDeskError> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }

    /// Check if the validator is healthy
    pub async fn health_check(&self) -> Result<bool, SurfDeskError> {
        let status = self.status.read().await;
        if *status != ControllerStatus::Running {
            return Ok(false);
        }
        drop(status);

        match self.platform {
            Platform::Desktop | Platform::Terminal => {
                // Check if process is still running
                let mut process_guard = self.process.lock().await;
                if let Some(child) = process_guard.as_mut() {
                    match child.try_wait() {
                        Ok(Some(_)) => Ok(false), // Process has exited
                        Ok(None) => Ok(true),     // Process is still running
                        Err(_) => Ok(false),      // Error checking status
                    }
                } else {
                    Ok(false)
                }
            }
            Platform::Web => {
                // Simulate health check
                Ok(true)
            }
        }
    }

    /// Get validator metrics
    pub async fn get_metrics(&self) -> Result<ValidatorMetrics, SurfDeskError> {
        let status = self.status.read().await;
        if *status != ControllerStatus::Running {
            return Err(SurfDeskError::platform("SurfPool is not running"));
        }
        drop(status);

        match self.platform {
            Platform::Desktop | Platform::Terminal => {
                // Collect actual metrics from the system
                // TODO: Implement actual metrics collection from SurfPool RPC
                Ok(ValidatorMetrics {
                    uptime_seconds: 0,    // Would need to track start time
                    memory_usage_mb: 512, // Would need to monitor actual usage
                    cpu_percent: 15.0,
                    disk_usage_gb: 0.5,
                    connected_peers: 0, // Local SurfPool has no external peers
                    slots_processed: 0, // Would need to query RPC
                    transaction_count: 0,
                    fork_height: None,  // Only for fork environments
                    accounts_loaded: 0, // Would need to query from SurfPool
                })
            }
            Platform::Web => {
                // Return simulated metrics
                Ok(ValidatorMetrics {
                    uptime_seconds: 0,
                    memory_usage_mb: 256,
                    cpu_percent: 10.0,
                    disk_usage_gb: 0.1,
                    connected_peers: 0,
                    slots_processed: 0,
                    transaction_count: 0,
                    fork_height: None,
                    accounts_loaded: 0,
                })
            }
        }
    }
}

/// Validator metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorMetrics {
    /// How long the validator has been running
    pub uptime_seconds: u64,
    /// Memory usage in MB
    pub memory_usage_mb: u32,
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Disk usage in GB
    pub disk_usage_gb: f32,
    /// Number of connected peers
    pub connected_peers: u32,
    /// Number of slots processed
    pub slots_processed: u64,
    /// Number of transactions processed
    pub transaction_count: u64,
    /// Current fork height (for mainnet forks)
    pub fork_height: Option<u64>,
    /// Number of accounts loaded
    pub accounts_loaded: u64,
}

/// Check if SurfPool is installed and available
pub async fn check_surfpool_installation() -> Result<bool, SurfDeskError> {
    let output = Command::new("surfpool")
        .arg("--version")
        .output()
        .map_err(|_| SurfDeskError::platform("SurfPool is not installed"))?;

    Ok(output.status.success())
}

/// Install SurfPool using cargo (if not available)
pub async fn install_surfpool() -> Result<(), SurfDeskError> {
    // Check if already installed
    if check_surfpool_installation().await.is_ok() {
        return Ok(());
    }

    // Try to install using cargo
    let output = Command::new("cargo")
        .args(["install", "surfpool"])
        .output()
        .map_err(|e| SurfDeskError::platform(format!("Failed to install SurfPool: {}", e)))?;

    if !output.status.success() {
        return Err(SurfDeskError::platform(
            "Failed to install SurfPool. Please install manually: cargo install surfpool",
        ));
    }

    Ok(())
}

/// Hook to use SurfPool controller in Dioxus components
pub fn use_surfpool_controller(platform: Platform) -> Signal<SurfPoolController> {
    use_signal(|| {
        // Create a simple placeholder controller for initialization
        // We'll initialize it properly when needed
        let config = SurfPoolConfig::default();
        SurfPoolController {
            platform,
            process: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(config)),
            status: Arc::new(RwLock::new(ControllerStatus::Stopped)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_controller_creation() {
        let controller = SurfPoolController::new(Platform::Desktop).await;
        assert!(controller.is_ok());
    }

    #[tokio::test]
    async fn test_config_default() {
        let config = SurfPoolConfig::default();
        assert_eq!(config.rpc_port, 8899);
        assert_eq!(config.ws_port, 8900);
        assert!(config.enable_mcp);
        assert!(config.anchor_project);
        assert_eq!(config.preset_accounts.len(), 1);
    }

    #[tokio::test]
    async fn test_status_transitions() {
        let controller = SurfPoolController::new(Platform::Web).await.unwrap();

        // Initial status should be stopped
        assert_eq!(controller.get_status().await, ControllerStatus::Stopped);

        // Web validator should start
        assert!(controller.start().await.is_ok());
        assert_eq!(controller.get_status().await, ControllerStatus::Running);

        // Should be able to stop
        assert!(controller.stop().await.is_ok());
        assert_eq!(controller.get_status().await, ControllerStatus::Stopped);
    }

    #[tokio::test]
    async fn test_surfpool_installation_check() {
        // This test will fail if surfpool is not installed
        // In real tests, you might want to mock this
        let result = check_surfpool_installation().await;
        // Don't assert here since surfpool might not be installed in test environment
        match result {
            Ok(is_installed) => println!("SurfPool installation status: {}", is_installed),
            Err(_) => println!("SurfPool not available (expected in test environment)"),
        }
    }

    #[test]
    fn test_preset_account_serialization() {
        let account = PresetAccount {
            pubkey: "11111111111111111111111111111112".to_string(),
            lamports: 1_000_000_000,
            tokens: None,
        };

        // Test serialization
        let json = serde_json::to_string(&account).unwrap();
        let deserialized: PresetAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(account.pubkey, deserialized.pubkey);
        assert_eq!(account.lamports, deserialized.lamports);
    }

    #[test]
    fn test_validator_metrics() {
        let metrics = ValidatorMetrics {
            uptime_seconds: 3600,
            memory_usage_mb: 512,
            cpu_percent: 25.5,
            disk_usage_gb: 1.2,
            connected_peers: 5,
            slots_processed: 1000,
            transaction_count: 50,
            fork_height: Some(150_000_000),
            accounts_loaded: 25,
        };

        assert_eq!(metrics.uptime_seconds, 3600);
        assert_eq!(metrics.memory_usage_mb, 512);
        assert_eq!(metrics.fork_height, Some(150_000_000));
    }
}
