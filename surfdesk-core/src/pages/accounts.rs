//! Account Manager Page Component
//!
//! This page provides the interface for managing Solana accounts, including
//! creating new accounts, viewing account details, managing balances, and
//! handling account operations like transfers and stake management.

use dioxus::prelude::*;

/// Account Manager page component
#[component]
pub fn AccountManager() -> Element {
    let accounts = use_signal(|| {
        vec![
            Account {
                address: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
                balance: 1000000000, // 1 SOL in lamports
                label: "Main Account".to_string(),
                created_at: "2025-10-19".to_string(),
            },
            Account {
                address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                balance: 500000000, // 0.5 SOL
                label: "Test Account".to_string(),
                created_at: "2025-10-18".to_string(),
            },
        ]
    });

    let selected_account = use_signal(|| None::<Account>);
    let show_create_modal = use_signal(|| false);
    let show_import_modal = use_signal(|| false);
    let new_account_label = use_signal(|| "".to_string());

    rsx! {
        div { class: "account-manager-page",
            div { class: "page-header",
                h1 { "Account Manager" }
                p { "Manage your Solana accounts and wallets" }
            }

            // Action Buttons
            div { class: "action-bar",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_create_modal.set(true),
                    "➕ Create Account"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| show_import_modal.set(true),
                    "📥 Import Account"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        // TODO: Refresh account balances
                        tracing::info!("Refreshing account balances");
                    },
                    "🔄 Refresh"
                }
            }

            // Accounts Grid
            div { class: "accounts-grid",
                for (index, account) in accounts().iter().enumerate() {
                    AccountCard {
                        account: account.clone(),
                        is_selected: selected_account()
                            .as_ref()
                            .map(|a| a.address == account.address)
                            .unwrap_or(false),
                        on_select: move |_| {
                            selected_account.set(Some(account.clone()));
                        },
                        on_delete: move |_| {
                            let mut accs = accounts.write();
                            accs.remove(index);
                            if selected_account()
                                .as_ref()
                                .map(|a| a.address == account.address)
                                .unwrap_or(false)
                            {
                                selected_account.set(None);
                            }
                        },
                    }
                }
            }

            // Account Details
            if let Some(account) = selected_account() {
                div { class: "account-details-section",
                    h2 { "Account Details" }
                    AccountDetails {
                        account: account.clone(),
                        on_update: move |updated_account| {
                            // Find and update the account in the list
                            for (index, acc) in accounts().iter().enumerate() {
                                if acc.address == updated_account.address {
                                    accounts.write()[index] = updated_account;
                                    break;
                                }
                            }
                            selected_account.set(Some(updated_account));
                        },
                    }
                }
            }

            // Create Account Modal
            if show_create_modal() {
                CreateAccountModal {
                    on_close: move |_| show_create_modal.set(false),
                    on_create: move |account| {
                        accounts.write().push(account);
                        show_create_modal.set(false);
                    },
                }
            }

            // Import Account Modal
            if show_import_modal() {
                ImportAccountModal {
                    on_close: move |_| show_import_modal.set(false),
                    on_import: move |account| {
                        accounts.write().push(account);
                        show_import_modal.set(false);
                    },
                }
            }
        }
    }
}

/// Account data structure
#[derive(Clone, Debug, PartialEq)]
pub struct Account {
    pub address: String,
    pub balance: u64, // in lamports
    pub label: String,
    pub created_at: String,
}

impl Account {
    /// Convert balance from lamports to SOL
    pub fn balance_sol(&self) -> f64 {
        self.balance as f64 / 1_000_000_000.0
    }
}

/// Account card component
#[component]
fn AccountCard(
    account: Account,
    is_selected: bool,
    on_select: EventHandler<MouseEvent>,
    on_delete: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "account-card {if is_selected { 'selected' } else { '' }}",
            onclick: on_select,

            div { class: "account-card-header",
                div { class: "account-info",
                    h3 { class: "account-label", "{account.label}" }
                    p { class: "account-address",
                        "{format_address(&account.address)}"
                    }
                }
                button {
                    class: "account-delete",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_delete.call(evt);
                    },
                    "🗑️"
                }
            }

            div { class: "account-balance",
                div { class: "balance-amount",
                    span { class: "balance-value", "{account.balance_sol():.4}" }
                    span { class: "balance-currency", "SOL" }
                }
                div { class: "balance-lamports",
                    "{account.balance} lamports"
                }
            }

            div { class: "account-meta",
                span { class: "account-date", "Created: {account.created_at}" }
                div { class: "account-status",
                    div { class: "status-dot status-active" }
                    span { "Active" }
                }
            }
        }
    }
}

