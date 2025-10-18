//! # SurfPool Service - Terminal Strategy
//!
//! This service provides integration with SurfPool using terminal commands.
//! Instead of complex service integration, we use simple command-line calls
//! to manage the external SurfPool process using Dioxus signals for
//! single-threaded compatibility.

use crate::error::{Result, SurfDeskError};
use crate::Platform;
use async_trait::async_trait;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command as TokioCommand;

/// SurfPool service using terminal command strategy
#[derive(Debug, Clone)]
pub struct SurfPoolService {
    pub platform: Platform,
    /// Service status
    pub status: ServiceStatus,
    /// Current configuration
    pub config: SurfPoolConfig,
}

/// Service status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ServiceStatus {
    /// Service is stopped
    #[default]
    Stopped,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is stopping
    Stopping,
    /// Service encountered an error
    Error(String),
    /// Service status is unknown
    Unknown,
}

/// Simple SurfPool configuration for terminal strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurfPoolConfig {
    /// RPC port for the validator
    pub rpc_port: u16,
    /// WebSocket port for the validator
    pub ws_port: u16,
    /// Fork URL for mainnet forking
    pub fork_url: Option<String>,
    /// Enable MCP (Model Context Protocol)
    pub enable_mcp: bool,
}

impl Default for SurfPoolConfig {
    fn default() -> Self {
        Self {
            rpc_port: 8899,
            ws_port: 8900,
            fork_url: None,
            enable_mcp: true,
        }
    }
}

/// SurfPool process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfPoolProcess {
    /// Process ID
    pub pid: Option<u32>,
    /// RPC port
    pub rpc_port: u16,
    /// WebSocket port
    pub ws_port: u16,
    /// Status
    pub status: String,
    /// Start time
    pub start_time: Option<String>,
}

impl SurfPoolService {
    /// Create a new SurfPool service with terminal strategy
    pub async fn new() -> Result<Self> {
        let platform = crate::current_platform();

        info!(
            "Initializing SurfPool service (terminal strategy) for platform: {:?}",
            platform
        );

        // Check if SurfPool is installed
        if !Self::is_surfpool_installed().await? {
            warn!("SurfPool is not installed");
            return Err(SurfDeskError::platform(
                "SurfPool is not installed. Install with: cargo install surfpool",
            ));
        }

        info!("SurfPool is available via terminal");

        Ok(Self {
            platform,
            status: ServiceStatus::Stopped,
            config: SurfPoolConfig::default(),
        })
    }

    /// Create a fallback SurfPool service for when SurfPool is not installed
    pub fn new_fallback() -> Self {
        warn!("Creating fallback SurfPool service - limited functionality");
        Self {
            platform: crate::current_platform(),
            status: ServiceStatus::Error("SurfPool not installed".to_string()),
            config: SurfPoolConfig::default(),
        }
    }

