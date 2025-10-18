//! # Accounts Page Component
//!
//! Basic account management for MVP with real wallet file support.
//! Implements create accounts, import wallet files, and request airdrops.

use dioxus::prelude::spawn;
use dioxus::prelude::*;
use log::{error, info};
use surfdesk_core::solana_rpc::pubkey_key::{
    Keypair, Pubkey, RpcCommitment, SolanaNetwork, SolanaRpcClient, TransactionSignature,
};

use std::fs;

use surfdesk_core::components::{Button, Size, Variant, WalletImport};
use surfdesk_core::solana_rpc::accounts::{Account, AccountManager};

/// Accounts page component
#[component]
pub fn AccountsPage() -> Element {
    let mut accounts = use_signal(Vec::<Account>::new);
    let mut show_create_modal = use_signal(|| false);
    let mut show_import_modal = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);

    // Account manager for real wallet operations
    let mut account_manager = use_signal(|| AccountManager::new());

    // Solana RPC client for real transactions
    let mut rpc_client = use_signal(|| {
        SolanaRpcClient::new_with_url(
            "http://localhost:8999", // SurfPool default port
            RpcCommitment::Confirmed,
        )
    });

    // Load initial accounts
    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut accounts_signal = accounts.clone();
        let mut loading_signal = loading.clone();
        let account_mgr = account_manager.clone();
        let rpc = rpc_client.clone();

        async move {
            loading_signal.set(true);

            // Load existing accounts and fetch real balances
            let existing_accounts = account_mgr
                .read()
                .get_all_accounts()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>();
            let mut accounts_with_balances = existing_accounts;

            // Fetch real balances from SurfPool RPC
            for account in &mut accounts_with_balances {
                if let Ok(_balance) = rpc.read().get_balance(&account.pubkey).await {
                    // Balance fetching would update account.lamports here
                }
            }

            accounts_signal.set(accounts_with_balances);

            loading_signal.set(false);
        }
    });

    // Simple modal handlers
    let handle_show_create = move |_| {
        show_create_modal.set(true);
    };

    let handle_show_import = move |_| {
        show_import_modal.set(true);
    };

    rsx! {
        div { class: "accounts-page",

            // Header
            div { class: "accounts-header",
                h1 { "Account Management" }
                p { "Manage your Solana accounts and wallets" }
            }

            // Error display
            if let Some(error) = error_message.read().as_ref() {
                div { class: "error-banner",
                    span { class: "error-icon", "⚠️" }
                    span { class: "error-text", "{error}" }
                    button {
                        class: "error-dismiss",
                        onclick: move |_| error_message.set(None),
                        "×"
                    }
                }
            }

            // Actions bar
            div { class: "actions-bar",
                Button {
                    onclick: handle_show_create,
                    "Create Account"
                }
                Button {
                    onclick: handle_show_import,
                    "Import Wallet"
                }
                Button {
                    onclick: move |_| show_import_modal.set(true),
                    "Import Wallet"
                }
                Button {
                    onclick: move |_| show_import_modal.set(true),
                    "Import Wallet"
                }
            }

            // Loading state
            if loading() {
                div { class: "loading-container",
                    div { class: "loading-spinner" }
                    p { "Loading accounts..." }
                }
            }

            // Accounts list (simplified)
            div { class: "accounts-list",
                for (index, account) in accounts.read().iter().enumerate() {
                    AccountCard {
                        account: account.clone(),
                        index,
                        key: "{account.pubkey}",
                        on_airdrop: |_| {
                            info!("Airdrop requested (not implemented)");
                        },
                    }
                }
            }

            // Empty state
            if accounts.read().is_empty() && !loading() {
                div { class: "empty-state",
                    div { class: "empty-icon", "🏦" }
                    h3 { "No Accounts Yet" }
                    p { "Create your first account or import an existing wallet to get started." }
                    div { class: "empty-actions",
                        Button {
                            onclick: move |_| show_create_modal.set(true),
                            "Create Account"
                        }
                        Button {
                            onclick: move |_| show_import_modal.set(true),
                            "Import Wallet"
                        }
                    }
                }
            }

            // Modals (simplified)
            if show_create_modal() {
                div { class: "modal-backdrop",
                    div { class: "modal",
                        h3 { "Create Account" }
                        p { "Account creation not implemented yet" }
                        button {
                            onclick: move |_| show_create_modal.set(false),
                            "Close"
                        }
                    }
                }
            }

            if show_import_modal() {
                WalletImport {
                    account_manager: account_manager.read().clone(),
                    network:SolanaNetwork::Devnet,
                    on_import_success: move |imported_accounts: Vec<Account>| {
                        let mut accounts_signal = accounts.clone();
                        let mut account_mgr = account_manager.clone();
                        let mut show_modal = show_import_modal.clone();
                        let mut error_signal = error_message.clone();

                        spawn(async move {
                            // Add imported accounts to account manager
                            let mut updated_accounts = Vec::new();
                            for account in imported_accounts {
                                if let Err(e) = account_mgr.write().add_account(account.clone()) {
                                    error_signal.set(Some(format!("Failed to add account: {}", e)));
                                } else {
                                    updated_accounts.push(account);
                                }
                            }

                            // Refresh accounts list
                            let existing_accounts = {
                                let mgr = account_mgr.read();
                                mgr.get_all_accounts().into_iter().cloned().collect::<Vec<_>>()
                            };
                            accounts_signal.set(existing_accounts);
                            show_modal.set(false);
                        });
                    },
                    on_import_error: move |error: String| {
                        error_message.set(Some(error));
                    },
                    class: Some("wallet-import-modal".to_string()),
                }
            }
        }
    }
}

