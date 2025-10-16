//! # Account Explorer & Builder Component
//!
//! Professional account management system for SurfDesk desktop application.
//! Features account building, deployment, and SurfPool integration.

use dioxus::prelude::*;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

/// Account data structure
#[derive(Debug, Clone, PartialEq)]
pub struct AccountData {
    /// Account public key
    pub pubkey: Pubkey,
    /// Account owner
    pub owner: Pubkey,
    /// Account lamports
    pub lamports: u64,
    /// Account data
    pub data: Vec<u8>,
    /// Account executable status
    pub executable: bool,
    /// Account rent epoch
    pub rent_epoch: u64,
}

/// Account builder state
#[derive(Debug, Clone)]
pub struct AccountBuilder {
    /// Account name
    pub name: String,
    /// Account description
    pub description: String,
    /// Initial balance in SOL
    pub initial_balance: f64,
    /// Account owner program ID
    pub owner_program: String,
    /// Account space in bytes
    pub space: u64,
    /// Whether account is executable
    pub executable: bool,
    /// Custom account data (hex string)
    pub custom_data: String,
    /// Generated keypair
    pub keypair: Option<Keypair>,
    /// Built account data
    pub account_data: Option<AccountData>,
}

impl Default for AccountBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            initial_balance: 0.1,
            owner_program: "11111111111111111111111111111111".to_string(), // System Program
            space: 0,
            executable: false,
            custom_data: String::new(),
            keypair: None,
            account_data: None,
        }
    }
}

/// Account explorer props
#[derive(Debug, Clone, Props)]
pub struct AccountExplorerProps {
    /// Current network
    pub network: String,
    /// Whether SurfPool is running
    pub surfpool_running: bool,
    /// on account created handler
    pub on_account_created: EventHandler<AccountData>,
    /// on deploy handler
    pub on_deploy: EventHandler<Transaction>,
}

