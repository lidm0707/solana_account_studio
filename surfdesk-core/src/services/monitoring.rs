//! # Account Monitoring Service
//!
//! This service provides real-time monitoring of Solana accounts with configurable
//! alerts and notifications. It runs background tasks to check account states
//! and triggers alerts based on user-defined conditions.

use crate::error::Result;
use crate::types::{
    Account, AccountMonitoring, AlertConfig, AlertType, MonitoringConfig, MonitoringEvent,
    MonitoringEventType, MonitoringStats, SolanaPubkey,
};
use log::{debug, error, info, warn};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::Instant;
use uuid::Uuid;

// Missing type definitions
#[derive(Debug, Clone, Default)]
pub struct AlertConditions {
    pub min_balance: Option<f64>,
    pub max_balance: Option<f64>,
    pub balance_change_threshold: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum NotificationChannel {
    InApp,
    Email(String),
    Webhook(String),
}

impl Default for NotificationChannel {
    fn default() -> Self {
        Self::InApp
    }
}

/// Account monitoring service
pub struct MonitoringService {
    /// Service configuration
    config: Arc<RwLock<MonitoringConfig>>,
    /// Currently monitored accounts
    monitored_accounts: Arc<RwLock<HashMap<SolanaPubkey, AccountMonitoring>>>,
    /// Monitoring statistics
    stats: Arc<RwLock<MonitoringStats>>,
    /// Background task handles
    task_handles: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
    /// Service state
    is_running: Arc<RwLock<bool>>,
    /// Event handlers for alerts
    alert_handlers: Arc<RwLock<Vec<Box<dyn AlertHandler>>>>,
}

impl std::fmt::Debug for MonitoringService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonitoringService")
            .field("config", &"[config]")
            .field("monitored_accounts", &"[accounts]")
            .field("stats", &"[stats]")
            .field("task_handles", &"[tasks]")
            .field("is_running", &"[state]")
            .field("alert_handlers", &"[handlers]")
            .finish()
    }
}

/// Alert handler trait for custom alert processing
#[async_trait::async_trait]
pub trait AlertHandler: Send + Sync {
    /// Handle an alert event
    async fn handle_alert(&self, event: &MonitoringEvent) -> Result<()>;
}

impl MonitoringService {
    /// Create a new monitoring service
    pub async fn new() -> Result<Self> {
        let config = MonitoringConfig::default();
        Self::with_config(config).await
    }