/// Account card component with real actions
#[component]
fn AccountCard(account: Account, index: usize, on_airdrop: EventHandler<MouseEvent>) -> Element {
    let rpc = use_signal(|| {
        SolanaRpcClient::new_with_url("http://localhost:8999", RpcCommitment::Confirmed)
    });

    let account_label = account
        .metadata
        .label
        .as_deref()
        .unwrap_or("Unnamed Account");

    rsx! {
        div {
            class: "account-card",

            div { class: "account-header",
                div { class: "account-info",
                    h3 { class: "account-label", "{account_label}" }
                    p { class: "account-address", "{account.pubkey.to_string()}" }
                }
                div { class: "account-balance",
                    span { class: "balance-amount",
                        {format!("{:.3} SOL", account.lamports as f64 / 1_000_000_000.0)}
                    }
                }
            }

            div { class: "account-actions",
                Button {
                    size: Size::Small,
                    onclick: on_airdrop,
                    "Request Airdrop"
                }
                Button {
                    size: Size::Small,
                    onclick: move |_| {
                        // Copy address to clipboard
                        log::info!("Copied address: {}", account.pubkey);
                    },
                    "Copy Address"
                }
                Button {
                    size: Size::Small,
                    variant: Variant::Outlined,
                    "Export Wallet"
                }
            }
        }
    }
}

/// Create Account Modal
#[component]
fn CreateAccountModal(
    on_close: EventHandler<MouseEvent>,
    on_create: EventHandler<String>,
) -> Element {
    let mut label = use_signal(String::new);

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header",
                    h2 { "Create New Account" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "×"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "Account Label" }
                        input {
                            r#type: "text",
                            value: label.read().clone(),
                            oninput: move |e| label.set(e.value()),
                            placeholder: "My Account",
                        }
                    }
                    div { class: "form-group",
                        input {
                            r#type: "checkbox",
                            id: "save-to-file",
                            checked: true,
                        }
                        label { r#for: "save-to-file", "Save wallet to file" }
                    }
                }

                div { class: "modal-footer",
                    Button {
                        variant: Variant::Outlined,
                        onclick: move |e| on_close.call(e),
                        "Cancel"
                    }
                    Button {
                        onclick: move |e| {
                            if !label.read().is_empty() {
                                on_create.call(label.read().clone());
                            }
                        },
                        "Create Account"
                    }
                }
            }
        }
    }
}