/// Account explorer component
#[component]
pub fn AccountExplorer(props: AccountExplorerProps) -> Element {
    let mut builder = use_signal(AccountBuilder::default);
    let mut accounts = use_signal(Vec::<AccountData>::new);
    let mut active_tab = use_signal(|| "builder".to_string());
    let mut is_building = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    // Generate new keypair
    let generate_keypair = move |_| {
        let mut new_builder = builder();
        new_builder.keypair = Some(Keypair::new());
        builder.set(new_builder);
    };

    // Build account
    let build_account = move |_| {
        let mut current_builder = builder();

        // Validation
        if current_builder.name.trim().is_empty() {
            error_message.set("Account name is required".to_string());
            return;
        }

        if current_builder.keypair.is_none() {
            error_message.set("Please generate a keypair first".to_string());
            return;
        }

        if current_builder.initial_balance < 0.000000001 {
            error_message.set("Initial balance must be at least 0.000000001 SOL".to_string());
            return;
        }

        is_building.set(true);
        error_message.set(String::new());

        // Simulate account building (in real app, this would interact with blockchain)
        use_coroutine(|_| {
            let builder_state = builder.clone();
            let is_building_signal = is_building.clone();
            let accounts_signal = accounts.clone();
            let on_account_created = props.on_account_created.clone();

            async move {
                // Simulate building delay
                tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

                let mut current = builder_state();
                if let Some(keypair) = &current.keypair {
                    let account_data = AccountData {
                        pubkey: keypair.pubkey(),
                        owner: Pubkey::from_str(&current.owner_program).unwrap_or_default(),
                        lamports: (current.initial_balance * 1_000_000_000.0) as u64,
                        data: if current.custom_data.is_empty() {
                            vec![0u8; current.space as usize]
                        } else {
                            hex::decode(&current.custom_data).unwrap_or_default()
                        },
                        executable: current.executable,
                        rent_epoch: 0,
                    };

                    current.account_data = Some(account_data.clone());
                    builder_state.set(current);

                    // Add to accounts list
                    let mut acc_list = accounts_signal();
                    acc_list.push(account_data.clone());
                    accounts_signal.set(acc_list);

                    // Call handler
                    on_account_created.call(account_data);
                }

                is_building_signal.set(false);
            }
        });
    };

    // Deploy account
    let deploy_account = move |_| {
        let current_builder = builder();

        if let Some(_account_data) = &current_builder.account_data {
            if let Some(keypair) = &current_builder.keypair {
                // Create deployment transaction
                let mut transaction = Transaction::new_with_payer(
                    &[
                        // In real app, this would be actual create account instruction
                        Instruction::new_with_bytes(
                            Pubkey::from_str(&current_builder.owner_program).unwrap_or_default(),
                            &[],
                        ),
                    ],
                    Some(&keypair.pubkey()),
                );

                // Sign transaction
                transaction.sign(&[keypair], props.network.parse().unwrap());

                // Call deploy handler
                props.on_deploy.call(transaction);
            }
        } else {
            error_message.set("Please build the account first".to_string());
        }
    };

    // Reset builder
    let reset_builder = move |_| {
        builder.set(AccountBuilder::default());
        error_message.set(String::new());
    };

    let current_builder = builder();

    rsx! {
        div { class: "account-explorer p-6 bg-white rounded-lg shadow-lg",

            // Header
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-2xl font-bold text-gray-900", "Account Builder & Explorer" }

                div { class: "flex items-center gap-2",
                    // Network indicator
                    span { class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium",
                        "{props.network}"
                    }

                    // SurfPool status
                    span { class: "px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium",
                        if props.surfpool_running { "🟢 SurfPool Active" } else { "🔴 SurfPool Inactive" }
                    }
                }
            }

            // Error message
            if !error_message().is_empty() {
                div { class: "mb-4 p-4 bg-red-50 border border-red-200 rounded-md",
                    div { class: "flex",
                        div { class: "flex-shrink-0",
                            svg { class: "h-5 w-5 text-red-400",
                                "viewBox": "0 0 20 20",
                                "fill": "currentColor",
                                path { "d": "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" }
                            }
                        }
                        div { class: "ml-3",
                            h3 { class: "text-sm font-medium text-red-800", "Error" }
                            div { class: "mt-2 text-sm text-red-700", "{error_message()}" }
                        }
                    }
                }
            }

            // Tabs
            div { class: "border-b border-gray-200 mb-6",
                nav { class: "-mb-px flex space-x-8",
                    button {
                        class: "py-2 px-1 border-b-2 font-medium text-sm {
                            if active_tab() == "builder" {
                                "border-indigo-500 text-indigo-600"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                            }
                        }",
                        onclick: move |_| active_tab.set("builder".to_string()),
                        "Build Account"
                    }

                    button {
                        class: "py-2 px-1 border-b-2 font-medium text-sm {
                            if active_tab() == "explorer" {
                                "border-indigo-500 text-indigo-600"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                            }
                        }",
                        onclick: move |_| active_tab.set("explorer".to_string()),
                        "Explore Accounts"
                    }
                }
            }

            // Tab content
            match active_tab().as_str() {
                "builder" => rsx! {
                    AccountBuilderTab {
                        builder: current_builder.clone(),
                        is_building: is_building(),
                        on_generate_keypair: generate_keypair,
                        on_build: build_account,
                        on_deploy: deploy_account,
                        on_reset: reset_builder,
                    }
                },
                "explorer" => rsx! {
                    AccountExplorerTab {
                        accounts: accounts(),
                        surfpool_running: props.surfpool_running,
                    }
                },
                _ => rsx! { div { "Unknown tab" } }
            }
        }
    }
}

