//! # Services Module
//!
//! This module contains all the service layer components for SurfDesk.
//! Services provide the business logic and external integrations that
//! power the application across all platforms.

pub mod config;
pub mod events;
pub mod logger;

#[cfg(feature = "solana")]
pub mod solana;

#[cfg(feature = "database")]
pub mod database;

pub mod surfpool;

use crate::error::Result;

/// Service manager for coordinating all services
pub struct ServiceManager {
    /// Configuration service
    config_service: config::ConfigService,
    /// Logger service
    logger_service: logger::LoggerService,
    /// Event service
    event_service: events::EventService,
    /// Solana service (optional)
    #[cfg(feature = "solana")]
    solana_service: Option<solana::SolanaService>,
    /// Database service (optional)
    #[cfg(feature = "database")]
    database_service: Option<database::DatabaseService>,
    /// SurfPool service
    surfpool_service: Option<surfpool::SurfPoolService>,
}

impl ServiceManager {
    /// Create a new service manager
    pub async fn new() -> Result<Self> {
        // Initialize services in dependency order
        let config_service = config::ConfigService::new().await?;
        let logger_service = logger::LoggerService::new(&config_service)?;
        let event_service = events::EventService::new()?;

        log::info!("Service manager initialized");

        Ok(Self {
            config_service,
            logger_service,
            event_service,
            #[cfg(feature = "solana")]
            solana_service: None,
            #[cfg(feature = "database")]
            database_service: None,
            surfpool_service: None,
        })
    }

    /// Initialize all services
    pub async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing all services");

        // Initialize database if enabled
        #[cfg(feature = "database")]
        {
            self.database_service =
                Some(database::DatabaseService::new(&self.config_service).await?);
            log::info!("Database service initialized");
        }

        // Initialize Solana service if enabled
        #[cfg(feature = "solana")]
        {
            let rpc_url = self.config_service.get_solana_rpc_url().await?;
            self.solana_service = Some(solana::SolanaService::new(rpc_url).await?);
            log::info!("Solana service initialized");
        }

        // Initialize SurfPool service
        {
            self.surfpool_service = Some(surfpool::SurfPoolService::new().await?);
            log::info!("SurfPool service initialized");
        }

        log::info!("All services initialized successfully");
        Ok(())
    }

    /// Get the configuration service
    pub fn config_service(&self) -> &config::ConfigService {
        &self.config_service
    }

    /// Get the logger service
    pub fn logger_service(&self) -> &logger::LoggerService {
        &self.logger_service
    }

    /// Get the event service
    pub fn event_service(&self) -> &events::EventService {
        &self.event_service
    }

    /// Get the Solana service
    #[cfg(feature = "solana")]
    pub fn solana_service(&self) -> Option<&solana::SolanaService> {
        self.solana_service.as_ref()
    }

    /// Get the database service
    #[cfg(feature = "database")]
    pub fn database_service(&self) -> Option<&database::DatabaseService> {
        self.database_service.as_ref()
    }

    /// Get the SurfPool service
    pub fn surfpool_service(&self) -> Option<&surfpool::SurfPoolService> {
        self.surfpool_service.as_ref()
    }

    /// Shutdown all services
    pub async fn shutdown(&self) -> Result<()> {
        log::info!("Shutting down services");

        // Shutdown services in reverse order
        {
            if let Some(ref service) = self.surfpool_service {
                service.shutdown().await?;
                log::info!("SurfPool service shutdown");
            }
        }

        #[cfg(feature = "solana")]
        {
            if let Some(ref service) = self.solana_service {
                service.shutdown().await?;
                log::info!("Solana service shutdown");
            }
        }

        #[cfg(feature = "database")]
        {
            if let Some(ref service) = self.database_service {
                service.shutdown().await?;
                log::info!("Database service shutdown");
            }
        }

        self.event_service.shutdown().await?;
        log::info!("Event service shutdown");

        log::info!("All services shutdown successfully");
        Ok(())
    }
}

/// Service trait for common service operations
#[async_trait::async_trait]
pub trait Service {
    /// Service name
    fn name(&self) -> &'static str;

    /// Initialize the service
    async fn initialize(&mut self) -> Result<()>;

    /// Check if the service is healthy
    async fn health_check(&self) -> Result<bool>;

    /// Shutdown the service
    async fn shutdown(&self) -> Result<()>;
}

/// Service health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceHealth {
    /// Service is healthy
    Healthy,
    /// Service is unhealthy
    Unhealthy,
    /// Service status unknown
    Unknown,
}

impl ServiceHealth {
    /// Get display name for the health status
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Healthy => "Healthy",
            Self::Unhealthy => "Unhealthy",
            Self::Unknown => "Unknown",
        }
    }

    /// Get CSS class for styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Healthy => "health-healthy",
            Self::Unhealthy => "health-unhealthy",
            Self::Unknown => "health-unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_creation() {
        let result = ServiceManager::new().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_manager_initialization() {
        let mut manager = ServiceManager::new().await.unwrap();
        let result = manager.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_manager_shutdown() {
        let mut manager = ServiceManager::new().await.unwrap();
        manager.initialize().await.unwrap();
        let result = manager.shutdown().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_health() {
        assert_eq!(ServiceHealth::Healthy.display_name(), "Healthy");
        assert_eq!(ServiceHealth::Healthy.css_class(), "health-healthy");

        assert_eq!(ServiceHealth::Unhealthy.display_name(), "Unhealthy");
        assert_eq!(ServiceHealth::Unhealthy.css_class(), "health-unhealthy");

        assert_eq!(ServiceHealth::Unknown.display_name(), "Unknown");
        assert_eq!(ServiceHealth::Unknown.css_class(), "health-unknown");
    }
}
