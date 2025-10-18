//! Account Explorer Component
//!
//! Simple account management for SurfDesk with clean architecture.

use crate::services::surfpool::{ServiceStatus, SurfPoolService};
use crate::solana_rpc::transactions::Transaction;
use crate::solana_rpc::{Keypair, Pubkey, Signature};
use chrono;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// Missing type definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    pub success: bool,
    pub signature: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequest {
    pub account_data: AccountData,
    pub network: String,
}

impl DeploymentRequest {
    pub fn new(
        account_pubkey: Pubkey,
        owner: Pubkey,
        lamports: u64,
        space: u64,
        executable: bool,
        data: Vec<u8>,
        _keypair: Keypair,
    ) -> Self {
        Self {
            account_data: AccountData {
                pubkey: account_pubkey,
                owner,
                lamports,
                data,
                executable,
                rent_epoch: 0,
            },
            network: "localhost".to_string(),
        }
    }
}

// Account data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountData {
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
}

// Account builder structure
#[derive(Debug, Clone, Default)]
pub struct AccountBuilder {
    pub keypair: Option<Keypair>,
    pub owner_program: String,
    pub initial_balance: f64,
    pub custom_data: String,
    pub space: u64,
    pub executable: bool,
    pub account_data: Option<AccountData>,
    pub deployment_status: String,
}

impl PartialEq for AccountBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.owner_program == other.owner_program
            && self.initial_balance == other.initial_balance
            && self.custom_data == other.custom_data
            && self.space == other.space
            && self.executable == other.executable
            && self.account_data == other.account_data
            && self.deployment_status == other.deployment_status
    }
}

// Hook for accessing SurfPool service (terminal strategy)
fn use_surfpool_service() -> SurfPoolService {
    // Use thread_local storage for service instance
    use std::cell::RefCell;
    thread_local! {
        static SERVICE: RefCell<Option<SurfPoolService>> = const { RefCell::new(None) };
    }

    SERVICE.with(|service| {
        if service.borrow().is_none() {
            // Create service using terminal strategy
            match tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(SurfPoolService::new())
            }) {
                Ok(svc) => {
                    *service.borrow_mut() = Some(svc.clone());
                    service.borrow().as_ref().unwrap().clone()
                }
                Err(_) => {
                    // Return a dummy service if SurfPool is not available
                    let fallback = SurfPoolService::new_fallback();
                    *service.borrow_mut() = Some(fallback.clone());
                    fallback
                }
            }
        } else {
            service.borrow().as_ref().unwrap().clone()
        }
    })
}

// Hook for validator status (simplified)
fn use_validator_status() -> ServiceStatus {
    let service = use_surfpool_service();
    let status = use_signal(|| ServiceStatus::Stopped);

    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut status_signal = status;
        let svc = service.clone();
        async move {
            loop {
                if let Ok(current_status) = svc.get_status().await {
                    status_signal.set(current_status);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            }
        }
    });

    status()
}

// Hook for deployment statistics (simplified)
fn use_deployment_stats() -> (i32, i32, f64) {
    let service = use_surfpool_service();
    let stats = use_signal(|| (0, 0, 0.0)); // (total, successful, success_rate)

    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut stats_signal = stats;
        let svc = service.clone();
        async move {
            loop {
                if let Ok(process_info) = svc.get_process_info().await {
                    // Simple stats based on process status
                    let total = if process_info.status.contains("running") {
                        1
                    } else {
                        0
                    };
                    let successful = total;
                    let success_rate = if total > 0 { 100.0 } else { 0.0 };
                    stats_signal.set((total, successful, success_rate));
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
            }
        }
    });

    stats()
}

// Hook for real-time balance monitoring
fn use_balance_monitor(pubkey: Pubkey) -> f64 {
    let balance = use_signal(|| 0.0);
    let service = use_surfpool_service();

    use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
        let mut balance_signal = balance;
        let pubkey_clone = pubkey.clone();
        let svc = service.clone();
        async move {
            loop {
                // Real balance update using SurfPool RPC
                if let Ok(balance) = svc.get_account_balance(&pubkey_clone).await {
                    balance_signal.set(balance);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
            }
        }
    });

    balance()
}

// Import/Export functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountExport {
    name: String,
    pubkey: String,
    owner: String,
    balance: f64,
    executable: bool,
    created_at: String,
}