/// Account builder tab component
#[component]
fn AccountBuilderTab(
    builder: AccountBuilder,
    is_building: bool,
    on_generate_keypair: EventHandler<MouseEvent>,
    on_build: EventHandler<MouseEvent>,
    on_deploy: EventHandler<MouseEvent>,
    on_reset: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

            // Left column - Form
            div { class: "space-y-6",

                // Basic Information
                div { class: "bg-gray-50 p-6 rounded-lg",
                    h3 { class: "text-lg font-medium text-gray-900 mb-4", "Basic Information" }

                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Account Name *" }
                            input {
                                r#type: "text",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                value: "{builder.name}",
                                placeholder: "My Account",
                                oninput: move |evt| {
                                    // Update builder name (would need to be handled properly in real implementation)
                                }
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Description" }
                            textarea {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                rows: 3,
                                placeholder: "Account description...",
                                value: "{builder.description}",
                                oninput: move |evt| {
                                    // Update builder description
                                }
                            }
                        }
                    }
                }

                // Keypair Management
                div { class: "bg-gray-50 p-6 rounded-lg",
                    h3 { class: "text-lg font-medium text-gray-900 mb-4", "Keypair Management" }

                    if let Some(keypair) = &builder.keypair {
                        div { class: "space-y-3",
                            div { class: "p-3 bg-green-50 border border-green-200 rounded-md",
                                div { class: "text-sm font-medium text-green-800", "✓ Keypair Generated" }
                                div { class: "mt-1 text-xs text-green-700 font-mono break-all",
                                    "Pubkey: {keypair.pubkey()}"
                                }
                            }

                            button {
                                class: "w-full px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 disabled:opacity-50",
                                onclick: on_generate_keypair,
                                "Generate New Keypair"
                            }
                        }
                    } else {
                        div { class: "text-center py-6",
                            p { class: "text-gray-500 mb-4", "No keypair generated yet" }
                            button {
                                class: "px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700",
                                onclick: on_generate_keypair,
                                "Generate Keypair"
                            }
                        }
                    }
                }

                // Account Configuration
                div { class: "bg-gray-50 p-6 rounded-lg",
                    h3 { class: "text-lg font-medium text-gray-900 mb-4", "Account Configuration" }

                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Initial Balance (SOL)" }
                            input {
                                r#type: "number",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                value: "{builder.initial_balance}",
                                step: "0.000000001",
                                min: "0.000000001",
                                oninput: move |evt| {
                                    // Update initial balance
                                }
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Owner Program ID" }
                            input {
                                r#type: "text",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                value: "{builder.owner_program}",
                                placeholder: "11111111111111111111111111111111",
                                oninput: move |evt| {
                                    // Update owner program
                                }
                            }
                        }

                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-1", "Account Space (bytes)" }
                            input {
                                r#type: "number",
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                value: "{builder.space}",
                                min: "0",
                                oninput: move |evt| {
                                    // Update space
                                }
                            }
                        }

                        div { class: "flex items-center",
                            input {
                                r#type: "checkbox",
                                class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                                checked: builder.executable,
                                onchange: move |evt| {
                                    // Update executable status
                                }
                            }
                            label { class: "ml-2 block text-sm text-gray-900", "Executable Account" }
                        }
                    }
                }

                // Custom Data
                div { class: "bg-gray-50 p-6 rounded-lg",
                    h3 { class: "text-lg font-medium text-gray-900 mb-4", "Custom Account Data" }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Data (hex string)" }
                        textarea {
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm",
                            rows: 4,
                            placeholder: "48656c6c6f20576f726c64...", // "Hello World" in hex
                            value: "{builder.custom_data}",
                            oninput: move |evt| {
                                // Update custom data
                            }
                        }
                        p { class: "mt-1 text-xs text-gray-500", "Enter hex-encoded account data or leave empty for zero-initialized data" }
                    }
                }
            }

            // Right column - Actions & Preview
            div { class: "space-y-6",

                // Action Buttons
                div { class: "bg-gray-50 p-6 rounded-lg",
                    h3 { class: "text-lg font-medium text-gray-900 mb-4", "Actions" }

                    div { class: "space-y-3",
                        button {
                            class: "w-full px-4 py-3 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed font-medium",
                            onclick: on_build,
                            disabled: is_building || builder.keypair.is_none(),
                            if is_building {
                                "Building Account..."
                            } else {
                                "Build Account"
                            }
                        }

                        if builder.account_data.is_some() {
                            button {
                                class: "w-full px-4 py-3 bg-green-600 text-white rounded-md hover:bg-green-700 font-medium",
                                onclick: on_deploy,
                                "Deploy Account"
                            }
                        }

                        button {
                            class: "w-full px-4 py-3 bg-gray-600 text-white rounded-md hover:bg-gray-700 font-medium",
                            onclick: on_reset,
                            "Reset Builder"
                        }
                    }
                }

                // Account Preview
                if let Some(account_data) = &builder.account_data {
                    div { class: "bg-green-50 p-6 rounded-lg border border-green-200",
                        h3 { class: "text-lg font-medium text-green-900 mb-4", "✓ Account Built Successfully" }

                        div { class: "space-y-2 text-sm",
                            div { class: "flex justify-between",
                                span { class: "font-medium text-gray-700", "Public Key:" }
                                span { class: "font-mono text-gray-900 break-all", "{account_data.pubkey}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "font-medium text-gray-700", "Owner:" }
                                span { class: "font-mono text-gray-900", "{account_data.owner}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "font-medium text-gray-700", "Balance:" }
                                span { class: "font-mono text-gray-900", "{account_data.lamports / 1_000_000_000} SOL" }
                            }
                            div { class: "flex justify-between",
                                span { class: "font-medium text-gray-700", "Data Size:" }
                                span { class: "font-mono text-gray-900", "{} bytes", account_data.data.len() }
                            }
                            div { class: "flex justify-between",
                                span { class: "font-medium text-gray-700", "Executable:" }
                                span { class: "font-mono text-gray-900", "{account_data.executable}" }
                            }
                        }
                    }
                }

                // Instructions
                div { class: "bg-blue-50 p-6 rounded-lg border border-blue-200",
                    h3 { class: "text-lg font-medium text-blue-900 mb-4", "📋 Instructions" }

                    ol { class: "list-decimal list-inside space-y-2 text-sm text-blue-800",
                        li { "Generate a new keypair for your account" }
                        li { "Configure account settings (balance, owner, space, etc.)" }
                        li { "Click 'Build Account' to create account data" }
                        li { "Click 'Deploy Account' to deploy to the blockchain" }
                        li { "Switch to 'Explore Accounts' tab to view all accounts" }
                    }

                    div { class: "mt-4 p-3 bg-blue-100 rounded-md",
                        p { class: "text-xs text-blue-700",
                            "💡 Tip: Make sure SurfPool is running before deploying accounts to the local testnet."
                        }
                    }
                }
            }
        }
    }
}