/// Account details component
#[component]
fn AccountDetails(account: Account, on_update: EventHandler<Account>) -> Element {
    let new_label = use_signal(|| account.label.clone());
    let show_receive_modal = use_signal(|| false);
    let show_send_modal = use_signal(|| false);

    rsx! {
        div { class: "account-details",
            div { class: "details-grid",
                // Basic Information
                div { class: "detail-section",
                    h3 { "Information" }
                    div { class: "detail-item",
                        label { "Label:" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            value: "{new_label}",
                            onchange: move |evt| new_label.set(evt.value())
                        }
                    }
                    div { class: "detail-item",
                        label { "Address:" }
                        div { class: "address-display",
                            span { "{account.address}" }
                            button {
                                class: "btn btn-small btn-secondary",
                                onclick: move |_| {
                                    // TODO: Copy address to clipboard
                                    tracing::info!("Copying address to clipboard");
                                },
                                "📋"
                            }
                        }
                    }
                    div { class: "detail-item",
                        label { "Created:" }
                        span { "{account.created_at}" }
                    }
                }

                // Balance Information
                div { class: "detail-section",
                    h3 { "Balance" }
                    div { class: "balance-display",
                        div { class: "balance-primary",
                            span { class: "balance-value", "{account.balance_sol():.4}" }
                            span { class: "balance-currency", "SOL" }
                        }
                    }
                    div { class: "detail-item",
                        label { "Lamports:" }
                        span { "{account.balance}" }
                    }
                }

                // Quick Actions
                div { class: "detail-section",
                    h3 { "Quick Actions" }
                    div { class: "action-buttons",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| show_receive_modal.set(true),
                            "📥 Receive"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| show_send_modal.set(true),
                            "📤 Send"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                // TODO: View transaction history
                                tracing::info!("Viewing transaction history");
                            },
                            "📜 History"
                        }
                    }
                }
            }

            div { class: "details-actions",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let mut updated = account.clone();
                        updated.label = new_label();
                        on_update.call(updated);
                    },
                    "💾 Save Changes"
                }
            }
        }

        // Receive Modal
        if show_receive_modal() {
            ReceiveModal {
                address: account.address.clone(),
                on_close: move |_| show_receive_modal.set(false),
            }
        }

        // Send Modal
        if show_send_modal() {
            SendModal {
                account: account.clone(),
                on_close: move |_| show_send_modal.set(false),
            }
        }
    }
}

/// Create account modal component
#[component]
fn CreateAccountModal(
    on_close: EventHandler<MouseEvent>,
    on_create: EventHandler<Account>,
) -> Element {
    let label = use_signal(|| "New Account".to_string());

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal",
                div { class: "modal-header",
                    h2 { "Create New Account" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "Account Label" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            value: "{label}",
                            placeholder: "Enter account label...",
                            onchange: move |evt| label.set(evt.value())
                        }
                    }

                    div { class: "info-box",
                        h4 { "🔐 Security Note" }
                        p { "A new keypair will be generated and stored securely. Make sure to backup your private key." }
                    }
                }

                div { class: "modal-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: on_close,
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // TODO: Generate actual keypair
                            let new_account = Account {
                                address: generate_mock_address(),
                                balance: 0,
                                label: label(),
                                created_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                            };
                            on_create.call(new_account);
                        },
                        "🚀 Create Account"
                    }
                }
            }
        }
    }
}

