//! # Application State Management Module
//!
//! This module provides comprehensive state management for the SurfDesk application.
//! It uses Dioxus signals for reactive state management across all platforms and
//! provides a centralized store for application data.

use crate::{error::Result, services::solana::SolanaService, types::*};
use dioxus::prelude::*;
use std::sync::Arc;

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
/// across components and platforms. It uses Dioxus signals for reactive updates.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current projects
    pub projects: Signal<Vec<Project>>,
    /// Active project ID
    pub active_project_id: Signal<Option<ProjectId>>,
    /// Environments for the active project
    pub environments: Signal<Vec<Environment>>,
    /// Active environment ID
    pub active_environment_id: Signal<Option<EnvironmentId>>,
    /// Accounts in the active environment
    pub accounts: Signal<Vec<Account>>,
    /// Transactions in the active environment
    pub transactions: Signal<Vec<Transaction>>,
    /// Solana service instance
    pub solana_service: Signal<Option<Arc<SolanaService>>>,
    /// Connection status
    pub connection_status: Signal<ConnectionStatus>,
    /// Current network
    pub current_network: Signal<SolanaNetwork>,
    /// Loading states
    pub loading: Signal<LoadingState>,
    /// Error state
    pub error: Signal<Option<crate::error::SurfDeskError>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: Signal::new(Vec::new()),
            active_project_id: Signal::new(None),
            environments: Signal::new(Vec::new()),
            active_environment_id: Signal::new(None),
            accounts: Signal::new(Vec::new()),
            transactions: Signal::new(Vec::new()),
            solana_service: Signal::new(None),
            connection_status: Signal::new(ConnectionStatus::Disconnected),
            current_network: Signal::new(crate::types::SolanaNetwork::Devnet),
            loading: Signal::new(LoadingState::default()),
            error: Signal::new(None),
        }
    }
}

/// Connection status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    /// Disconnected
    #[default]
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Error
    Error,
}

impl ConnectionStatus {
    /// Get display name for the connection status
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting...",
            Self::Connected => "Connected",
            Self::Error => "Error",
        }
    }

    /// Get CSS class for styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Disconnected => "status-disconnected",
            Self::Connecting => "status-connecting",
            Self::Connected => "status-connected",
            Self::Error => "status-error",
        }
    }
}

/// Loading state tracking
#[derive(Debug, Clone, Default)]
pub struct LoadingState {
    /// Whether projects are loading
    pub projects: bool,
    /// Whether environments are loading
    pub environments: bool,
    /// Whether accounts are loading
    pub accounts: bool,
    /// Whether transactions are loading
    pub transactions: bool,
    /// Whether a transaction is being sent
    pub sending_transaction: bool,
    /// Whether simulation is running
    pub simulating: bool,
}

impl LoadingState {
    /// Check if anything is currently loading
    pub fn is_loading(&self) -> bool {
        self.projects
            || self.environments
            || self.accounts
            || self.transactions
            || self.sending_transaction
            || self.simulating
    }

    /// Get the current loading message
    pub fn loading_message(&self) -> Option<&'static str> {
        if self.projects {
            Some("Loading projects...")
        } else if self.environments {
            Some("Loading environments...")
        } else if self.accounts {
            Some("Loading accounts...")
        } else if self.transactions {
            Some("Loading transactions...")
        } else if self.sending_transaction {
            Some("Sending transaction...")
        } else if self.simulating {
            Some("Simulating transaction...")
        } else {
            None
        }
    }
}

/// Loading operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingOperation {
    Projects,
    Environments,
    Accounts,
    Transactions,
    SendingTransaction,
    Simulating,
}

impl AppState {
    /// Create a new application state instance
    pub fn new() -> Self {
        Self {
            projects: Signal::new(Vec::new()),
            active_project_id: Signal::new(None),
            environments: Signal::new(Vec::new()),
            active_environment_id: Signal::new(None),
            accounts: Signal::new(Vec::new()),
            transactions: Signal::new(Vec::new()),
            solana_service: Signal::new(None),
            connection_status: Signal::new(ConnectionStatus::Disconnected),
            current_network: Signal::new(SolanaNetwork::Devnet),
            loading: Signal::new(LoadingState::default()),
            error: Signal::new(None),
        }
    }

    /// Initialize the application state
    pub async fn initialize(&mut self) -> Result<()> {
        // Initialize default state
        self.initialize_defaults().await?;
        Ok(())
    }

