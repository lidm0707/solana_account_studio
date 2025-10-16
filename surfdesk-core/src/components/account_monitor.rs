//! # Account Monitor Component
//!
//! Real-time account monitoring component with WebSocket integration.
//! Displays live account balances, transaction status, and real-time updates
//! for Solana accounts being tracked.

use crate::services::websocket::{AccountNotification, WebSocketManager, WebSocketMessage};
use crate::solana_rpc::Pubkey;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Account data structure for monitoring
#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredAccount {
    /// Account public key
    pub pubkey: String,
    /// Current balance in lamports
    pub balance: u64,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Subscription ID for WebSocket updates
    pub subscription_id: Option<u64>,
    /// Connection status
    pub status: AccountStatus,
    /// Transaction history (last 5)
    pub recent_transactions: Vec<String>,
}

/// Account monitoring status
#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    /// Not being monitored
    Idle,
    /// Connecting to WebSocket
    Connecting,
    /// Actively monitoring
    Monitoring,
    /// Connection failed
    Failed(String),
    /// Disconnected
    Disconnected,
}

/// Props for the AccountMonitor component
#[derive(Debug, Clone, Props)]
pub struct AccountMonitorProps {
    /// WebSocket manager for real-time updates
    websocket_manager: Option<WebSocketManager>,
    /// Initial accounts to monitor
    initial_accounts: Vec<String>,
    /// Callback when account is added
    on_account_added: Option<String>,
    /// Callback when account is removed
    on_account_removed: Option<String>,
    /// Show detailed view
    show_details: Option<bool>,
}