/// Import account modal component
#[component]
fn ImportAccountModal(
    on_close: EventHandler<MouseEvent>,
    on_import: EventHandler<Account>,
) -> Element {
    let private_key = use_signal(|| String::new());
    let label = use_signal(|| "Imported Account".to_string());

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal",
                div { class: "modal-header",
                    h2 { "Import Account" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "Private Key or Seed Phrase" }
                        textarea {
                            class: "form-control",
                            rows: 3,
                            placeholder: "Enter private key or seed phrase...",
                            value: "{private_key}",
                            onchange: move |evt| private_key.set(evt.value())
                        }
                    }

                    div { class: "form-group",
                        label { "Account Label" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            value: "{label}",
                            placeholder: "Enter account label...",
                            onchange: move |evt| label.set(evt.value())
                        }
                    }
                }

                div { class: "modal-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: on_close,
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // TODO: Validate and import key
                            if !private_key().is_empty() {
                                let imported_account = Account {
                                    address: generate_mock_address(),
                                    balance: 0,
                                    label: label(),
                                    created_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                                };
                                on_import.call(imported_account);
                            }
                        },
                        "📥 Import Account"
                    }
                }
            }
        }
    }
}

/// Receive modal component
#[component]
fn ReceiveModal(address: String, on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "modal-overlay",
            div { class: "modal",
                div { class: "modal-header",
                    h2 { "Receive SOL" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "qr-code-placeholder",
                        div { class: "qr-icon", "📱" }
                        p { "QR Code would appear here" }
                    }

                    div { class: "address-display-large",
                        label { "Your Address:" }
                        div { class: "address-value",
                            "{address}"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                // TODO: Copy address
                                tracing::info!("Copying address to clipboard");
                            },
                            "📋 Copy"
                        }
                    }

                    div { class: "info-box",
                        h4 { "ℹ️ Information" }
                        p { "Share this address to receive SOL. The address is your public key and can be safely shared." }
                    }
                }

                div { class: "modal-footer",
                    button {
                        class: "btn btn-primary",
                        onclick: on_close,
                        "Done"
                    }
                }
            }
        }
    }
}

/// Send modal component
#[component]
fn SendModal(account: Account, on_close: EventHandler<MouseEvent>) -> Element {
    let recipient_address = use_signal(|| String::new());
    let amount = use_signal(|| "0.0".to_string());
    let memo = use_signal(|| String::new());

    rsx! {
        div { class: "modal-overlay",
            div { class: "modal",
                div { class: "modal-header",
                    h2 { "Send SOL" }
                    button {
                        class: "modal-close",
                        onclick: on_close,
                        "✕"
                    }
                }

                div { class: "modal-body",
                    div { class: "form-group",
                        label { "Available Balance" }
                        div { class: "balance-display",
                            span { class: "balance-value", "{account.balance_sol():.4}" }
                            span { class: "balance-currency", "SOL" }
                        }
                    }

                    div { class: "form-group",
                        label { "Recipient Address" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            placeholder: "Enter recipient address...",
                            value: "{recipient_address}",
                            onchange: move |evt| recipient_address.set(evt.value())
                        }
                    }

                    div { class: "form-group",
                        label { "Amount (SOL)" }
                        input {
                            r#type: "number",
                            class: "form-control",
                            placeholder: "0.0",
                            value: "{amount}",
                            step: "0.000001",
                            onchange: move |evt| amount.set(evt.value())
                        }
                    }

                    div { class: "form-group",
                        label { "Memo (Optional)" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            placeholder: "Enter memo...",
                            value: "{memo}",
                            onchange: move |evt| memo.set(evt.value())
                        }
                    }
                }

                div { class: "modal-footer",
                    button {
                        class: "btn btn-secondary",
                        onclick: on_close,
                        "Cancel"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // TODO: Validate and send transaction
                            if !recipient_address().is_empty() && !amount().is_empty() {
                                tracing::info!("Sending {} SOL to {}", amount(), recipient_address());
                                on_close.call(MouseEvent::default());
                            }
                        },
                        "📤 Send"
                    }
                }
            }
        }
    }
}

/// Utility function to format address for display
fn format_address(address: &str) -> String {
    if address.len() > 20 {
        format!("{}...{}", &address[..8], &address[address.len() - 8..])
    } else {
        address.to_string()
    }
}

/// Generate mock address for demonstration
fn generate_mock_address() -> String {
    // TODO: Generate actual Solana address
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);

    let hash = hasher.finish();
    format!("{:x}", hash)[..44].to_string()
}
