//! # Application State Management Module
//!
//! This module provides comprehensive state management for the SurfDesk application.
//! It uses Dioxus signals for reactive state management across all platforms and
//! provides a centralized store for application data without Arc/RwLock for
//! single-threaded Dioxus compatibility.

use crate::{
    error::Result,
    services::{monitoring::MonitoringService, surfpool::SurfPoolService},
    types::*,
};
use dioxus::prelude::*;

/// UI state for components
#[derive(Debug, Clone)]
pub struct UIState {
    pub theme: Theme,
    pub sidebar: SidebarState,
    pub main_content: MainContentState,
    pub modal: Option<ModalState>,
    pub notifications: Vec<Notification>,
}

/// Main application state structure
///
/// This structure holds all the application state that needs to be shared
/// across components and platforms. It uses Dioxus signals for reactive updates
/// without Arc/RwLock for single-threaded compatibility.
#[derive(Debug)]
pub struct AppState {
    /// Current projects
    pub projects: Vec<Project>,
    /// Active project ID
    pub active_project_id: Option<ProjectId>,
    /// Environments for the active project
    pub environments: Vec<Environment>,
    /// Active environment ID
    pub active_environment_id: Option<EnvironmentId>,
    /// Accounts in the active environment
    pub accounts: Vec<Account>,
    /// Transactions in the active environment
    pub transactions: Vec<Transaction>,
    /// Monitoring service instance (stored directly, not in Arc)
    pub monitoring_service: Option<MonitoringService>,
    /// Monitoring statistics
    pub monitoring_stats: MonitoringStats,
    /// SurfPool service instance (stored directly, not in Arc)
    pub surfpool_service: Option<SurfPoolService>,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// Current network
    pub current_network: SolanaNetwork,
    /// Loading states
    pub loading: LoadingState,
    /// Error state
    pub error: Option<crate::error::SurfDeskError>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            active_project_id: None,
            environments: Vec::new(),
            active_environment_id: None,
            accounts: Vec::new(),
            transactions: Vec::new(),
            monitoring_service: None,
            monitoring_stats: MonitoringStats::default(),
            surfpool_service: None,
            connection_status: ConnectionStatus::Disconnected,
            current_network: SolanaNetwork::Mainnet,
            loading: LoadingState::default(),
            error: None,
        }
    }
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize the application state with services
    pub async fn initialize(&mut self) -> Result<()> {
        log::info!("Initializing application state");

        // Initialize monitoring service
        let monitoring_service = MonitoringService::new().await?;
        self.monitoring_service = Some(monitoring_service);

        // Initialize SurfPool service
        match SurfPoolService::new().await {
            Ok(surfpool_service) => {
                self.surfpool_service = Some(surfpool_service);
                log::info!("SurfPool service initialized");
            }
            Err(e) => {
                log::warn!("Failed to initialize SurfPool service: {}", e);
                // Create fallback service
                let fallback = SurfPoolService::new_fallback();
                self.surfpool_service = Some(fallback);
                log::info!("Using fallback SurfPool service");
            }
        }

        // Set initial connection status
        self.connection_status = ConnectionStatus::Disconnected;

        log::info!("Application state initialized");
        Ok(())
    }

    /// Get the monitoring service
    pub fn monitoring_service(&self) -> Option<&MonitoringService> {
        self.monitoring_service.as_ref()
    }

    /// Get mutable reference to monitoring service
    pub fn monitoring_service_mut(&mut self) -> Option<&mut MonitoringService> {
        self.monitoring_service.as_mut()
    }

    /// Get the SurfPool service
    pub fn surfpool_service(&self) -> Option<&SurfPoolService> {
        self.surfpool_service.as_ref()
    }

    /// Get mutable reference to SurfPool service
    pub fn surfpool_service_mut(&mut self) -> Option<&mut SurfPoolService> {
        self.surfpool_service.as_mut()
    }

    /// Update monitoring statistics
    pub fn update_monitoring_stats(&mut self, stats: MonitoringStats) {
        self.monitoring_stats = stats;
    }

    /// Update connection status
    pub fn update_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status = status;
    }

    /// Set the current network
    pub fn set_network(&mut self, network: SolanaNetwork) {
        self.current_network = network;
    }

    /// Add a project
    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    /// Remove a project
    pub fn remove_project(&mut self, project_id: &ProjectId) {
        self.projects.retain(|p| p.id != *project_id);
    }

    /// Set the active project
    pub fn set_active_project(&mut self, project_id: Option<ProjectId>) {
        self.active_project_id = project_id;
    }

    /// Add an environment
    pub fn add_environment(&mut self, environment: Environment) {
        self.environments.push(environment);
    }

    /// Remove an environment
    pub fn remove_environment(&mut self, environment_id: &EnvironmentId) {
        self.environments.retain(|e| e.id != *environment_id);
    }

    /// Set the active environment
    pub fn set_active_environment(&mut self, environment_id: Option<EnvironmentId>) {
        self.active_environment_id = environment_id;
    }

    /// Add an account
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    /// Remove an account
    pub fn remove_account(&mut self, account_id: &AccountId) {
        self.accounts.retain(|a| a.id != *account_id);
    }

    /// Update an account
    pub fn update_account(&mut self, account_id: &AccountId, account: Account) {
        if let Some(pos) = self.accounts.iter().position(|a| a.id == *account_id) {
            self.accounts[pos] = account;
        }
    }

    /// Add a transaction
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: LoadingState) {
        self.loading = loading;
    }

    /// Set error state
    pub fn set_error(&mut self, error: Option<crate::error::SurfDeskError>) {
        self.error = error;
    }

    /// Clear error state
    pub fn clear_error(&mut self) {
        self.error = None;
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.loading.is_loading()
    }

    /// Get current error
    pub fn get_error(&self) -> Option<&crate::error::SurfDeskError> {
        self.error.as_ref()
    }

    /// Get active project
    pub fn get_active_project(&self) -> Option<Project> {
        self.active_project_id
            .and_then(|id| self.projects.iter().find(|p| p.id == id))
            .cloned()
    }

    /// Get active environment
    pub fn get_active_environment(&self) -> Option<Environment> {
        self.active_environment_id
            .and_then(|id| self.environments.iter().find(|e| e.id == id))
            .cloned()
    }

    /// Get accounts for active environment
    pub fn get_active_accounts(&self) -> Vec<Account> {
        let active_env_id = self.active_environment_id;
        self.accounts
            .iter()
            .filter(|a| a.environment_id == active_env_id.unwrap_or_default())
            .cloned()
            .collect()
    }

    /// Get transactions for active environment
    pub fn get_active_transactions(&self) -> Vec<Transaction> {
        let active_env_id = self.active_environment_id;
        self.transactions
            .iter()
            .filter(|t| t.environment_id == active_env_id.unwrap_or_default())
            .cloned()
            .collect()
    }

    /// Refresh data for active environment
    pub async fn refresh_active_environment(&mut self) -> Result<()> {
        self.set_loading(LoadingState::loading("Refreshing data..."));

        // Clear any existing errors
        self.clear_error();

        // In a real implementation, this would fetch data from the backend
        // For now, we'll just simulate a delay
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        self.set_loading(LoadingState::idle());
        Ok(())
    }

    /// Search accounts by label or pubkey
    pub fn search_accounts(&self, query: &str) -> Vec<Account> {
        let query_lower = query.to_lowercase();

        self.accounts
            .iter()
            .filter(|account| {
                // Note: Account structure may need to be updated based on actual fields
                // This is a placeholder that will need adjustment
                account
                    .pubkey
                    .to_string()
                    .to_lowercase()
                    .contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Search transactions by signature or memo
    pub fn search_transactions(&self, query: &str) -> Vec<Transaction> {
        let query_lower = query.to_lowercase();

        self.transactions
            .iter()
            .filter(|transaction| transaction.signature.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    /// Get statistics for the active environment
    pub fn get_environment_stats(&self) -> EnvironmentStats {
        let accounts = self.get_active_accounts();
        let transactions = self.get_active_transactions();

        let total_balance: f64 = accounts.iter().map(|a| a.balance as f64).sum();
        let active_accounts = accounts.len();
        let total_transactions = transactions.len();

        EnvironmentStats {
            total_balance,
            active_accounts,
            total_transactions,
            last_updated: chrono::Utc::now(),
        }
    }

    /// Backup current state
    /// Backup state
    pub fn backup_state(&self) -> StateBackup {
        StateBackup {
            projects: self.projects.clone(),
            active_project_id: self.active_project_id,
            environments: self.environments.clone(),
            active_environment_id: self.active_environment_id,
            accounts: self.accounts.clone(),
            transactions: self.transactions.clone(),
            current_network: self.current_network,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Restore state
    pub fn restore_state(&mut self, backup: StateBackup) {
        self.projects = backup.projects;
        self.active_project_id = backup.active_project_id;
        self.environments = backup.environments;
        self.active_environment_id = backup.active_environment_id;
        self.accounts = backup.accounts;
        self.transactions = backup.transactions;
        self.current_network = backup.current_network;
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.projects = Vec::new();
        self.active_project_id = None;
        self.environments = Vec::new();
        self.active_environment_id = None;
        self.accounts = Vec::new();
        self.transactions = Vec::new();
        self.monitoring_service = None;
        self.monitoring_stats = MonitoringStats::default();
        self.surfpool_service = None;
        self.connection_status = ConnectionStatus::Disconnected;
        self.current_network = SolanaNetwork::Mainnet;
        self.loading = LoadingState::idle();
        self.error = None;
    }
}

/// Connection status enum
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

impl ConnectionStatus {
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }

    /// Check if connecting
    pub fn is_connecting(&self) -> bool {
        matches!(self, Self::Connecting | Self::Reconnecting)
    }

    /// Get display text
    pub fn display_text(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting...",
            Self::Connected => "Connected",
            Self::Reconnecting => "Reconnecting...",
            Self::Failed => "Connection Failed",
        }
    }

    /// Get CSS class for styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Disconnected => "status-disconnected",
            Self::Connecting => "status-connecting",
            Self::Connected => "status-connected",
            Self::Reconnecting => "status-reconnecting",
            Self::Failed => "status-failed",
        }
    }
}

/// Loading state for async operations
#[derive(Debug, Clone, Default)]
pub struct LoadingState {
    pub is_loading: bool,
    pub message: Option<String>,
    pub progress: Option<f32>,
}

impl LoadingState {
    /// Create idle state
    pub fn idle() -> Self {
        Self {
            is_loading: false,
            message: None,
            progress: None,
        }
    }

    /// Create loading state with message
    pub fn loading(message: impl Into<String>) -> Self {
        Self {
            is_loading: true,
            message: Some(message.into()),
            progress: None,
        }
    }

    /// Create loading state with progress
    pub fn loading_with_progress(message: impl Into<String>, progress: f32) -> Self {
        Self {
            is_loading: true,
            message: Some(message.into()),
            progress: Some(progress),
        }
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Get loading message
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Get progress
    pub fn progress(&self) -> Option<f32> {
        self.progress
    }
}

/// Environment statistics
#[derive(Debug, Clone)]
pub struct EnvironmentStats {
    pub total_balance: f64,
    pub active_accounts: usize,
    pub total_transactions: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// State backup for persistence
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateBackup {
    pub projects: Vec<Project>,
    pub active_project_id: Option<ProjectId>,
    pub environments: Vec<Environment>,
    pub active_environment_id: Option<EnvironmentId>,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub current_network: SolanaNetwork,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hook to use application state in components
pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}

/// Hook to get mutable access to app state
pub fn use_app_state_mut() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}
