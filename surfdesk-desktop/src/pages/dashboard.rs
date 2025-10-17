//! # Dashboard Page Component
//!
//! Main dashboard showing account overview, portfolio metrics,
//! recent transactions, and quick actions for the SurfDesk desktop application.

use crate::surfpool::{SurfPoolManager, SurfPoolStatus};
use dioxus::prelude::*;
use std::sync::Arc;
use surfdesk_core::components::{Button, Card, Size, Variant};

/// Dashboard page component
#[component]
pub fn DashboardPage() -> Element {
    let mut total_balance = use_signal(|| 2_456_789_000u64); // 2.456 SOL in lamports
    let mut account_count = use_signal(|| 3);

    // Mock data for portfolio change
    let portfolio_change = || 2.5;

    rsx! {
        div { class: "dashboard-page",

            // Header
            div { class: "dashboard-header",
                h1 { "SurfDesk Dashboard" }
                p { "Your Solana Account Management Hub" }
            }

            // Main dashboard content
            div { class: "dashboard-content",

                // Stats cards row
                div { class: "stats-row",

                    // Total balance card
                    Card {
                        title: Some("Total Balance".to_string()),
                        size: Size::Large,
                        children: rsx! {
                            div { class: "stat-value",
                                {format!("{:.3} SOL", total_balance() as f64 / 1_000_000_000.0)}
                            }
                            div { class: "stat-description",
                                "Across all accounts"
                            }
                        }
                    }

                    // Account count card
                    Card {
                        title: Some("Accounts".to_string()),
                        size: Size::Medium,
                        children: rsx! {
                            div { class: "stat-value",
                                "{account_count()}"
                            }
                            div { class: "stat-description",
                                "Active accounts"
                            }
                        }
                    }

                    // Portfolio change card
                    Card {
                        title: Some("24h Change".to_string()),
                        size: Size::Medium,
                        children: rsx! {
                            div { class: "stat-value",
                                {format!("{:+.1}%", portfolio_change())}
                            }
                            div { class: "stat-description",
                                "Portfolio performance"
                            }
                        }
                    }

                    // Network status card
                    Card {
                        title: Some("Network Status".to_string()),
                        size: Size::Small,
                        children: rsx! {
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
                }

                // Second row - SurfPool and recent activity
                div { class: "dashboard-row",

                    // SurfPool status card
                    Card {
                        variant: Variant::Default,
                        title: Some("SurfPool Status".to_string()),
                        elevated: true,
                        size: Size::Large,

                        div { class: "surfpool-dashboard",
                            match surfpool_status() {
                                SurfPoolStatus::Running { .. } => rsx! {
                                    div { class: "surfpool-running",
                                        div { class: "status-indicator running" }
                                        h3 { "SurfPool Running" }
                                        p { "Local Solana validator is active" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                "Stop Validator"
                                            }
                                            Button {
                                                "View Logs"
                                            }
                                        }
                                    }
                                },
                                SurfPoolStatus::Stopped => rsx! {
                                    div { class: "surfpool-stopped",
                                        div { class: "status-indicator stopped" }
                                        h3 { "SurfPool Stopped" }
                                        p { "Local Solana validator is not running" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                "Start Validator"
                                            }
                                            Button {
                                                "Configure"
                                            }
                                        }
                                    }
                                },
                                _ => rsx! {
                                    div { class: "surfpool-error",
                                        div { class: "status-indicator error" }
                                        h3 { "SurfPool Error" }
                                        p { "Validator encountered an issue" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                "Restart"
                                            }
                                            Button {
                                                "Troubleshoot"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Recent transactions
                    Card {
                        variant: Variant::Default,
                        title: Some("Recent Transactions".to_string()),
                        elevated: true,
                        size: Size::Large,

                        div { class: "transactions-list",
                            if recent_transactions().is_empty() {
                                div { class: "empty-state",
                                    p { "No recent transactions" }
                                    Button {
                                        "View Explorer"
                                    }
                                }
                            } else {
                                for (i, tx) in recent_transactions().iter().take(5).enumerate() {
                                    div { class: "transaction-item",
                                        div { class: "transaction-info",
                                            span { class: "transaction-signature",
                                                {format!("{}...", &tx.signature[..8.min(tx.signature.len())])}
                                            }
                                            span { class: "transaction-time",
                                                {format!("{} min ago", tx.timestamp)}
                                            }
                                        }
                                        div { class: "transaction-amount",
                                            span { class: "amount",
                                                {format!("{:.4} SOL", tx.amount)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Quick actions section
                Card {
                    variant: Variant::Default,
                    title: Some("Quick Actions".to_string()),
                    subtitle: Some("Common tasks you can perform quickly".to_string()),
                    elevated: true,

                    div { class: "quick-actions-grid",
                        Button {
                            "Start SurfPool"
                        }
                        Button {
                            "Create Account"
                        }
                        Button {
                            "Deploy Program"
                        }
                        Button {
                            "View Analytics"
                        }
                    }
                }
            }
        }
    }
}

// Mock functions for demo
fn surfpool_status() -> SurfPoolStatus {
    SurfPoolStatus::Stopped
}

fn recent_transactions() -> Vec<TransactionSummary> {
    vec![]
}

/// Transaction summary for display
#[derive(Debug, Clone)]
pub struct TransactionSummary {
    pub id: String,
    pub signature: String,
    pub timestamp: String,
    pub amount: f64,
    pub status: String,
    pub from: String,
    pub to: String,
}

impl Default for TransactionSummary {
    fn default() -> Self {
        Self {
            id: "tx_123".to_string(),
            signature: "abc123def456".to_string(),
            timestamp: "5".to_string(),
            amount: 0.5,
            status: "confirmed".to_string(),
            from: "11111111111111111111111111111112".to_string(),
            to: "22222222222222222222222222222223".to_string(),
        }
    }
}

/// Quick action card component
#[component]
fn QuickActionCard(
    icon: String,
    title: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "quick-action-card",
            onclick: move |evt| onclick.call(evt),
            div { class: "quick-action-icon", "{icon}" }
            div { class: "quick-action-content",
                h4 { "{title}" }
                p { "{description}" }
            }
        }
    }
}