    /// Initialize default application state
    async fn initialize_defaults(&mut self) -> Result<()> {
        // Set initial connection status
        self.connection_status.set(ConnectionStatus::Disconnected);

        // Set default network
        self.current_network.set(SolanaNetwork::Devnet);

        log::info!("Application state initialized with defaults");
        Ok(())
    }

    /// Get the current connection status display name
    pub fn connection_status(&self) -> String {
        self.connection_status.read().display_name().to_string()
    }

    /// Get the current network display name
    pub fn current_network(&self) -> String {
        self.current_network.read().display_name().to_string()
    }

    /// Get the active project
    pub fn active_project(&self) -> Option<Project> {
        let active_id = self.active_project_id.read();
        let projects = self.projects.read();
        active_id.and_then(|id| projects.iter().find(|p| p.id == id).cloned())
    }

    /// Get the active environment
    pub fn active_environment(&self) -> Option<Environment> {
        let active_id = self.active_environment_id.read();
        let environments = self.environments.read();
        active_id.and_then(|id| environments.iter().find(|e| e.id == id).cloned())
    }

    /// Set the active project
    pub fn set_active_project(&mut self, project_id: Option<ProjectId>) {
        self.active_project_id.set(project_id);
        log::debug!("Active project changed to: {:?}", project_id);

        // Clear active environment when changing projects
        self.active_environment_id.set(None);
    }

    /// Set the active environment
    pub fn set_active_environment(&mut self, environment_id: Option<EnvironmentId>) {
        self.active_environment_id.set(environment_id);
        log::debug!("Active environment changed to: {:?}", environment_id);
    }

    /// Add a project
    pub fn add_project(&mut self, project: Project) {
        let mut projects = self.projects.read().clone();
        projects.push(project);
        self.projects.set(projects);
        log::info!("Added new project");
    }