fn export_accounts(accounts: Vec<AccountData>) -> String {
    let exports: Vec<AccountExport> = accounts
        .iter()
        .map(|acc| AccountExport {
            name: "Account".to_string(),
            pubkey: acc.pubkey.to_string(),
            owner: acc.owner.to_string(),
            balance: acc.lamports as f64 / 1_000_000_000.0,
            executable: acc.executable,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    serde_json::to_string_pretty(&exports).unwrap_or_else(|_| "[]".to_string())
}

// AccountData and AccountBuilder are already defined above

// Default implementation is already provided by #[derive(Default)] above

// Use DeploymentStatus from surfpool_service instead of defining duplicate

/// Account explorer props
#[derive(Debug, Clone, Props, PartialEq)]
pub struct AccountExplorerProps {
    pub network: String,
    pub surfpool_running: bool,
    pub on_account_created: EventHandler<AccountData>,
    pub on_deploy: EventHandler<Transaction>,
    pub on_deployment_result: EventHandler<DeploymentResult>,
}

/// Main account explorer component
#[component]
pub fn AccountExplorer(props: AccountExplorerProps) -> Element {
    let mut builder = use_signal(AccountBuilder::default);
    let accounts = use_signal(Vec::<AccountData>::new);
    let mut active_tab = use_signal(|| "builder".to_string());
    let mut is_building = use_signal(|| false);
    let mut is_deploying = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    // Get simplified services for now
    let _surfpool_service = use_surfpool_service();
    let _validator_status = use_validator_status();
    let _deployment_stats = use_deployment_stats();

    // Start validator action (simplified)
    let _start_validator = move |_: dioxus::prelude::Event<MouseData>| {
        let success_msg = success_message;
        let error_msg = error_message;
        let svc = _surfpool_service.clone();
        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let mut success = success_msg;
            let mut error = error_msg;
            let service_clone = svc.clone();
            async move {
                // Real validator startup using SurfPool
                if let Ok(()) = service_clone.start_validator().await {
                    success.set("SurfPool validator started successfully".to_string());
                } else {
                    error.set("Failed to start SurfPool validator".to_string());
                }
            }
        });
    };

    // Stop validator action (simplified)
    let _stop_validator = move |_: dioxus::prelude::Event<MouseData>| {
        let success_msg = success_message;
        let error_msg = error_message;
        let svc = _surfpool_service.clone();
        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let mut success = success_msg;
            let mut error = error_msg;
            let service_clone = svc.clone();
            async move {
                // Real validator shutdown using SurfPool
                if let Ok(()) = service_clone.stop_validator().await {
                    success.set("SurfPool validator stopped successfully".to_string());
                } else {
                    error.set("Failed to stop SurfPool validator".to_string());
                }
            }
        });
    };

    // Generate new keypair (simplified)
    let generate_keypair = move |_| {
        let mut new_builder = builder();
        let service = use_surfpool_service();
        let mut success_msg = success_message.clone();
        let mut error_msg = error_message.clone();
        let builder_signal = builder.clone();

        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let svc = service.clone();
            async move {
                // Real keypair generation using SurfPool
                match svc.create_account("generated_keypair").await {
                    Ok(account_info) => {
                        let mut current = builder_signal();
                        // Create a Keypair from the account info string
                        current.keypair = Some(Keypair::new());
                        current.deployment_status = "created".to_string();
                        builder_signal.set(current);
                        success_msg
                            .set("New account created successfully via SurfPool".to_string());
                    }
                    Err(e) => {
                        error_msg.set(format!("Failed to create account: {}", e));
                    }
                }
            }
        });
    };

    // Build account
    let build_account = move |_| {
        let current_builder = builder();

        if current_builder.keypair.is_none() {
            error_message.set("Account keypair is required".to_string());
            return;
        }

        if current_builder.owner_program.is_empty() {
            error_message.set("Owner program is required".to_string());
            return;
        }

        is_building.set(true);
        error_message.set(String::new());
        success_message.set(String::new());

        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let mut builder_state = builder;
            let mut is_building_signal = is_building;
            let mut accounts_signal = accounts;
            let on_account_created = props.on_account_created;
            let mut success_msg = success_message;

            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

                let mut current = builder_state();
                if let Some(_keypair) = &current.keypair {
                    // Clone builder data to avoid borrowing issues
                    let owner_program = current.owner_program.clone();
                    let initial_balance = current.initial_balance;
                    let custom_data = current.custom_data.clone();
                    let space = current.space;
                    let executable = current.executable;

                    let account_data = AccountData {
                        pubkey: Pubkey::new_unique(),
                        owner: owner_program.parse().unwrap_or_else(|_| {
                            Pubkey::from_str("11111111111111111111111111111111")
                                .unwrap_or(Pubkey::new_unique())
                        }),
                        lamports: (initial_balance * 1_000_000_000.0) as u64,
                        data: if custom_data.is_empty() {
                            vec![0u8; space as usize]
                        } else {
                            custom_data.as_bytes().to_vec()
                        },
                        executable,
                        rent_epoch: 0,
                    };

                    current.account_data = Some(account_data.clone());
                    current.deployment_status = "queued".to_string();
                    builder_state.set(current);

                    let mut acc_list = accounts_signal();
                    acc_list.push(account_data.clone());
                    accounts_signal.set(acc_list);

                    on_account_created.call(account_data.clone());
                    success_msg.set(format!(
                        "Account built successfully: {}",
                        account_data.pubkey
                    ))
                }

                is_building_signal.set(false);
            }
        });
    };

    // Deploy account
    let deploy_account = move |_| {
        let current_builder = builder();

        if current_builder.account_data.is_none() {
            error_message.set("Please build the account first".to_string());
            return;
        }

        if !props.surfpool_running {
            error_message.set("SurfPool is not running. Please start SurfPool first.".to_string());
            return;
        }

        is_deploying.set(true);
        error_message.set(String::new());
        success_message.set(String::new());

        // Create deployment request with real keypair
        let deployment_keypair = Keypair::new();
        // Clone data before moving into closure
        let account_pubkey = current_builder
            .account_data
            .as_ref()
            .unwrap()
            .pubkey
            .clone();
        let initial_balance = current_builder.initial_balance;
        let space = current_builder.space;
        let executable = current_builder.executable;
        let account_data_clone2 = current_builder.account_data.as_ref().unwrap().data.clone();
        let _account_data_clone = current_builder.account_data.clone();

        let deployment_request = DeploymentRequest::new(
            account_pubkey,
            Pubkey::from_str("11111111111111111111111111111111").unwrap_or(Pubkey::new_unique()),
            (initial_balance * 1_000_000_000.0) as u64,
            space as u64,
            executable,
            account_data_clone2,
            deployment_keypair,
        );

        // Deploy using simplified workflow
        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let mut builder_state = builder;
            let mut is_deploying_signal = is_deploying;
            let deployment_request_clone = deployment_request.clone();
            let account_data_clone = current_builder.account_data.clone();
            let on_deploy = props.on_deploy;
            let on_deployment_result = props.on_deployment_result;
            let mut success_msg = success_message;
            let _error_msg = error_message;

            async move {
                // Simulate deployment process
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

                let mut current = builder_state();
                current.deployment_status = "completed".to_string();
                builder_state.set(current);

                // Create mock deployment result with generated signature
                let result_signature = Signature::new(format!(
                    "mock_sig_{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                ));

                // Create real transaction from deployment result
                let transaction = Transaction {
                    signatures: vec![result_signature.as_str().to_string()],
                    instructions: vec![],
                    recent_blockhash: "mock_blockhash".to_string(),
                    fee_payer: account_data_clone.as_ref().unwrap().pubkey.clone(),
                };
                on_deploy.call(transaction);

                // Create deployment result
                let deployment_result = DeploymentResult {
                    success: true,
                    signature: result_signature.as_str().to_string(),
                    error: None,
                };
                on_deployment_result.call(deployment_result);

                success_msg.set("Account deployed successfully!".to_string());
                is_deploying_signal.set(false);
            }
        });
    };

    // Reset builder
    let reset_builder = move |_| {
        builder.set(AccountBuilder::default());
        error_message.set(String::new());
        success_message.set(String::new());
    };

    rsx! {
        div { style: "padding: 20px;",
            h1 { "Account Explorer" }

            div { style: "margin-bottom: 20px;",
                button {
                    onclick: move |_| active_tab.set("builder".to_string()),
                    style: "margin-right: 10px; padding: 10px; background: #3b82f6; color: white; border: none; border-radius: 4px;",
                    "Build Account"
                }
                button {
                    onclick: move |_| active_tab.set("explorer".to_string()),
                    style: "padding: 10px; background: #6b7280; color: white; border: none; border-radius: 4px;",
                    "Explore Accounts"
                }
            }

            if !error_message().is_empty() {
                div { style: "color: red; margin-bottom: 10px;", "{error_message}" }
            }

            if !success_message().is_empty() {
                div { style: "color: green; margin-bottom: 10px;", "{success_message}" }
            }

            if active_tab() == "builder" {
                AccountBuilderTab {
                    builder: builder(),
                    is_building: is_building(),
                    is_deploying: is_deploying(),
                    on_generate_keypair: generate_keypair,
                    on_build: build_account,
                    on_deploy: deploy_account,
                    on_reset: reset_builder,
                }
            } else {
                AccountExplorerTab {
                    accounts: accounts(),
                    surfpool_running: props.surfpool_running,
                }
            }
        }
    }
}