/// Account explorer tab component
#[component]
fn AccountExplorerTab(accounts: Vec<AccountData>, surfpool_running: bool) -> Element {
    rsx! {
        div { class: "space-y-6",

            // Status Banner
            div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                div { class: "flex",
                    div { class: "flex-shrink-0",
                        svg { class: "h-5 w-5 text-blue-400",
                            "viewBox": "0 0 20 20",
                            "fill": "currentColor",
                            path { "d": "M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" }
                        }
                    }
                    div { class: "ml-3",
                        h3 { class: "text-sm font-medium text-blue-800", "Account Explorer" }
                        div { class: "mt-2 text-sm text-blue-700",
                            if accounts.is_empty() {
                                "No accounts built yet. Switch to the 'Build Account' tab to create your first account."
                            } else {
                                format!("Found {} account(s). Click on any account to view details.", accounts.len())
                            }
                        }
                    }
                }
            }

            // Accounts List
            if !accounts.is_empty() {
                div { class: "bg-white shadow overflow-hidden sm:rounded-md",
                    ul { class: "divide-y divide-gray-200",
                        for account in accounts.iter() {
                            li { class: "p-6 hover:bg-gray-50",
                                div { class: "flex items-center justify-between",
                                    div { class="flex-1 min-w-0",
                                        p { class: "text-sm font-medium text-indigo-600 truncate",
                                            "{account.pubkey}"
                                        }
                                        p { class: "text-sm text-gray-500",
                                            "Owner: {account.owner} • {account.lamports / 1_000_000_000} SOL"
                                        }
                                    }
                                    div { class="flex-shrink-0 flex items-center space-x-2",
                                        if account.executable {
                                            span { class: "px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded-full",
                                                "Executable"
                                            }
                                        }
                                        span { class: "px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded-full",
                                            "{account.data.len()} bytes"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Empty State
            if accounts.is_empty() {
                div { class: "text-center py-12",
                    svg { class: "mx-auto h-12 w-12 text-gray-400",
                        "fill": "none",
                        "viewBox": "0 0 24 24",
                        "stroke": "currentColor",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            "d": "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                        }
                    }
                    h3 { class: "mt-2 text-sm font-medium text-gray-900", "No accounts" }
                    p { class: "mt-1 text-sm text-gray-500", "Get started by building your first account." }
                    div { class: "mt-6",
                        button {
                            class: "inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700",
                            onclick: move |_| {
                                // Switch to builder tab (would need to be implemented)
                            },
                            "Build Account"
                        }
                    }
                }
            }
        }
    }
}