    /// Update an existing project
    pub fn update_project(&mut self, project_id: ProjectId, updated_project: Project) {
        let mut projects = self.projects.read().clone();
        if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
            *project = updated_project;
            self.projects.set(projects);
            log::info!("Updated project: {:?}", project_id);
        }
    }

    /// Remove a project
    pub fn remove_project(&mut self, project_id: ProjectId) {
        let mut projects = self.projects.read().clone();
        projects.retain(|p| p.id != project_id);
        self.projects.set(projects);

        // Clear active project if it was removed
        if *self.active_project_id.read() == Some(project_id) {
            self.active_project_id.set(None);
        }

        log::info!("Removed project: {:?}", project_id);
    }

    /// Add an environment
    pub fn add_environment(&mut self, environment: Environment) {
        let mut environments = self.environments.read().clone();
        environments.push(environment);
        self.environments.set(environments);
        log::info!("Added new environment");
    }

    /// Update an existing environment
    pub fn update_environment(
        &mut self,
        environment_id: EnvironmentId,
        updated_environment: Environment,
    ) {
        let mut environments = self.environments.read().clone();
        if let Some(environment) = environments.iter_mut().find(|e| e.id == environment_id) {
            *environment = updated_environment;
            self.environments.set(environments);
            log::info!("Updated environment: {:?}", environment_id);
        }
    }

    /// Remove an environment
    pub fn remove_environment(&mut self, environment_id: EnvironmentId) {
        let mut environments = self.environments.read().clone();
        environments.retain(|e| e.id != environment_id);
        self.environments.set(environments);

        // Clear active environment if it was removed
        if *self.active_environment_id.read() == Some(environment_id) {
            self.active_environment_id.set(None);
        }

        log::info!("Removed environment: {:?}", environment_id);
    }

    /// Add an account
    pub fn add_account(&mut self, account: Account) {
        let mut accounts = self.accounts.read().clone();
        accounts.push(account);
        self.accounts.set(accounts);
        log::info!("Added new account");
    }

    /// Update an existing account
    pub fn update_account(&mut self, account_id: AccountId, updated_account: Account) {
        let mut accounts = self.accounts.read().clone();
        if let Some(account) = accounts.iter_mut().find(|a| a.id == account_id) {
            *account = updated_account;
            self.accounts.set(accounts);
            log::info!("Updated account: {:?}", account_id);
        }
    }

    /// Remove an account
    pub fn remove_account(&mut self, account_id: AccountId) {
        let mut accounts = self.accounts.read().clone();
        accounts.retain(|a| a.id != account_id);
        self.accounts.set(accounts);
        log::info!("Removed account: {:?}", account_id);
    }

    /// Add a transaction
    pub fn add_transaction(&mut self, transaction: Transaction) {
        let mut transactions = self.transactions.read().clone();
        transactions.push(transaction);
        self.transactions.set(transactions);
        log::info!("Added new transaction");
    }

    /// Update an existing transaction
    pub fn update_transaction(
        &mut self,
        transaction_id: TransactionId,
        updated_transaction: Transaction,
    ) {
        let mut transactions = self.transactions.read().clone();
        if let Some(transaction) = transactions.iter_mut().find(|t| t.id == transaction_id) {
            *transaction = updated_transaction;
            self.transactions.set(transactions);
            log::info!("Updated transaction: {:?}", transaction_id);
        }
    }

    /// Set loading state for a specific operation
    pub fn set_loading(&mut self, operation: LoadingOperation, loading: bool) {
        let mut loading_state = self.loading.read().clone();
        match operation {
            LoadingOperation::Projects => loading_state.projects = loading,
            LoadingOperation::Environments => loading_state.environments = loading,
            LoadingOperation::Accounts => loading_state.accounts = loading,
            LoadingOperation::Transactions => loading_state.transactions = loading,
            LoadingOperation::SendingTransaction => loading_state.sending_transaction = loading,
            LoadingOperation::Simulating => loading_state.simulating = loading,
        }
        self.loading.set(loading_state);

        if loading {
            log::debug!("Started loading: {:?}", operation);
        } else {
            log::debug!("Finished loading: {:?}", operation);
        }
    }

    /// Check if anything is loading
    pub fn is_loading(&self) -> bool {
        self.loading.read().is_loading()
    }

    /// Get the current loading message
    pub fn loading_message(&self) -> Option<String> {
        self.loading.read().loading_message().map(|s| s.to_string())
    }

    /// Set error state
    pub fn set_error(&mut self, error: Option<crate::error::SurfDeskError>) {
        self.error.set(error.clone());
        if let Some(ref err) = error {
            log::error!("Application error: {}", err);
        }
    }

    /// Clear error state
    pub fn clear_error(&mut self) {
        self.error.set(None);
    }

    /// Set connection status
    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status.set(status);
        log::info!("Connection status changed to: {}", status.display_name());
    }

    /// Set current network
    pub fn set_current_network(&mut self, network: SolanaNetwork) {
        self.current_network.set(network);
        log::info!("Current network changed to: {}", network.display_name());
    }

    /// Initialize Solana service
    pub async fn initialize_solana_service(&mut self, rpc_url: String) -> Result<()> {
        #[cfg(feature = "solana")]
        {
            use crate::services::solana::SolanaService;

            match SolanaService::new(rpc_url).await {
                Ok(service) => {
                    self.solana_service.set(Some(Arc::new(service)));
                    self.set_connection_status(ConnectionStatus::Connected);
                    log::info!("Solana service initialized successfully");
                    Ok(())
                }
                Err(e) => {
                    self.set_connection_status(ConnectionStatus::Error);
                    self.set_error(Some(e.clone()));
                    Err(e)
                }
            }
        }

        #[cfg(not(feature = "solana"))]
        {
            let error = crate::error::SurfDeskError::unsupported("Solana support not enabled");
            self.set_error(Some(error.clone()));
            Err(error)
        }
    }

    /// Get the Solana service
    pub fn solana_service(&self) -> Option<Arc<crate::services::solana::SolanaService>> {
        self.solana_service.read().clone()
    }

    /// Refresh the active environment
    pub async fn refresh_active_environment(&mut self) -> Result<()> {
        let active_environment = match self.active_environment() {
            Some(env) => env,
            None => return Ok(()),
        };

        // Set loading state
        self.set_loading(LoadingOperation::Accounts, true);

        // Fetch accounts for the active environment
        if let Some(ref service) = self.solana_service() {
            match service.get_accounts(&active_environment).await {
                Ok(accounts) => {
                    // Clear existing accounts and add new ones
                    // This is a simplified approach - in reality you'd want to merge/update
                    self.accounts.set(accounts);
                    log::info!("Refreshed {} accounts", self.accounts.read().len());
                }
                Err(e) => {
                    self.set_error(Some(e.clone()));
                    log::error!("Failed to refresh accounts: {}", e);
                }
            }
        }

        // Clear loading state
        self.set_loading(LoadingOperation::Accounts, false);
        Ok(())
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            theme: Theme::Auto,
            sidebar: SidebarState {
                expanded: true,
                active_section: SidebarSection::Dashboard,
                collapsed_sections: Vec::new(),
            },
            main_content: MainContentState {
                current_view: ContentView::Dashboard,
                view_state: serde_json::Value::Object(serde_json::Map::new()),
            },
            modal: None,
            notifications: Vec::new(),
        }
    }
}

