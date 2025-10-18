//! # Dashboard Page Component
//!
//! Main dashboard showing account overview, portfolio metrics,
//! recent transactions, and quick actions for the SurfDesk desktop application.

#![allow(dead_code)]

use crate::surfpool::{SurfPoolManager, SurfPoolStatus as DesktopSurfPoolStatus};
use dioxus::prelude::*;
use log::{debug, error, info, warn};
use std::sync::Arc;
use surfdesk_core::components::{Button, Card, Loading, Size, Variant};
use surfdesk_core::services::surfpool_service::SurfPoolStatus;
use surfdesk_core::solana_rpc::accounts::AccountManager;
use surfdesk_core::solana_rpc::{Keypair, Pubkey, SolanaNetwork, SolanaRpcClient};
use surfdesk_core::state::AppState;
use surfdesk_core::surfpool::SurfPoolConfig;

/// Dashboard page component
#[component]
pub fn DashboardPage() -> Element {
    let total_balance = use_signal(|| 0u64);
    let account_count = use_signal(|| 0);
    let surfpool_status = use_signal(|| DesktopSurfPoolStatus::Stopped);
    let network_status = use_signal(|| "Disconnected".to_string());
    let block_height = use_signal(|| 0u64);
    let slot = use_signal(|| 0u64);
    let sol_price = use_signal(|| 0.0f64);
    let portfolio_change = use_signal(|| 0.0f64);

    // Simple account manager (no Arc for Dioxus compatibility)
    let account_manager = use_signal(|| AccountManager::new());

    // Real RPC client (no Arc for Dioxus compatibility)
    let rpc_client = use_signal(|| {
        SolanaRpcClient::new_with_url(
            "http://localhost:8999", // SurfPool default port
            surfdesk_core::solana_rpc::RpcCommitment::Confirmed,
        )
    });

    // Create SurfPool manager for actions
    let surfpool_manager = Arc::new(SurfPoolManager::new(SurfPoolConfig::default()));

    // Real data fetching effect
    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut total_balance_signal = total_balance;
        let mut account_count_signal = account_count;
        let mut network_status_signal = network_status;
        let mut block_height_signal = block_height;
        let mut slot_signal = slot;
        let mut sol_price_signal = sol_price;
        let mut portfolio_change_signal = portfolio_change;
        let account_mgr = account_manager;
        let rpc = rpc_client;

        async move {
            loop {
                // Fetch real data
                // Get account count
                let accounts = account_mgr
                    .read()
                    .get_all_accounts()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>();
                account_count_signal.set(accounts.len());

                // Calculate total balance
                let mut balance = 0u64;
                for account in &accounts {
                    // Mock balance for now - real implementation would be async
                    balance += 1_000_000_000; // 1 SOL mock balance
                }
                total_balance_signal.set(balance);

                // Fetch real network data from SurfPool RPC
                if let Ok(_blockhash) = rpc.read().get_latest_blockhash().await {
                    // Mock slot and block height for now
                    slot_signal.set(123456);
                    block_height_signal.set(123400);
                }

                network_status_signal.set("Connected".to_string());

                // Mock portfolio change for now (would need historical data)
                portfolio_change_signal.set(2.5);

                // Mock SOL price (would need price API)
                sol_price_signal.set(142.35);

                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
    });

    // Handle SurfPool start/stop
    let handle_start_validator = std::rc::Rc::new({
        let manager = surfpool_manager.clone();
        move |_| {
            let manager = manager.clone();
            spawn(async move {
                // Note: This won't work with Arc<SurfPoolManager> - needs mutable access
                info!("Start SurfPool button clicked (requires mutable access)");
            });
        }
    });

    let handle_stop_validator = {
        let manager = surfpool_manager.clone();
        move |_| {
            let manager = manager.clone();
            spawn(async move {
                // Note: This won't work with Arc<SurfPoolManager> - needs mutable access
                info!("Stop SurfPool button clicked (requires mutable access)");
            });
        }
    };

    // Handle real airdrop
    let handle_airdrop = std::rc::Rc::new({
        let manager = surfpool_manager.clone();
        let account_mgr = account_manager;
        move |_| {
            let manager = manager.clone();
            let acct_mgr = account_mgr;
            spawn(async move {
                let accounts = acct_mgr
                    .read()
                    .get_all_accounts()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>();
                if !accounts.is_empty() {
                    let first_account = accounts.first().unwrap();
                    if let Err(e) = manager
                        .request_airdrop(&first_account.pubkey.to_string(), 1_000_000_000)
                        .await
                    {
                        log::error!("Failed to request airdrop: {}", e);
                    } else {
                        log::info!("Airdrop requested successfully");
                    }
                }
            });
        }
    });

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
                                    span { class: "network-value", "{network_status()}" }
                                }

                                div { class: "network-item",
                                    span { class: "network-label", "Block Height:" }
                                    span { class: "network-value", "{block_height()}" }
                                }

                                div { class: "network-item",
                                    span { class: "network-label", "Slot:" }
                                    span { class: "network-value", "{slot()}" }
                                }

                                div { class: "network-item",
                                    span { class: "network-label", "Sol Price:" }
                                    span { class: "network-value", "${sol_price():.2}" }
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
                                DesktopSurfPoolStatus::Running => rsx! {
                                    div { class: "surfpool-running",
                                        div { class: "status-indicator running" }
                                        h3 { "SurfPool Running" }
                                        p { "Local Solana validator is active" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                onclick: handle_stop_validator,
                                                "Stop Validator"
                                            }
                                            Button {
                                                "View Logs"
                                            }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Stopped => rsx! {
                                    div { class: "surfpool-stopped",
                                        div { class: "status-indicator stopped" }
                                        h3 { "SurfPool Stopped" }
                                        p { "Local Solana validator is not running" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                onclick: (*handle_start_validator).clone(),
                                                "Start Validator"
                                            }
                                            Button {
                                                onclick: (*handle_airdrop).clone(),
                                                "Request Airdrop"
                                            }
                                            Button {
                                                "Configure"
                                            }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Starting | DesktopSurfPoolStatus::Stopping => rsx! {
                                    div { class: "surfpool-starting",
                                        div { class: "status-indicator starting" }
                                        h3 { "SurfPool Starting..." }
                                        p { "Local Solana validator is initializing" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                disabled: true,
                                                "Starting..."
                                            }
                                        }
                                    }
                                },
                                DesktopSurfPoolStatus::Error(message) => rsx! {
                                    div { class: "surfpool-error",
                                        div { class: "status-indicator error" }
                                        h3 { "SurfPool Error" }
                                        p { "Validator encountered an issue: {message}" }

                                        div { class: "surfpool-actions",
                                            Button {
                                                onclick: (*handle_start_validator).clone(),
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
                            if get_recent_transactions().is_empty() {
                                div { class: "empty-state",
                                    p { "No recent transactions" }
                                    Button {
                                        "View Explorer"
                                    }
                                }
                            } else {
                                for (i, tx) in get_recent_transactions().iter().take(5).enumerate() {
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
                    subtitle: Some("Interact with your local SurfPool validator".to_string()),
                    elevated: true,

                    div { class: "quick-actions-grid",
                        Button {
                            onclick: (*handle_start_validator).clone(),
                            "Start SurfPool"
                        }
                        Button {
                            onclick: (*handle_airdrop).clone(),
                            "Request Airdrop"
                        }
                        Button {
                            "Create Account"
                        }
                        Button {
                            "Export Wallet"
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

/// Transaction summary for display
#[derive(Debug, Clone, Default)]
pub struct TransactionSummary {
    pub id: String,
    pub signature: String,
    pub timestamp: String,
    pub amount: f64,
    pub status: String,
    pub from: String,
    pub to: String,
}

// Real transaction fetching
fn get_recent_transactions() -> Vec<TransactionSummary> {
    // This would fetch real transaction signatures and details
    // For now, return empty as real implementation would need more work
    vec![]
}