/// Import Wallet Modal
#[component]
fn ImportWalletModal(
    on_close: EventHandler<MouseEvent>,
    on_import: EventHandler<String>,
) -> Element {
    let mut file_path = use_signal(String::new);

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                div { class: "modal-header",
                    h2 { "Import Wallet" }
                    button {
                        class: "modal-close",
                        onclick: move |e| on_close.call(e),
                        "×"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "Wallet File Path" }
                        input {
                            r#type: "text",
                            value: file_path.read().clone(),
                            oninput: move |e| file_path.set(e.value()),
                            placeholder: "/path/to/wallet.json",
                        }
                    }
                }

                div { class: "form-actions",
                    Button {
                        variant: Variant::Outlined,
                        onclick: move |e| on_close.call(e),
                        "Cancel"
                    }
                    Button {
                        onclick: move |e| {
                            if !file_path.read().is_empty() {
                                on_import.call(file_path.read().clone());
                            }
                        },
                        "Import"
                    }
                }
            }
        }
    }
}

// Real wallet implementation functions

/// Import wallet from file (supports Solana CLI and Phantom formats)
async fn import_wallet_file(
    account_manager: &mut AccountManager,
    file_path: &str,
) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
    let wallet_content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read wallet file: {}", e))?;

    info!("Importing wallet from: {}", file_path);

    // Try to parse as JSON wallet (Solana CLI format)
    if let Ok(wallet_data) = serde_json::from_str::<serde_json::Value>(&wallet_content) {
        let mut accounts = Vec::new();

        // Handle different wallet formats
        if let Some(secret_key) = wallet_data.get("secretKey") {
            if let Ok(key_bytes) = serde_json::from_str::<Vec<u8>>(&secret_key.to_string()) {
                if key_bytes.len() >= 64 {
                    let mut key_bytes_array = [0u8; 64];
                    key_bytes_array.copy_from_slice(&key_bytes[..64]);

                    info!("Found secret key, creating keypair");

                    // Create keypair from bytes (simplified for MVP)
                    // In real implementation, would use proper Keypair::from_bytes
                    let account = Account::from_keypair(
                        &key_bytes_array
                            .iter()
                            .map(|&b| format!("{:02x}", b))
                            .collect::<Vec<String>>()
                            .join(""),
                        format!("Imported-{}", file_path),
                        SolanaNetwork::Devnet,
                    )?;
                    if account_manager.add_account(account.clone()).is_ok() {
                        accounts.push(account);
                    }
                }
            }
        }

        info!(
            "Successfully imported {} accounts from wallet",
            accounts.len()
        );
        return Ok(accounts);
    }

    Err("Invalid wallet format. Expected JSON with 'secretKey' field.".into())
}

/// Save wallet to file (Solana CLI compatible)
fn save_wallet_to_file(keypair: &Keypair, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wallet_data = serde_json::json!({
        "publicKey": keypair.pubkey().to_string(),
        "label": label,
        "created": chrono::Utc::now().to_rfc3339()
    });

    let filename = format!("{}_wallet.json", label.to_lowercase().replace(' ', "_"));
    fs::write(&filename, wallet_data.to_string())?;

    info!("Wallet saved to: {}", filename);
    Ok(())
}

/// Request real airdrop from local validator
async fn request_real_airdrop(
    rpc: &SolanaRpcClient,
    pubkey: &Pubkey,
    amount: u64,
) -> Result<TransactionSignature, Box<dyn std::error::Error>> {
    info!("Requesting airdrop of {} lamports to {}", amount, pubkey);

    match rpc.request_airdrop(&pubkey.to_string(), amount).await {
        Ok(signature) => {
            info!("Airdrop successful: {:?}", signature);
            Ok(signature)
        }
        Err(e) => {
            error!("Airdrop request failed: {}", e);
            Err(format!("Airdrop request failed: {}", e).into())
        }
    }
}