impl UIState {
    /// Add a notification
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
        // Keep only the last 100 notifications
        if self.notifications.len() > 100 {
            self.notifications.remove(0);
        }
    }

    /// Remove a notification by ID
    pub fn remove_notification(&mut self, notification_id: String) {
        self.notifications.retain(|n| n.id != notification_id);
    }

    /// Clear all notifications
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    /// Set the current view
    pub fn set_current_view(&mut self, view: ContentView) {
        self.main_content.current_view = view;
    }

    /// Show a modal
    pub fn show_modal(&mut self, modal: ModalState) {
        self.modal = Some(modal);
    }

    /// Hide the modal
    pub fn hide_modal(&mut self) {
        self.modal = None;
    }

    /// Toggle sidebar visibility
    pub fn toggle_sidebar(&mut self) {
        self.sidebar.expanded = !self.sidebar.expanded;
    }

    /// Set the active sidebar section
    pub fn set_active_section(&mut self, section: SidebarSection) {
        self.sidebar.active_section = section;
    }

    /// Toggle a sidebar section
    pub fn toggle_section(&mut self, section: SidebarSection) {
        if self.sidebar.collapsed_sections.contains(&section) {
            self.sidebar.collapsed_sections.retain(|s| s != &section);
        } else {
            self.sidebar.collapsed_sections.push(section);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_creation() {
        let state = AppState::new();
        assert!(state.projects.read().is_empty());
        assert!(state.active_project_id.read().is_none());
        assert!(state.environments.read().is_empty());
        assert!(state.active_environment_id.read().is_none());
    }

    fn test_loading_state() {
        let mut state = LoadingState::default();
        assert!(!state.is_loading());

        state.projects = true;
        assert!(state.is_loading());
        assert_eq!(state.loading_message(), Some("Loading projects..."));

        state.projects = false;
        assert!(!state.is_loading());
        assert!(state.loading_message().is_none());
    }

    fn test_ui_state_default() {
        let state = UIState::default();
        assert_eq!(state.theme, Theme::Auto);
        assert!(state.sidebar.expanded);
        assert_eq!(state.sidebar.active_section, SidebarSection::Dashboard);
        assert!(state.notifications.is_empty());
        assert!(state.modal.is_none());
    }

    fn test_ui_state_notifications() {
        let mut state = UIState::default();
        let notification = Notification {
            id: uuid::Uuid::new_v4().to_string(),
            notification_type: NotificationType::Info,
            title: "Test".to_string(),
            message: "Test message".to_string(),
            persistent: false,
            created_at: chrono::Utc::now(),
        };

        state.add_notification(notification.clone());
        assert_eq!(state.notifications.len(), 1);

        state.remove_notification(notification.id);
        assert!(state.notifications.is_empty());

        state.clear_notifications();
        assert!(state.notifications.is_empty());
    }

    fn test_ui_state_sidebar() {
        let mut state = UIState::default();
        assert!(state.sidebar.expanded);

        state.toggle_sidebar();
        assert!(!state.sidebar.expanded);

        state.set_active_section(SidebarSection::Accounts);
        assert_eq!(state.sidebar.active_section, SidebarSection::Accounts);

        state.toggle_section(SidebarSection::Accounts);
        assert!(state
            .sidebar
            .collapsed_sections
            .contains(&SidebarSection::Accounts));
    }

    fn test_connection_status() {
        let status = ConnectionStatus::Connected;
        assert_eq!(status.display_name(), "Connected");
        assert_eq!(status.css_class(), "status-connected");

        let status = ConnectionStatus::Error;
        assert_eq!(status.display_name(), "Error");
        assert_eq!(status.css_class(), "status-error");
    }

    fn test_solana_network() {
        let network = SolanaNetwork::Mainnet;
        assert_eq!(network.display_name(), "Mainnet");

        let network = SolanaNetwork::Devnet;
        assert_eq!(network.display_name(), "Devnet");
    }
}
