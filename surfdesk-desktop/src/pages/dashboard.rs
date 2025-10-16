//! # Dashboard Page Component
//!
//! Main dashboard showing account overview, portfolio metrics,
//! recent transactions, and quick actions for the SurfDesk desktop application.

use crate::components::*;
use crate::surfpool::{SurfPoolManager, SurfPoolStatus};
use dioxus::prelude::*;
use std::sync::Arc;

/// Dashboard page component
#[component]
pub fn DashboardPage(surfpool_manager: Arc<SurfPoolManager>) -> Element {
    let mut total_balance = use_signal(|| 2_456_789_000); // 2.456 SOL in lamports
    let mut account_count = use_signal(|| 3);
    let mut recent_transactions = use_signal(Vec::<TransactionSummary>::new);
    let mut portfolio_change = use_signal(|| 12.5);
    let surfpool_status = use_signal(|| surfpool_manager.get_status());

    // Initialize mock data
    use_effect(move || {
        let mut transactions = vec![
            TransactionSummary {
                id: "1".to_string(),
                type_: TransactionType::Transfer,
                amount: 0_500_000_000, // 0.5 SOL
                from: "Your Account".to_string(),
                to: "Destination".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(5),
                status: TransactionStatus::Confirmed,
                signature: "5KJp7z3L4X...".to_string(),
            },
            TransactionSummary {
                id: "2".to_string(),
                type_: TransactionType::Receive,
                amount: 1_200_000_000, // 1.2 SOL
                from: "Sender".to_string(),
                to: "Your Account".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::hours(2),
                status: TransactionStatus::Confirmed,
                signature: "7LmQ8x4N5Y...".to_string(),
            },
            TransactionSummary {
                id: "3".to_string(),
                type_: TransactionType::Swap,
                amount: 0_750_000_000, // 0.75 SOL
                from: "Your Account".to_string(),
                to: "DEX Contract".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::hours(6),
                status: TransactionStatus::Confirmed,
                signature: "8MnR9y5O6Z...".to_string(),
            },
        ];
        transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        recent_transactions.set(transactions);
    });

    // Update SurfPool status periodically
    use_coroutine(|_| {
        let manager = Arc::clone(&surfpool_manager);
        let status = surfpool_status.clone();

        async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
            loop {
                interval.tick().await;
                status.set(manager.get_status());
            }
        }
    });

    let create_account = move |_| {
        log::info!("Create account clicked");
        // TODO: Open account creation modal
    };

    let import_account = move |_| {
        log::info!("Import account clicked");
        // TODO: Open account import modal
    };

    let request_airdrop = move |_| {
        log::info!("Request airdrop clicked");
        // TODO: Request airdrop from current network
    };

    rsx! {
        div { class: "dashboard-page",

            // Page header
            div { class: "page-header",
                div { class: "page-title-section",
                    h1 { class: "page-title", "Dashboard" }
                    p { class: "page-subtitle", "Overview of your Solana accounts and portfolio" }
                }

                div { class: "page-actions",
                    Button {
                        variant: ButtonVariant::Primary,
                        icon: Some("🏄".to_string()),
                        onclick: create_account,
                        "Create Account"
                    }

                    Button {
                        variant: ButtonVariant::Secondary,
                        icon: Some("📥".to_string()),
                        onclick: import_account,
                        "Import Account"
                    }

                    Button {
                        variant: ButtonVariant::Tertiary,
                        icon: Some("💰".to_string()),
                        onclick: request_airdrop,
                        "Request Airdrop"
                    }
                }
            }

            // Main dashboard content
            div { class: "dashboard-content",

                // Stats cards row
                div { class: "stats-row",

                    // Total balance card
                    StatsCard {
                        title: "Total Balance".to_string(),
                        value: format!("{:.3} SOL", total_balance() as f64 / 1_000_000_000.0),
                        description: Some("Across all accounts".to_string()),
                        progress: None,
                        variant: CardVariant::Primary,
                        icon: Some("💎".to_string()),
                    }

                    // Account count card
                    StatsCard {
                        title: "Accounts".to_string(),
                        value: account_count().to_string(),
                        description: Some("Active accounts".to_string()),
                        progress: None,
                        variant: CardVariant::Secondary,
                        icon: Some("🏦".to_string()),
                    }

                    // Portfolio change card
                    StatsCard {
                        title: "24h Change".to_string(),
                        value: format!("{:+.1}%", portfolio_change()),
                        description: Some("Portfolio performance".to_string()),
                        progress: Some(if portfolio_change() > 0.0 { 75.0 } else { 25.0 }),
                        variant: if portfolio_change() > 0.0 { CardVariant::Success } else { CardVariant::Warning },
                        icon: Some(if portfolio_change() > 0.0 { "📈".to_string() } else { "📉".to_string() }),
                    }

                    // Network status card
                    Card {
                        variant: CardVariant::Default,
                        title: Some("Network Status".to_string()),
                        icon: Some("🌐".to_string()),
                        elevated: true,

                        div { class: "network-status",
                            div { class: "network-item",
                                span { class: "network-label", "Current Network:" }
                                span { class: "network-value", "Devnet" }
                            }

                            div { class: "network-item",
                                span { class: "network-label", "Block Height:" }
                                span { class: "network-value", "185,423,109" }
                            }

                            div { class: "network-item",
                                span { class: "network-label", "Slot:" }
                                span { class: "network-value", "185,423,567" }
                            }

                            div { class: "network-item",
                                span { class: "network-label", "Sol Price:" }
                                span { class: "network-value", "$142.35" }
                            }
                        }
                    }
                }

                // Second row - SurfPool and recent activity
                div { class: "dashboard-row",

                    // SurfPool status card
                    Card {
                        variant: CardVariant::Gradient,
                        title: Some("SurfPool Status".to_string()),
                        icon: Some("🌊".to_string()),
                        elevated: true,
                        size: Size::Large,

                        div { class: "surfpool-dashboard",
                            match surfpool_status() {
                                SurfPoolStatus::Running { pid, uptime, port, rpc_url, .. } => {
                                    rsx! {
                                        div { class: "surfpool-running",
                                            div { class: "surfpool-status-row",
                                                span { class: "status-indicator status-online" }
                                                span { class: "status-text", "Running" }
                                            }

                                            div { class: "surfpool-metrics",
                                                div { class: "metric-item",
                                                    span { class: "metric-label", "Process ID" }
                                                    span { class: "metric-value", "{pid}" }
                                                }

                                                div { class: "metric-item",
                                                    span { class: "metric-label", "Port" }
                                                    span { class: "metric-value", "{port}" }
                                                }

                                                div { class: "metric-item",
                                                    span { class: "metric-label", "Uptime" }
                                                    span { class: "metric-value", "{uptime}s" }
                                                }

                                                div { class: "metric-item",
                                                    span { class: "metric-label", "RPC URL" }
                                                    span { class: "metric-value", "{rpc_url}" }
                                                }
                                            }

                                            div { class: "surfpool-actions",
                                                Button {
                                                    variant: ButtonVariant::Error,
                                                    size: Size::Small,
                                                    icon: Some("⏹️".to_string()),
                                                    "Stop SurfPool"
                                                }

                                                Button {
                                                    variant: ButtonVariant::Ghost,
                                                    size: Size::Small,
                                                    icon: Some("🔧".to_string()),
                                                    "Manage"
                                                }
                                            }
                                        }
                                    }
                                }
                                SurfPoolStatus::Stopped => {
                                    rsx! {
                                        div { class: "surfpool-stopped",
                                            div { class: "surfpool-status-row",
                                                span { class: "status-indicator status-offline" }
                                                span { class: "status-text", "Stopped" }
                                            }

                                            p { class: "surfpool-description",
                                                "Start SurfPool to run a local Solana validator for development and testing."
                                            }

                                            div { class: "surfpool-actions",
                                                Button {
                                                    variant: ButtonVariant::Primary,
                                                    icon: Some("▶️".to_string()),
                                                    "Start SurfPool"
                                                }

                                                Button {
                                                    variant: ButtonVariant::Ghost,
                                                    icon: Some("⚙️".to_string()),
                                                    "Configure"
                                                }
                                            }
                                        }
                                    }
                                }
                                SurfPoolStatus::Starting => {
                                    rsx! {
                                        div { class: "surfpool-starting",
                                            div { class: "surfpool-status-row",
                                                span { class: "status-indicator status-starting animate-pulse" }
                                                span { class: "status-text", "Starting..." }
                                            }

                                            p { class: "surfpool-description",
                                                "SurfPool is starting up. This may take a few moments..."
                                            }
                                        }
                                    }
                                }
                                SurfPoolStatus::Error { message, .. } => {
                                    rsx! {
                                        div { class: "surfpool-error",
                                            div { class: "surfpool-status-row",
                                                span { class: "status-indicator status-error" }
                                                span { class: "status-text", "Error" }
                                            }

                                            p { class: "surfpool-error-message",
                                                "{message}"
                                            }

                                            div { class: "surfpool-actions",
                                                Button {
                                                    variant: ButtonVariant::Primary,
                                                    icon: Some("🔄".to_string()),
                                                    "Retry"
                                                }

                                                Button {
                                                    variant: ButtonVariant::Ghost,
                                                    icon: Some("📋".to_string()),
                                                    "View Logs"
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    rsx! {
                                        div { class: "surfpool-unknown",
                                            p { "Unknown SurfPool status" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Recent transactions
                    Card {
                        variant: CardVariant::Default,
                        title: Some("Recent Transactions".to_string()),
                        icon: Some("📋".to_string()),
                        elevated: true,
                        size: Size::Large,

                        div { class: "transactions-list",
                            if recent_transactions().is_empty() {
                                div { class: "empty-state",
                                    span { class: "empty-icon", "📭" }
                                    p { class: "empty-text", "No recent transactions" }
                                    p { class: "empty-description", "Your transaction history will appear here" }
                                }
                            } else {
                                div { class: "transactions-container",
                                    for (index, transaction) in recent_transactions().iter().enumerate().take(5) {
                                        TransactionItem {
                                            transaction: transaction.clone(),
                                            index: index,
                                        }
                                    }
                                }

                                if recent_transactions().len() > 5 {
                                    div { class: "view-all",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: Size::Small,
                                            full_width: true,
                                            "View All Transactions"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Quick actions section
                Card {
                    variant: CardVariant::Glass,
                    title: Some("Quick Actions".to_string()),
                    subtitle: Some("Common tasks you can perform quickly".to_string()),
                    icon: Some("⚡".to_string()),
                    elevated: true,

                    div { class: "quick-actions-grid",
                        QuickActionCard {
                            icon: "🏄".to_string(),
                            title: "Start SurfPool".to_string(),
                            description: "Launch local validator for development".to_string(),
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                log::info!("Start SurfPool quick action");
                            },
                        }

                        QuickActionCard {
                            icon: "💰".to_string(),
                            title: "Request Airdrop".to_string(),
                            description: "Get test SOL for development".to_string(),
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| {
                                log::info!("Request airdrop quick action");
                            },
                        }

                        QuickActionCard {
                            icon: "🔧".to_string(),
                            title: "Build Transaction".to_string(),
                            description: "Create and sign transactions".to_string(),
                            variant: ButtonVariant::Tertiary,
                            onclick: move |_| {
                                log::info!("Build transaction quick action");
                            },
                        }

                        QuickActionCard {
                            icon: "📊".to_string(),
                            title: "View Analytics".to_string(),
                            description: "Analyze your portfolio performance".to_string(),
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| {
                                log::info!("View analytics quick action");
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Transaction summary for display
#[derive(Debug, Clone)]
pub struct TransactionSummary {
    pub id: String,
    pub type_: TransactionType,
    pub amount: u64,
    pub from: String,
    pub to: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: TransactionStatus,
    pub signature: String,
}

/// Transaction types
#[derive(Debug, Clone)]
pub enum TransactionType {
    Transfer,
    Receive,
    Swap,
    Stake,
    Program,
}

/// Transaction status
#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Transaction item component
#[component]
fn TransactionItem(transaction: TransactionSummary, index: usize) -> Element {
    let status_color = match transaction.status {
        TransactionStatus::Confirmed => "success",
        TransactionStatus::Pending => "warning",
        TransactionStatus::Failed => "error",
    };

    let type_icon = match transaction.type_ {
        TransactionType::Transfer => "💸",
        TransactionType::Receive => "📥",
        TransactionType::Swap => "🔄",
        TransactionType::Stake => "🔒",
        TransactionType::Program => "⚙️",
    };

    let amount_formatted = format!("{:.3} SOL", transaction.amount as f64 / 1_000_000_000.0);
    let time_ago = format_time_ago(transaction.timestamp);

    rsx! {
        div { class: "transaction-item",
            div { class: "transaction-icon",
                span { "{type_icon}" }
            }

            div { class: "transaction-details",
                div { class: "transaction-header",
                    span { class: "transaction-type", "{transaction.type_:?}" }
                    span { class: "transaction-amount", "{amount_formatted}" }
                }

                div { class: "transaction-meta",
                    span { class: "transaction-time", "{time_ago}" }
                    span { class: "transaction-signature", "{transaction.signature}" }
                }
            }

            div { class: "transaction-status",
                span { class: format!("status-badge status-{}", status_color),
                    match transaction.status {
                        TransactionStatus::Confirmed => "✓",
                        TransactionStatus::Pending => "⏳",
                        TransactionStatus::Failed => "✗",
                    }
                }
            }
        }
    }
}

/// Quick action card component
#[component]
fn QuickActionCard(
    icon: String,
    title: String,
    description: String,
    variant: ButtonVariant,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "quick-action-card",
            onclick: onclick,

            div { class: "quick-action-icon",
                span { "{icon}" }
            }

            div { class: "quick-action-content",
                h3 { class: "quick-action-title", "{title}" }
                p { class: "quick-action-description", "{description}" }
            }

            div { class: "quick-action-arrow",
                span { "→" }
            }
        }
    }
}

/// Format timestamp as "X minutes/hours ago"
fn format_time_ago(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(timestamp);

    if duration.num_minutes() < 60 {
        format!("{} min ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours ago", duration.num_hours())
    } else {
        format!("{} days ago", duration.num_days())
    }
}
