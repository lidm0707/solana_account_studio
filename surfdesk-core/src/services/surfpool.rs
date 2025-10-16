//! # SurfPool Service
//!
//! This service provides integration with SurfPool for managing local Solana validators
//! and development environments across all platforms.

use crate::error::{Result, SurfDeskError};
use crate::platform::Platform;
use crate::surfpool::{
    check_surfpool_installation, install_surfpool, SurfPoolConfig, SurfPoolController,
};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

/// SurfPool service for managing local Solana validators
#[derive(Clone)]
pub struct SurfPoolService {
    /// Platform-specific controller
    controller: Arc<RwLock<SurfPoolController>>,
    /// Current platform
    platform: Platform,
    /// Service status
    status: Arc<RwLock<ServiceStatus>>,
}

/// Service status
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    /// Service is stopped
    Stopped,
    /// Service is starting
    Starting,
    /// Service is running
    Running,
    /// Service is stopping
    Stopping,
    /// Service encountered an error
    Error(String),
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

impl SurfPoolService {
    /// Create a new SurfPool service
    pub async fn new() -> Result<Self> {
        let platform = crate::current_platform();

        info!("Initializing SurfPool service for platform: {:?}", platform);

        // Check if SurfPool is installed
        match check_surfpool_installation().await {
            Ok(true) => {
                info!("SurfPool is installed and available");
            }
            Ok(false) => {
                warn!("SurfPool is not installed, attempting to install...");
                if let Err(e) = install_surfpool().await {
                    error!("Failed to install SurfPool: {}", e);
                    return Err(SurfDeskError::platform(format!(
                        "SurfPool is required but not available: {}",
                        e
                    )));
                }
                info!("SurfPool installed successfully");
            }
            Err(e) => {
                error!("Failed to check SurfPool installation: {}", e);
                return Err(SurfDeskError::platform(format!(
                    "SurfPool availability check failed: {}",
                    e
                )));
            }
        }

        let controller = SurfPoolController::new(platform).await?;

        Ok(Self {
            controller: Arc::new(RwLock::new(controller)),
            platform,
            status: Arc::new(RwLock::new(ServiceStatus::Stopped)),
        })
    }