    /// Check if SurfPool is installed by trying to run --version
    async fn is_surfpool_installed() -> Result<bool> {
        let output = TokioCommand::new("surfpool")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Start SurfPool using terminal command
    pub async fn start(&mut self) -> Result<()> {
        // Check if this is a fallback service
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot start SurfPool: not installed. Install with: cargo install surfpool",
            ));
        }

        if self.status != ServiceStatus::Stopped {
            return Err(SurfDeskError::platform(
                "SurfPool service is already running",
            ));
        }

        self.status = ServiceStatus::Starting;
        info!("Starting SurfPool with terminal strategy");

        let config = self.config.clone();

        // Build the surfpool command
        let mut cmd = TokioCommand::new("surfpool");
        cmd.arg("start");
        cmd.arg("--rpc-port");
        cmd.arg(config.rpc_port.to_string());
        cmd.arg("--ws-port");
        cmd.arg(config.ws_port.to_string());

        if config.enable_mcp {
            cmd.arg("--enable-mcp");
        }

        if let Some(fork_url) = &config.fork_url {
            cmd.arg("--fork");
            cmd.arg(fork_url);
        }

        // Start the process in the background
        match cmd.spawn() {
            Ok(_child) => {
                self.status = ServiceStatus::Running;
                info!(
                    "SurfPool started successfully on ports {} (RPC) and {} (WS)",
                    config.rpc_port, config.ws_port
                );
                Ok(())
            }
            Err(e) => {
                self.status = ServiceStatus::Error(format!("Failed to start SurfPool: {}", e));
                error!("Failed to start SurfPool: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to start SurfPool: {}",
                    e
                )))
            }
        }
    }

    /// Stop SurfPool using terminal command
    pub async fn stop(&mut self) -> Result<()> {
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot stop SurfPool: not installed",
            ));
        }

        if self.status == ServiceStatus::Stopped {
            return Ok(());
        }

        self.status = ServiceStatus::Stopping;
        info!("Stopping SurfPool with terminal strategy");

        match TokioCommand::new("surfpool").arg("stop").output().await {
            Ok(output) => {
                if output.status.success() {
                    self.status = ServiceStatus::Stopped;
                    info!("SurfPool stopped successfully");
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.status =
                        ServiceStatus::Error(format!("Failed to stop SurfPool: {}", stderr));
                    error!("Failed to stop SurfPool: {}", stderr);
                    Err(SurfDeskError::platform(format!(
                        "Failed to stop SurfPool: {}",
                        stderr
                    )))
                }
            }
            Err(e) => {
                self.status = ServiceStatus::Error(format!("Failed to stop SurfPool: {}", e));
                error!("Failed to stop SurfPool: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to stop SurfPool: {}",
                    e
                )))
            }
        }
    }

    /// Get current status
    pub async fn get_status(&self) -> ServiceStatus {
        self.status.clone()
    }

    /// Get current configuration
    pub async fn get_config(&self) -> SurfPoolConfig {
        self.config.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: SurfPoolConfig) -> Result<()> {
        if self.status == ServiceStatus::Running {
            return Err(SurfDeskError::platform(
                "Cannot update configuration while SurfPool is running",
            ));
        }

        self.config = config;
        info!("SurfPool configuration updated");
        Ok(())
    }

    /// Get process information
    pub async fn get_process_info(&self) -> Result<SurfPoolProcess> {
        if self.is_fallback() {
            return Ok(SurfPoolProcess {
                pid: None,
                rpc_port: self.config.rpc_port,
                ws_port: self.config.ws_port,
                status: "Not installed".to_string(),
                start_time: None,
            });
        }

        match TokioCommand::new("surfpool").arg("status").output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let status_str = if output.status.success() {
                    "Running".to_string()
                } else {
                    "Stopped".to_string()
                };

                Ok(SurfPoolProcess {
                    pid: None, // Parse from output if needed
                    rpc_port: self.config.rpc_port,
                    ws_port: self.config.ws_port,
                    status: status_str,
                    start_time: None, // Parse from output if needed
                })
            }
            Err(e) => {
                error!("Failed to get SurfPool status: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to get status: {}",
                    e
                )))
            }
        }
    }

    /// Check if this is a fallback service
    pub fn is_fallback(&self) -> bool {
        matches!(self.status, ServiceStatus::Error(ref msg) if msg.contains("not installed"))
    }

    /// Generate a new keypair using SurfPool
    pub async fn generate_keypair(&self) -> Result<String> {
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot generate keypair: SurfPool not installed",
            ));
        }

        match TokioCommand::new("surfpool").arg("keygen").output().await {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let pubkey = stdout.trim().to_string();
                    info!("Generated new keypair: {}", pubkey);
                    Ok(pubkey)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to generate keypair: {}", stderr);
                    Err(SurfDeskError::platform(format!(
                        "Failed to generate keypair: {}",
                        stderr
                    )))
                }
            }
            Err(e) => {
                error!("Failed to run keygen command: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to run keygen: {}",
                    e
                )))
            }
        }
    }

    /// Deploy a program using SurfPool
    pub async fn deploy_program(&self, program_path: &str) -> Result<String> {
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot deploy program: SurfPool not installed",
            ));
        }

        match TokioCommand::new("surfpool")
            .arg("deploy")
            .arg(program_path)
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let program_id = stdout.trim().to_string();
                    info!("Deployed program: {}", program_id);
                    Ok(program_id)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to deploy program: {}", stderr);
                    Err(SurfDeskError::platform(format!(
                        "Failed to deploy program: {}",
                        stderr
                    )))
                }
            }
            Err(e) => {
                error!("Failed to run deploy command: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to run deploy: {}",
                    e
                )))
            }
        }
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &str) -> Result<f64> {
        if self.is_fallback() {
            // Return a simulated balance for fallback mode
            return Ok(1000000.0); // 1 SOL in lamports
        }

        match TokioCommand::new("surfpool")
            .arg("balance")
            .arg(pubkey)
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let balance_str = stdout.trim();
                    let balance = balance_str.parse::<f64>().map_err(|e| {
                        SurfDeskError::platform(format!("Invalid balance format: {}", e))
                    })?;
                    Ok(balance)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to get balance: {}", stderr);
                    Err(SurfDeskError::platform(format!(
                        "Failed to get balance: {}",
                        stderr
                    )))
                }
            }
            Err(e) => {
                error!("Failed to run balance command: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to run balance: {}",
                    e
                )))
            }
        }
    }

    /// Airdrop SOL to an account
    pub async fn airdrop(&self, pubkey: &str, amount: f64) -> Result<String> {
        if self.is_fallback() {
            return Err(SurfDeskError::platform(
                "Cannot airdrop: SurfPool not installed",
            ));
        }

        match TokioCommand::new("surfpool")
            .arg("airdrop")
            .arg(pubkey)
            .arg(amount.to_string())
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let signature = stdout.trim().to_string();
                    info!("Airdropped {} SOL to {}: {}", amount, pubkey, signature);
                    Ok(signature)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("Failed to airdrop: {}", stderr);
                    Err(SurfDeskError::platform(format!(
                        "Failed to airdrop: {}",
                        stderr
                    )))
                }
            }
            Err(e) => {
                error!("Failed to run airdrop command: {}", e);
                Err(SurfDeskError::platform(format!(
                    "Failed to run airdrop: {}",
                    e
                )))
            }
        }
    }

    /// Shutdown the service
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down SurfPool service");

        if self.status == ServiceStatus::Running {
            self.stop().await?;
        }

        info!("SurfPool service shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surfpool_service_creation() {
        // This test will likely fail unless surfpool is installed
        // In a real test environment, you'd mock the command execution
        let result = SurfPoolService::new().await;

        // We can't guarantee surfpool is installed, so we just test the fallback
        if result.is_err() {
            let fallback = SurfPoolService::new_fallback();
            assert!(fallback.is_fallback());
        }
    }

    #[tokio::test]
    async fn test_surfpool_config() {
        let config = SurfPoolConfig {
            rpc_port: 8999,
            ws_port: 9000,
            fork_url: Some("https://api.mainnet-beta.solana.com".to_string()),
            enable_mcp: false,
        };

        assert_eq!(config.rpc_port, 8999);
        assert_eq!(config.ws_port, 9000);
        assert!(config.fork_url.is_some());
        assert!(!config.enable_mcp);
    }

    #[tokio::test]
    async fn test_service_status() {
        let service = SurfPoolService::new_fallback();
        let status = service.get_status().await;
        assert!(matches!(status, ServiceStatus::Error(_)));
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut service = SurfPoolService::new_fallback();
        let new_config = SurfPoolConfig {
            rpc_port: 9999,
            ws_port: 10000,
            fork_url: None,
            enable_mcp: true,
        };

        let result = service.update_config(new_config.clone()).await;
        assert!(result.is_ok());

        let current_config = service.get_config().await;
        assert_eq!(current_config.rpc_port, 9999);
        assert_eq!(current_config.ws_port, 10000);
        assert!(current_config.enable_mcp);
    }

    #[tokio::test]
    async fn test_fallback_balance() {
        let service = SurfPoolService::new_fallback();
        let balance = service
            .get_balance("11111111111111111111111111111111")
            .await;
        assert!(balance.is_ok());
        assert_eq!(balance.unwrap(), 1000000.0);
    }
}