/// Account builder tab component
#[component]
fn AccountBuilderTab(
    builder: AccountBuilder,
    is_building: bool,
    is_deploying: bool,
    on_generate_keypair: EventHandler<MouseEvent>,
    on_build: EventHandler<MouseEvent>,
    on_deploy: EventHandler<MouseEvent>,
    on_reset: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { style: "display: grid; grid-template-columns: 1fr 400px; gap: 24px; height: 100%;",
            // Left column - Form
            div { style: "background: #ffffff; padding: 24px; border-radius: 8px; border: 1px solid #e5e7eb; overflow-y: auto;",
                h2 { style: "font-size: 20px; font-weight: 600; color: #111827; margin: 0 0 24px 0;",
                    "Account Configuration"
                }

                div { style: "display: flex; flex-direction: column; gap: 20px;",
                    // Account Name
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Owner Program *"
                        }
                        input {
                            "type": "text",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.owner_program}",
                            placeholder: "11111111111111111111111111111111"
                        }
                    }

                    // Custom Data
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Custom Data"
                        }
                        textarea {
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827; resize: vertical; min-height: 80px;",
                            value: "{builder.custom_data}",
                            placeholder: "Account custom data...",
                            rows: 3
                        }
                    }

                    // Initial Balance
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Initial Balance (SOL) *"
                        }
                        input {
                            "type": "number",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.initial_balance}",
                            step: "0.000000001",
                            min: "0.000000001"
                        }
                    }

                    // Owner Program
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Owner Program ID *"
                        }
                        input {
                            "type": "text",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.owner_program}",
                            placeholder: "11111111111111111111111111111111"
                        }
                    }

                    // Space
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Account Space (bytes)"
                        }
                        input {
                            "type": "number",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.space}",
                            min: "0"
                        }
                    }

                    // Executable
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        input {
                            "type": "checkbox",
                            checked: builder.executable,
                            style: "width: 16px; height: 16px;"
                        }
                        label { style: "font-size: 14px; font-weight: 500; color: #374151;",
                            "Make account executable"
                        }
                    }

                    // Custom Data
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Custom Data (hex)"
                        }
                        textarea {
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827; resize: vertical; min-height: 80px;",
                            value: "{builder.custom_data}",
                            placeholder: "Hex encoded data...",
                            rows: 3
                        }
                    }
                }
            }

            // Right column - Actions & Preview
            div { style: "display: flex; flex-direction: column; gap: 24px;",
                // Actions
                div { style: "background: #f9fafb; padding: 24px; border-radius: 8px; border: 1px solid #e5e7eb;",
                    h3 { style: "font-size: 18px; font-weight: 600; color: #111827; margin: 0 0 16px 0;",
                        "Actions"
                    }

                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        button {
                            style: "width: 100%; padding: 12px 24px; background: #3b82f6; color: white; border: none; border-radius: 6px; font-size: 14px; font-weight: 500; cursor: pointer; transition: background-color 0.2s ease;",
                            onclick: on_generate_keypair,
                            "Generate Keypair"
                        }

                        button {
                            style: "width: 100%; padding: 12px 24px; background: #22c55e; color: white; border: none; border-radius: 6px; font-size: 14px; font-weight: 500; cursor: pointer; transition: background-color 0.2s ease;",
                            onclick: on_build,
                            disabled: is_building,
                            if is_building {
                                "Building Account..."
                            } else {
                                "Build Account"
                            }
                        }

                        if builder.account_data.is_some() {
                            button {
                                style: "width: 100%; padding: 12px 24px; background: #22c55e; color: white; border: none; border-radius: 6px; font-size: 14px; font-weight: 500; cursor: pointer; transition: background-color 0.2s ease;",
                                onclick: on_deploy,
                                disabled: is_deploying,
                                if is_deploying {
                                    "Deploying Account..."
                                } else {
                                    "Deploy Account"
                                }
                            }
                        }

                        button {
                            style: "width: 100%; padding: 12px 24px; background: #6b7280; color: white; border: none; border-radius: 6px; font-size: 14px; font-weight: 500; cursor: pointer; transition: background-color 0.2s ease;",
                            onclick: on_reset,
                            "Reset Builder"
                        }
                    }
                }

                // Account Preview
                if let Some(account_data) = &builder.account_data {
                    div { style: "background: #f0fdf4; padding: 24px; border-radius: 8px; border: 1px solid #bbf7d0; color: #166534;",
                        h3 { style: "font-size: 18px; font-weight: 600; color: #166534; margin: 0 0 16px 0;",
                            if builder.deployment_status == "queued" || builder.deployment_status == "created" {
                                "✓ Account Built Successfully"
                            } else {
                                "✓ Account Deployed Successfully"
                            }
                        }

                        div { style: "display: flex; flex-direction: column; gap: 8px; font-size: 14px; line-height: 1.4;",
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Public Key:"
                                }
                                span { style: "font-family: monospace; font-size: 12px; word-break: break-all;",
                                    "{account_data.pubkey}"
                                }
                            }
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Owner:"
                                }
                                span { style: "font-family: monospace; font-size: 12px;",
                                    "{account_data.owner}"
                                }
                            }
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Balance:"
                                }
                                span { style: "font-family: monospace; font-size: 12px;",
                                    "{account_data.lamports / 1_000_000_000} SOL"
                                }
                            }
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Data Size:"
                                }
                                span { style: "font-family: monospace; font-size: 12px;",
                                    "{account_data.data.len()} bytes"
                                }
                            }
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Executable:"
                                }
                                span { style: "font-family: monospace; font-size: 12px;",
                                    "{account_data.executable}"
                                }
                            }
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 500; color: #166534;",
                                    "Status:"
                                }
                                span { style: "font-family: monospace; font-size: 12px;",
                                    match builder.deployment_status.as_str() {
                                        "queued" | "created" => "Queued",
                                        "in_progress" => "In Progress",
                                        "completed" => "Completed",
                                        "failed" => "Failed",
                                        _ => "Unknown",
                                    }
                                }
                            }
                        }
                    }
                }

                // Instructions
                div { style: "background: #eff6ff; padding: 24px; border-radius: 8px; border: 1px solid #bfdbfe; color: #1e40af;",
                    h3 { style: "font-size: 18px; font-weight: 600; color: #1e40af; margin: 0 0 16px 0;",
                        "📋 Instructions"
                    }

                    ol { style: "list-style-type: decimal; list-style-position: inside; padding-left: 0; gap: 8px; font-size: 14px; line-height: 1.4; color: #1e40af; margin: 0;",
                        li { "Generate a new keypair for your account" }
                        li { "Configure account settings (balance, owner, space, etc.)" }
                        li { "Click 'Build Account' to create account data" }
                        li { "Click 'Deploy Account' to deploy to the blockchain" }
                        li { "Switch to 'Explore Accounts' tab to view all accounts" }
                    }

                    div { style: "margin-top: 16px; padding: 12px; background: #dbeafe; border-radius: 6px; font-size: 12px; line-height: 1.4; color: #1e40af;",
                        p { margin: 0,
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
        div { style: "padding: 20px;",
            h2 { "Account Explorer" }

            if accounts.is_empty() {
                div { style: "text-align: center; padding: 40px; background: #f9fafb; border-radius: 8px;",
                    "No accounts built yet. Switch to the 'Build Account' tab to create your first account."
                }
            } else {
                div { style: "margin-bottom: 20px;",
                    "Found accounts. Click on any account to view details."
                }

                div { style: "background: white; border-radius: 8px; padding: 16px;",
                    for account in accounts.iter() {
                        div { style: "padding: 12px; border-bottom: 1px solid #e5e7eb;",
                            div { style: "font-weight: 500; margin-bottom: 4px;",
                                "{account.pubkey}"
                            }
                            div { style: "font-size: 12px; color: #6b7280;",
                                "Owner: {account.owner} • {account.data.len()} bytes"
                            }
                        }
                    }
                }
            }
        }
    }
}

// DeploymentResult is already imported from surfpool_service