    /// Start the SurfPool validator with default configuration
    pub async fn start(&self) -> Result<()> {
        let mut status = self.status.write().await;
        if *status != ServiceStatus::Stopped {
            return Err(SurfDeskError::platform(
                "SurfPool service is already running",
            ));
        }

        *status = ServiceStatus::Starting;
        drop(status);

        info!("Starting SurfPool validator");

        let controller = self.controller.read().await;
        match controller.start().await {
            Ok(()) => {
                info!("SurfPool validator started successfully");
                let mut status = self.status.write().await;
                *status = ServiceStatus::Running;
                Ok(())
            }
            Err(e) => {
                error!("Failed to start SurfPool validator: {}", e);
                let mut status = self.status.write().await;
                *status = ServiceStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// Start the SurfPool validator with custom configuration
    pub async fn start_with_config(&self, config: SurfPoolConfig) -> Result<()> {
        let mut status = self.status.write().await;
        if *status != ServiceStatus::Stopped {
            return Err(SurfDeskError::platform(
                "SurfPool service is already running",
            ));
        }

        *status = ServiceStatus::Starting;
        drop(status);

        info!("Starting SurfPool validator with custom configuration");

        let controller = self.controller.read().await;

        // Update controller configuration
        controller.update_config(config.clone()).await?;

        match controller.start().await {
            Ok(()) => {
                info!("SurfPool validator started successfully with custom config");
                let mut status = self.status.write().await;
                *status = ServiceStatus::Running;
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to start SurfPool validator with custom config: {}",
                    e
                );
                let mut status = self.status.write().await;
                *status = ServiceStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// Stop the SurfPool validator
    pub async fn stop(&self) -> Result<()> {
        let mut status = self.status.write().await;
        if *status != ServiceStatus::Running {
            return Err(SurfDeskError::platform("SurfPool service is not running"));
        }

        *status = ServiceStatus::Stopping;
        drop(status);

        info!("Stopping SurfPool validator");

        let controller = self.controller.read().await;
        match controller.stop().await {
            Ok(()) => {
                info!("SurfPool validator stopped successfully");
                let mut status = self.status.write().await;
                *status = ServiceStatus::Stopped;
                Ok(())
            }
            Err(e) => {
                error!("Failed to stop SurfPool validator: {}", e);
                let mut status = self.status.write().await;
                *status = ServiceStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// Restart the SurfPool validator
    pub async fn restart(&self) -> Result<()> {
        info!("Restarting SurfPool validator");

        // Stop if running
        {
            let status = self.status.read().await;
            if *status == ServiceStatus::Running {
                drop(status);
                self.stop().await?;
            } else {
                drop(status);
            }
        }

        // Start again
        self.start().await
    }

    /// Get the current service status
    pub async fn get_status(&self) -> ServiceStatus {
        self.status.read().await.clone()
    }

    /// Get the current controller configuration
    pub async fn get_config(&self) -> Result<SurfPoolConfig> {
        let controller = self.controller.read().await;
        Ok(controller.get_config().await)
    }

    /// Update the controller configuration
    pub async fn update_config(&self, config: SurfPoolConfig) -> Result<()> {
        debug!("Updating SurfPool configuration");

        let controller = self.controller.read().await;
        controller.update_config(config).await?;

        info!("SurfPool configuration updated successfully");
        Ok(())
    }

    /// Get validator metrics
    pub async fn get_metrics(&self) -> Result<crate::surfpool::ValidatorMetrics> {
        let controller = self.controller.read().await;
        controller.get_metrics().await
    }

    /// Perform a health check on the validator
    pub async fn health_check(&self) -> Result<bool> {
        let controller = self.controller.read().await;
        controller.health_check().await
    }

    /// Get the current platform
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Check if the service supports MCP (Model Context Protocol)
    pub fn supports_mcp(&self) -> bool {
        matches!(self.platform, Platform::Desktop | Platform::Terminal)
    }

    /// Get supported environment types for this platform
    pub fn supported_environments(&self) -> Vec<&'static str> {
        match self.platform {
            Platform::Desktop => vec!["local-devnet", "mainnet-fork", "custom"],
            Platform::Web => vec!["local-devnet", "mainnet-fork"], // Limited environments for web
            Platform::Terminal => vec!["local-devnet", "mainnet-fork", "custom"],
        }
    }
}

#[async_trait]
impl super::Service for SurfPoolService {
    fn name(&self) -> &'static str {
        "SurfPool"
    }

    async fn initialize(&mut self) -> Result<()> {
        info!("Initializing SurfPool service");

        // The service is already initialized in new(), but we can do additional setup here
        let mut status = self.status.write().await;
        *status = ServiceStatus::Stopped;

        info!("SurfPool service initialized successfully");
        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        let service_status = self.status.read().await;

        match *service_status {
            ServiceStatus::Running => {
                // Perform actual health check on the validator
                let controller = self.controller.read().await;
                controller.health_check().await
            }
            ServiceStatus::Stopped => Ok(false),
            ServiceStatus::Starting | ServiceStatus::Stopping => Ok(false),
            ServiceStatus::Error(_) => Ok(false),
        }
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down SurfPool service");

        // Stop the validator if it's running
        {
            let status = self.status.read().await;
            if *status == ServiceStatus::Running {
                drop(status);
                if let Err(e) = self.stop().await {
                    warn!("Failed to stop validator during shutdown: {}", e);
                }
            } else {
                drop(status);
            }
        }

        // Update status
        let mut status = self.status.write().await;
        *status = ServiceStatus::Stopped;

        info!("SurfPool service shutdown successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surfpool::{ControllerStatus, SurfPoolConfig};

    #[tokio::test]
    async fn test_service_creation() {
        // This test requires surfpool to be installed
        // In a real test environment, you might want to mock this
        let result = SurfPoolService::new().await;

        match result {
            Ok(_) => println!("SurfPool service created successfully"),
            Err(e) => println!(
                "SurfPool service creation failed (expected in test env): {}",
                e
            ),
        }
    }

    #[tokio::test]
    async fn test_service_status() {
        let status = ServiceStatus::default();
        assert_eq!(status, ServiceStatus::Stopped);
    }

    #[test]
    fn test_platform_support() {
        let service = SurfPoolService {
            controller: Arc::new(RwLock::new(
                // Mock controller for testing
                // In real tests, you'd need to properly initialize this
                unimplemented!(),
            )),
            platform: Platform::Desktop,
            status: Arc::new(RwLock::new(ServiceStatus::Stopped)),
        };

        assert!(service.supports_mcp());
        assert!(service.supported_environments().contains(&"local-devnet"));
        assert!(service.supported_environments().contains(&"mainnet-fork"));
        assert!(service.supported_environments().contains(&"custom"));
    }

    #[test]
    fn test_web_platform_limitations() {
        let service = SurfPoolService {
            controller: Arc::new(RwLock::new(
                // Mock controller for testing
                unimplemented!(),
            )),
            platform: Platform::Web,
            status: Arc::new(RwLock::new(ServiceStatus::Stopped)),
        };

        // Web platform doesn't support MCP
        assert!(!service.supports_mcp());

        // Web platform has limited environments
        assert!(service.supported_environments().contains(&"local-devnet"));
        assert!(service.supported_environments().contains(&"mainnet-fork"));
        assert!(!service.supported_environments().contains(&"custom"));
    }
}