    /// Create a monitoring service with custom configuration
    pub async fn with_config(config: MonitoringConfig) -> Result<Self> {
        info!("Initializing monitoring service");

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            monitored_accounts: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MonitoringStats::default())),
            task_handles: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
            alert_handlers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Start the monitoring service
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Monitoring service is already running");
            return Ok(());
        }

        info!("Starting monitoring service");
        *is_running = true;

        // Start background monitoring task
        let task_handle = self.start_monitoring_loop().await?;
        self.task_handles.lock().await.push(task_handle);

        info!("Monitoring service started successfully");
        Ok(())
    }

    /// Stop the monitoring service
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping monitoring service");

        let mut is_running = self.is_running.write().await;
        *is_running = false;

        // Cancel all background tasks
        let mut handles = self.task_handles.lock().await;
        for handle in handles.drain(..) {
            handle.abort();
        }

        info!("Monitoring service stopped");
        Ok(())
    }

    /// Add an account to monitoring
    pub async fn add_account_monitoring(&self, account: &Account) -> Result<()> {
        let mut monitored = self.monitored_accounts.write().await;

        let monitoring = AccountMonitoring {
            enabled: true,
            ..AccountMonitoring::default()
        };

        let is_enabled = monitoring.enabled;
        monitored.insert(account.pubkey.clone(), monitoring);

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_accounts += 1;
            if is_enabled {
                stats.active_accounts += 1;
            }
        }

        info!("Added account {} to monitoring", account.pubkey);
        Ok(())
    }

    /// Remove an account from monitoring
    pub async fn remove_account_monitoring(&self, pubkey: &SolanaPubkey) -> Result<()> {
        let mut monitored = self.monitored_accounts.write().await;

        if let Some(monitoring) = monitored.remove(pubkey) {
            let was_enabled = monitoring.enabled;

            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.total_accounts = stats.total_accounts.saturating_sub(1);
                if was_enabled {
                    stats.active_accounts = stats.active_accounts.saturating_sub(1);
                }
            }

            info!("Removed account {} from monitoring", pubkey);
        }

        Ok(())
    }

    /// Update monitoring configuration for an account
    pub async fn update_account_monitoring(
        &self,
        pubkey: &SolanaPubkey,
        monitoring: AccountMonitoring,
    ) -> Result<()> {
        let mut monitored = self.monitored_accounts.write().await;

        let was_enabled = monitored.get(pubkey).map(|m| m.enabled).unwrap_or(false);
        let enabled = monitoring.enabled;
        monitored.insert(pubkey.clone(), monitoring.clone());

        // Update stats if enabled state changed
        if was_enabled != enabled {
            let mut stats = self.stats.write().await;
            if enabled {
                stats.active_accounts += 1;
            } else {
                stats.active_accounts = stats.active_accounts.saturating_sub(1);
            }
        }

        info!("Updated monitoring configuration for account {}", pubkey);
        Ok(())
    }

    /// Add an alert configuration to an account
    pub async fn add_alert(&self, pubkey: &SolanaPubkey, alert: AlertConfig) -> Result<()> {
        let mut monitored = self.monitored_accounts.write().await;

        if let Some(monitoring) = monitored.get_mut(pubkey) {
            monitoring.alerts.push(alert);
            info!("Added alert for account {}", pubkey);
        } else {
            warn!("Account {} not found in monitoring", pubkey);
        }

        Ok(())
    }

    /// Remove an alert configuration from an account
    pub async fn remove_alert(&self, pubkey: &SolanaPubkey, alert_id: Uuid) -> Result<()> {
        let mut monitored = self.monitored_accounts.write().await;

        if let Some(monitoring) = monitored.get_mut(pubkey) {
            monitoring.alerts.retain(|alert| alert.id != alert_id);
            info!("Removed alert {} for account {}", alert_id, pubkey);
        }

        Ok(())
    }

    /// Get monitoring configuration for an account
    pub async fn get_account_monitoring(&self, pubkey: &SolanaPubkey) -> Option<AccountMonitoring> {
        let monitored = self.monitored_accounts.read().await;
        monitored.get(pubkey).cloned()
    }

    /// Get all monitored accounts
    pub async fn get_monitored_accounts(&self) -> HashMap<SolanaPubkey, AccountMonitoring> {
        let monitored = self.monitored_accounts.read().await;
        monitored.clone()
    }

    /// Get monitoring statistics
    pub async fn get_stats(&self) -> MonitoringStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Add a custom alert handler
    pub async fn add_alert_handler(&self, handler: Box<dyn AlertHandler>) {
        let mut handlers = self.alert_handlers.write().await;
        handlers.push(handler);
    }

    /// Get monitoring events for an account
    pub async fn get_account_events(&self, pubkey: &SolanaPubkey) -> Vec<MonitoringEvent> {
        let monitored = self.monitored_accounts.read().await;
        if let Some(monitoring) = monitored.get(pubkey) {
            monitoring.history.clone()
        } else {
            Vec::new()
        }
    }

    /// Trigger a manual check for specific accounts
    pub async fn trigger_manual_check(&self, pubkeys: &[SolanaPubkey]) -> Result<()> {
        info!("Triggering manual check for {} accounts", pubkeys.len());

        for pubkey in pubkeys {
            if let Err(e) = self.check_account(pubkey).await {
                error!("Failed to check account {}: {}", pubkey, e);
            }
        }

        Ok(())
    }

    /// Start the background monitoring loop
    async fn start_monitoring_loop(&self) -> Result<tokio::task::JoinHandle<()>> {
        let config = Arc::clone(&self.config);
        let monitored_accounts = Arc::clone(&self.monitored_accounts);
        let stats = Arc::clone(&self.stats);
        let is_running = Arc::clone(&self.is_running);
        let alert_handlers = Arc::clone(&self.alert_handlers);

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60)); // Check every minute

            loop {
                interval.tick().await;

                // Check if service is still running
                {
                    let running = is_running.read().await;
                    if !*running {
                        break;
                    }
                }

                // Get current configuration
                let interval_seconds = {
                    let config = config.read().await;
                    config.default_interval_seconds
                };

                // Create new interval if period changed
                if interval_seconds != 60 {
                    drop(interval);
                    interval = tokio::time::interval(Duration::from_secs(interval_seconds));
                }

                // Get accounts to check
                let accounts_to_check: Vec<SolanaPubkey> = {
                    let monitored = monitored_accounts.read().await;
                    monitored
                        .iter()
                        .filter(|(_, monitoring)| monitoring.enabled)
                        .map(|(pubkey, _)| pubkey.clone())
                        .collect()
                };

                if accounts_to_check.is_empty() {
                    continue;
                }

                debug!("Checking {} monitored accounts", accounts_to_check.len());

                // Check each account
                for pubkey in accounts_to_check {
                    let result = Self::check_account_internal(
                        &pubkey,
                        &monitored_accounts,
                        &stats,
                        &alert_handlers,
                    )
                    .await;

                    if let Err(e) = result {
                        error!("Failed to check account {}: {}", pubkey, e);
                    }
                }

                // Update last check time
                {
                    let mut stats = stats.write().await;
                    stats.last_check = Some(chrono::Utc::now());
                }
            }
        });

        Ok(handle)
    }

    /// Check a single account for changes
    async fn check_account(&self, pubkey: &SolanaPubkey) -> Result<()> {
        let monitored_accounts = Arc::clone(&self.monitored_accounts);
        let stats = Arc::clone(&self.stats);
        let alert_handlers = Arc::clone(&self.alert_handlers);

        Self::check_account_internal(pubkey, &monitored_accounts, &stats, &alert_handlers).await
    }

    /// Internal account checking logic
    async fn check_account_internal(
        pubkey: &SolanaPubkey,
        monitored_accounts: &Arc<RwLock<HashMap<SolanaPubkey, AccountMonitoring>>>,
        stats: &Arc<RwLock<MonitoringStats>>,
        alert_handlers: &Arc<RwLock<Vec<Box<dyn AlertHandler>>>>,
    ) -> Result<()> {
        let start_time = Instant::now();

        // Get current account state (this would normally involve RPC calls)
        // For now, we'll simulate the check
        let current_balance = Self::get_account_balance(pubkey).await?;
        let _current_data = Self::get_account_data(pubkey).await?;

        // Update monitoring data
        {
            let mut monitored = monitored_accounts.write().await;
            if let Some(monitoring) = monitored.get_mut(pubkey) {
                let previous_balance = monitoring.history.last().and_then(|event| {
                    if let MonitoringEventType::BalanceChanged { new_balance, .. } =
                        event.event_type
                    {
                        Some(new_balance)
                    } else {
                        None
                    }
                });

                // Check for balance changes
                if let Some(prev_balance) = previous_balance {
                    if prev_balance != current_balance {
                        let change_amount = current_balance as i64 - prev_balance as i64;

                        let event = MonitoringEvent {
                            id: Uuid::new_v4(),
                            timestamp: chrono::Utc::now(),
                            event_type: MonitoringEventType::BalanceChanged {
                                old_balance: prev_balance,
                                new_balance: current_balance,
                                change_amount,
                            },
                            account_pubkey: pubkey.clone(),
                            data: serde_json::json!({
                                "balance": current_balance,
                                "change_amount": change_amount
                            }),
                            previous_state: Some(serde_json::json!({ "balance": prev_balance })),
                            new_state: serde_json::json!({ "balance": current_balance }),
                        };

                        monitoring.history.push(event.clone());

                        // Limit history size
                        let max_events = 1000; // This should come from config
                        if monitoring.history.len() > max_events {
                            monitoring.history.remove(0);
                        }

                        // Check alerts
                        Self::check_alerts(pubkey, &monitoring.alerts, &event, alert_handlers)
                            .await?;
                    }
                }

                monitoring.last_checked = Some(chrono::Utc::now());
                monitoring.check_count += 1;
            }
        }

        // Update statistics
        {
            let mut stats = stats.write().await;
            stats.total_checks += 1;
            let duration = start_time.elapsed().as_millis() as f64;
            stats.avg_check_duration_ms =
                (stats.avg_check_duration_ms * (stats.total_checks - 1) as f64 + duration)
                    / stats.total_checks as f64;
        }

        Ok(())
    }

    /// Check if any alerts should be triggered
    async fn check_alerts(
        pubkey: &SolanaPubkey,
        alerts: &[AlertConfig],
        event: &MonitoringEvent,
        alert_handlers: &Arc<RwLock<Vec<Box<dyn AlertHandler>>>>,
    ) -> Result<()> {
        let now = chrono::Utc::now();

        for alert in alerts {
            if !alert.active {
                continue;
            }

            // Check cooldown
            if let Some(last_triggered) = alert.last_triggered {
                let cooldown = chrono::Duration::seconds(alert.cooldown_seconds as i64);
                if now.signed_duration_since(last_triggered) < cooldown {
                    continue;
                }
            }

            // Check if alert should trigger
            if Self::should_trigger_alert(alert, event).await? {
                info!("Alert '{}' triggered for account {}", alert.name, pubkey);

                // Create alert event
                let alert_message = Self::generate_alert_message(alert, event);
                let alert_event = MonitoringEvent {
                    id: Uuid::new_v4(),
                    timestamp: now,
                    event_type: MonitoringEventType::AlertTriggered {
                        alert_id: alert.id,
                        alert_name: alert.name.clone(),
                        message: alert_message,
                    },
                    account_pubkey: pubkey.clone(),
                    data: serde_json::json!({
                        "alert_id": alert.id,
                        "alert_name": alert.name,
                        "original_event": event
                    }),
                    previous_state: None,
                    new_state: serde_json::json!({ "alert_triggered": true }),
                };

                // Call all alert handlers
                {
                    let handlers = alert_handlers.read().await;
                    for handler in handlers.iter() {
                        if let Err(e) = handler.handle_alert(&alert_event).await {
                            error!("Alert handler failed: {}", e);
                        }
                    }
                }

                // Update alert stats (this would need mutable access to the alert)
                // For now, we just log it
            }
        }

        Ok(())
    }

    /// Determine if an alert should be triggered
    async fn should_trigger_alert(alert: &AlertConfig, event: &MonitoringEvent) -> Result<bool> {
        match &alert.alert_type {
            AlertType::BalanceChange => {
                if let MonitoringEventType::BalanceChanged { change_amount, .. } = &event.event_type
                {
                    if let Some(min_change) = alert.conditions.min_balance_change {
                        return Ok(change_amount.abs() >= min_change as i64);
                    }
                }
            }
            AlertType::LargeTransaction => {
                if let MonitoringEventType::TransactionDetected { amount, .. } = &event.event_type {
                    if let Some(max_amount) = alert.conditions.max_transaction_amount {
                        return Ok(*amount >= max_amount);
                    }
                }
            }
            AlertType::AccountActivity => {
                // Trigger on any activity
                return Ok(true);
            }
            AlertType::CustomThreshold => {
                // Custom condition evaluation would go here
                // For now, we'll return false
                return Ok(false);
            }
            _ => {}
        }

        Ok(false)
    }

    /// Generate alert message
    fn generate_alert_message(alert: &AlertConfig, event: &MonitoringEvent) -> String {
        match &alert.alert_type {
            AlertType::BalanceChange => {
                if let MonitoringEventType::BalanceChanged { change_amount, .. } = &event.event_type
                {
                    format!("Balance changed by {} lamports", change_amount)
                } else {
                    "Balance change detected".to_string()
                }
            }
            AlertType::LargeTransaction => "Large transaction detected".to_string(),
            AlertType::AccountActivity => "Account activity detected".to_string(),
            _ => format!("Alert '{}' triggered", alert.name),
        }
    }

    /// Get account balance (simulated - would use RPC in real implementation)
    async fn get_account_balance(_pubkey: &SolanaPubkey) -> Result<u64> {
        // Simulate balance with some randomness
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(1_000_000..100_000_000))
    }

    /// Get account data (simulated - would use RPC in real implementation)
    async fn get_account_data(_pubkey: &SolanaPubkey) -> Result<Value> {
        // Simulate account data
        Ok(serde_json::json!({
            "data": "simulated_account_data",
            "length": 165
        }))
    }

    /// Shutdown the monitoring service
    pub async fn shutdown(&self) -> Result<()> {
        self.stop().await?;
        info!("Monitoring service shutdown complete");
        Ok(())
    }
}