/// Account Monitor Component
#[component]
pub fn AccountMonitor(props: AccountMonitorProps) -> Element {
    let mut monitored_accounts = use_signal(|| {
        let mut accounts = HashMap::new();
        for pubkey in &props.initial_accounts {
            accounts.insert(
                pubkey.clone(),
                MonitoredAccount {
                    pubkey: pubkey.clone(),
                    balance: 0,
                    last_updated: current_timestamp(),
                    subscription_id: None,
                    status: AccountStatus::Idle,
                    recent_transactions: Vec::new(),
                },
            );
        }
        accounts
    });

    let mut connection_status = use_signal(|| "Disconnected".to_string());
    let mut websocket_messages = use_signal(Vec::<WebSocketMessage>::new);
    let mut error_message = use_signal(|| None::<String>);

    // WebSocket message handling effect
    use_effect(move || {
        let ws_manager = props.websocket_manager.clone();
        let mut accounts = monitored_accounts.clone();
        let mut messages = websocket_messages.clone();
        let mut status = connection_status.clone();
        let mut error = error_message.clone();

        spawn(async move {
            if let Some(manager) = ws_manager {
                // Get message receiver
                if let Some(mut receiver) = manager.get_message_receiver().await {
                    while let Some(message) = receiver.recv().await {
                        messages.write().push(message.clone());

                        match message {
                            WebSocketMessage::ConnectionStatus {
                                connected,
                                url,
                                error: err,
                            } => {
                                status.set(if connected {
                                    format!("Connected to {}", url)
                                } else {
                                    "Disconnected".to_string()
                                });

                                if let Some(err_msg) = err {
                                    error.set(Some(err_msg));
                                }
                            }
                            WebSocketMessage::AccountNotification { result, .. } => {
                                // Update account data
                                let mut accounts_map = accounts.read().clone();
                                if let Some(account) = accounts_map.get_mut(&result.pubkey) {
                                    account.balance = result.lamports;
                                    account.last_updated = current_timestamp();
                                    account.status = AccountStatus::Monitoring;
                                }
                                accounts.set(accounts_map);
                            }
                            WebSocketMessage::Error {
                                error,
                                subscription,
                            } => {
                                error.set(Some(error));
                                if let Some(sub_id) = subscription {
                                    // Update account status for this subscription
                                    let mut accounts_map = accounts.read().clone();
                                    for account in accounts_map.values_mut() {
                                        if account.subscription_id == Some(sub_id) {
                                            account.status = AccountStatus::Failed(
                                                "WebSocket error".to_string(),
                                            );
                                        }
                                    }
                                    accounts.set(accounts_map);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    });

    // Auto-connect effect
    use_effect(move || {
        if let Some(ref manager) = props.websocket_manager {
            spawn(async move {
                let _ = manager.connect().await;
            });
        }
    });

    // Format balance for display
    let format_balance = |balance: u64| {
        let sol = balance as f64 / 1_000_000_000.0;
        format!("{:.9} SOL", sol)
    };

    // Format timestamp
    let format_timestamp = |timestamp: u64| {
        let datetime = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
        format!("{:?}", datetime)
    };

    rsx! {
        div { class: "account-monitor",
            // Header
            div { class: "monitor-header",
                h3 { class: "monitor-title", "Account Monitor" }
                div { class: "connection-status",
                    span {
                        class: "status-indicator {if connection_status.read().contains("Connected") { "connected" } else { "disconnected" }}",
                        "{connection_status}"
                    }
                }
            }

            // Error display
            if let Some(error) = error_message.read().as_ref() {
                div { class: "error-message",
                    span { "Error: {error}" }
                }
            }

            // Account list
            div { class: "account-list",
                for (_, account) in monitored_accounts.read().iter() {
                    AccountCard {
                        account: account.clone(),
                        format_balance,
                        format_timestamp,
                    }
                }
            }

            // Add account form
            AddAccountForm {
                websocket_manager: props.websocket_manager.clone(),
                on_account_added: props.on_account_added.clone(),
            }
        }
    }
}

/// Individual account card component
#[component]
fn AccountCard(
    account: MonitoredAccount,
    format_balance: impl Fn(u64) -> String + 'static,
    format_timestamp: impl Fn(u64) -> String + 'static,
) -> Element {
    let status_class = match account.status {
        AccountStatus::Monitoring => "monitoring",
        AccountStatus::Connecting => "connecting",
        AccountStatus::Failed(_) => "failed",
        AccountStatus::Disconnected => "disconnected",
        AccountStatus::Idle => "idle",
    };

    rsx! {
        div { class: "account-card {status_class}",
            div { class: "account-header",
                div { class: "account-info",
                    span { class: "account-pubkey",
                        title: "{account.pubkey}",
                        "{account.pubkey[..8]}...{account.pubkey[account.pubkey.len()-8..]}"
                    }
                    span { class: "account-balance", "{format_balance(account.balance)}" }
                }
                div { class: "account-status",
                    span { class: "status-badge {status_class}",
                        "{match account.status {
                            AccountStatus::Monitoring => "Monitoring",
                            AccountStatus::Connecting => "Connecting",
                            AccountStatus::Failed(_) => "Failed",
                            AccountStatus::Disconnected => "Disconnected",
                            AccountStatus::Idle => "Idle",
                        }}"
                    }
                }
            }

            div { class: "account-details",
                div { class: "last-updated",
                    "Last updated: {format_timestamp(account.last_updated)}"
                }

                if !account.recent_transactions.is_empty() {
                    div { class: "recent-transactions",
                        h5 { "Recent Transactions" }
                        ul { class: "transaction-list",
                            for tx in &account.recent_transactions {
                                li {
                                    span { class: "tx-signature",
                                        title: "{tx}",
                                        "{tx[..8]}...{tx[tx.len()-8..]}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Add account form component
#[component]
fn AddAccountForm(
    websocket_manager: Option<WebSocketManager>,
    on_account_added: Option<String>,
) -> Element {
    let mut pubkey_input = use_signal(String::new);
    let mut is_adding = use_signal(|| false);
    let mut add_error = use_signal(|| None::<String>);

    let handle_add_account = move |_| {
        if websocket_manager.is_none() {
            add_error.set(Some("WebSocket not available".to_string()));
            return;
        }

        let pubkey = pubkey_input.read().clone();
        if pubkey.trim().is_empty() {
            add_error.set(Some("Please enter a public key".to_string()));
            return;
        }

        is_adding.set(true);
        add_error.set(None);

        let manager = websocket_manager.clone();
        let mut adding = is_adding.clone();
        let mut error = add_error.clone();

        spawn(async move {
            if let Some(ws_manager) = manager {
                match ws_manager.subscribe_account(&pubkey).await {
                    Ok(_) => {
                        log::info!("Successfully subscribed to account: {}", pubkey);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to subscribe: {}", e)));
                    }
                }
            }
            adding.set(false);
        });

        pubkey_input.set(String::new());
    };

    rsx! {
        div { class: "add-account-form",
            h4 { "Add Account to Monitor" }
            div { class: "form-group",
                input {
                    "type": "text",
                    placeholder: "Enter account public key",
                    value: "{pubkey_input}",
                    oninput: move |e| pubkey_input.set(e.value()),
                    class: "pubkey-input"
                }
                button {
                    onclick: handle_add_account,
                    disabled: is_adding.read().clone() || websocket_manager.is_none(),
                    class: "add-button {if is_adding.read().clone() { "loading" } else { "" }}",
                    "{if is_adding.read().clone() { "Adding..." } else { "Add Account " }}"
                }
            }

            if let Some(error) = add_error.read().as_ref() {
                div { class: "form-error", "{error}" }
            }
        }
    }
}

/// Helper function to get current timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_balance() {
        let formatter = |balance: u64| format!("{:.9} SOL", balance as f64 / 1_000_000_000.0);
        assert_eq!(formatter(1_000_000_000), "1.000000000 SOL");
        assert_eq!(formatter(500_000_000), "0.500000000 SOL");
    }

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_monitored_account_creation() {
        let account = MonitoredAccount {
            pubkey: "11111111111111111111111111111111".to_string(),
            balance: 1_000_000_000,
            last_updated: current_timestamp(),
            subscription_id: Some(123),
            status: AccountStatus::Monitoring,
            recent_transactions: Vec::new(),
        };

        assert_eq!(account.pubkey, "11111111111111111111111111111111");
        assert_eq!(account.balance, 1_000_000_000);
        assert_eq!(account.status, AccountStatus::Monitoring);
    }
}
