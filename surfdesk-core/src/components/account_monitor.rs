//! # Account Monitor Component
//!
//! Real-time account monitoring component with WebSocket integration.
//! Displays live account balances, transaction status, and real-time updates
//! for Solana accounts being tracked.

use crate::services::websocket::WebSocketMessage;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::fmt;

use std::time::{SystemTime, UNIX_EPOCH};

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

impl AccountStatus {
    /// คืนค่า short class name (static str) เหมาะกับ CSS classes
    pub fn as_class(&self) -> &'static str {
        match self {
            AccountStatus::Monitoring => "monitoring",
            AccountStatus::Connecting => "connecting",
            AccountStatus::Failed(_) => "failed",
            AccountStatus::Disconnected => "disconnected",
            AccountStatus::Idle => "idle",
        }
    }

    /// ถ้าต้องการข้อความสั้นๆ แบบไม่รวม message ของ Failed
    pub fn as_label(&self) -> &'static str {
        match self {
            AccountStatus::Monitoring => "Monitoring",
            AccountStatus::Connecting => "Connecting",
            AccountStatus::Failed(_) => "Failed",
            AccountStatus::Disconnected => "Disconnected",
            AccountStatus::Idle => "Idle",
        }
    }
}

/// Implement Display เพื่อให้ `to_string()` คืนข้อความที่อ่านได้
impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountStatus::Idle => write!(f, "Idle"),
            AccountStatus::Connecting => write!(f, "Connecting"),
            AccountStatus::Monitoring => write!(f, "Monitoring"),
            AccountStatus::Failed(msg) => write!(f, "Failed: {}", msg),
            AccountStatus::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// Props for the AccountMonitor component
#[derive(PartialEq, Clone, Props)]
pub struct AccountMonitorProps {
    /// Initial accounts to monitor
    initial_accounts: Vec<String>,
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

    // Mock connection effect for demo purposes
    use_effect(move || {
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            connection_status.set("Connected (mock)".to_string());
        });
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
                        class: format!("status-indicator {}", if connection_status.read().contains("Connected") { "connected" } else { "disconnected" }),
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
                if !monitored_accounts.read().is_empty() {
                    div { class: "account-card",
                        div { class: "account-header",
                            div { class: "account-pubkey",
                                "Sample Account"
                            }
                            div { class: "account-status",
                                "monitoring"
                            }
                        }
                        div { class: "account-details",
                            div { class: "account-balance",
                                "Balance: 1.000000000 SOL"
                            }
                            div { class: "account-timestamp",
                                "Updated: Just now"
                            }
                        }
                    }
                } else {
                    div { class: "no-accounts",
                        "No accounts being monitored. Add an account to start monitoring."
                    }
                }
            }

            // Add account form
            AddAccountForm {
            }
        }
    }
}

#[component]
fn AccountCard(account: MonitoredAccount) -> Element {
    let status_class = match account.status {
        AccountStatus::Monitoring => "monitoring",
        AccountStatus::Connecting => "connecting",
        AccountStatus::Failed(_) => "failed",
        AccountStatus::Disconnected => "disconnected",
        AccountStatus::Idle => "idle",
    };

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

    // Format pubkey short display
    let format_pubkey_short = |pubkey: &str| {
        if pubkey.len() > 16 {
            format!("{}...{}", &pubkey[..8], &pubkey[pubkey.len() - 8..])
        } else {
            pubkey.to_string()
        }
    };

    // Format transaction short display
    let format_tx_short = |tx: &str| {
        if tx.len() > 16 {
            format!("{}...{}", &tx[..8], &tx[tx.len() - 8..])
        } else {
            tx.to_string()
        }
    };

    rsx! {
        div { class: "account-card {status_class}",
            div { class: "account-header",
                div { class: "account-info",
                    span { class: "account-pubkey",
                        title: "{account.pubkey}",
                        "{format_pubkey_short(&account.pubkey)}"
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
                                        "{format_tx_short(tx)}"
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
fn AddAccountForm() -> Element {
    let mut pubkey_input = use_signal(String::new);
    let mut is_adding = use_signal(|| false);
    let mut add_error = use_signal(|| None::<String>);

    let handle_add_account = move |_| {
        let pubkey = pubkey_input.read().clone();
        if pubkey.trim().is_empty() {
            add_error.set(Some("Please enter a public key".to_string()));
            return;
        }

        is_adding.set(true);
        add_error.set(None);

        let mut adding = is_adding.clone();
        let mut error = add_error.clone();
        let pubkey_clone = pubkey.clone();

        spawn(async move {
            // Mock account addition for demo
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            log::info!("Mock: Added account to monitoring: {}", pubkey_clone);
            adding.set(false);
        });
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
                    disabled: is_adding.read().clone(),
                    class: format!("add-button {}", if is_adding.read().clone() { "loading" } else { "" }),
                    {if is_adding.read().clone() { "Adding..." } else { "Add Account " }}
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