/// Default in-app alert handler
pub struct InAppAlertHandler;

#[async_trait::async_trait]
impl AlertHandler for InAppAlertHandler {
    async fn handle_alert(&self, event: &MonitoringEvent) -> Result<()> {
        info!("In-app alert: {:?}", event);
        // In a real implementation, this would add the notification to the UI
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SolanaNetwork;

    #[tokio::test]
    async fn test_monitoring_service_creation() {
        let service = MonitoringService::new().await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_add_account_monitoring() {
        let service = MonitoringService::new().await.unwrap();

        let account = Account {
            id: Uuid::new_v4(),
            environment_id: Uuid::new_v4(),
            pubkey: SolanaPubkey::from_str("11111111111111111111111111111112").unwrap(),
            owner: SolanaPubkey::from_str("11111111111111111111111111111111").unwrap(),
            balance: 1000000,
            data: crate::types::AccountData::default(),
            executable: false,
            rent_epoch: 0,
            updated_at: chrono::Utc::now(),
            monitoring: AccountMonitoring::default(),
        };

        let result = service.add_account_monitoring(&account).await;
        assert!(result.is_ok());

        let monitoring = service.get_account_monitoring(&account.pubkey).await;
        assert!(monitoring.is_some());
        assert!(monitoring.unwrap().enabled);
    }

    #[tokio::test]
    async fn test_alert_configuration() {
        let service = MonitoringService::new().await.unwrap();
        let pubkey = SolanaPubkey::from_str("11111111111111111111111111111112").unwrap();

        let alert = AlertConfig {
            id: Uuid::new_v4(),
            name: "Test Alert".to_string(),
            alert_type: AlertType::BalanceChange,
            active: true,
            conditions: AlertConditions::default(),
            notification_channels: vec![NotificationChannel::InApp],
            cooldown_seconds: 300,
            last_triggered: None,
            trigger_count: 0,
        };

        let result = service.add_alert(&pubkey, alert.clone()).await;
        assert!(result.is_ok());

        // Note: We can't easily test the alert was added without the account being monitored first
        // This would require more complex test setup
    }

    #[tokio::test]
    async fn test_monitoring_stats() {
        let service = MonitoringService::new().await.unwrap();
        let stats = service.get_stats().await;

        assert_eq!(stats.total_accounts, 0);
        assert_eq!(stats.active_accounts, 0);
        assert_eq!(stats.total_checks, 0);
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = MonitoringService::new().await.unwrap();

        // Start service
        let start_result = service.start().await;
        assert!(start_result.is_ok());

        // Stop service
        let stop_result = service.stop().await;
        assert!(stop_result.is_ok());

        // Shutdown service
        let shutdown_result = service.shutdown().await;
        assert!(shutdown_result.is_ok());
    }
}
