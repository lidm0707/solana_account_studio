//! Account Explorer Component
//!
//! Simple account management for SurfDesk with clean architecture.

use crate::services::surfpool_service::{
    system_program, DeploymentRequest, DeploymentResult, DeploymentStatistics, Pubkey,
    SurfPoolService, SurfPoolStatus, Transaction,
};
use chrono;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Hook for accessing SurfPool service (simplified for compilation)
fn use_surfpool_service() -> Arc<SurfPoolService> {
    // Simplified mock service for development
    static mut SERVICE: Option<Arc<SurfPoolService>> = None;
    unsafe {
        if SERVICE.is_none() {
            SERVICE = Some(Arc::new(SurfPoolService::new_mock()));
        }
        SERVICE.clone().unwrap()
    }
}

// Hook for validator status (simplified)
fn use_validator_status() -> SurfPoolStatus {
    let _service = use_surfpool_service();
    let mut status = use_signal(|| SurfPoolStatus::Stopped);

    use_coroutine(move |_| {
        let status_signal = status.clone();
        async move {
            // Simulate status updates
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                // For now, just keep it as stopped
                status_signal.set(SurfPoolStatus::Stopped);
            }
        }
    });

    status()
}

// Hook for deployment statistics (simplified)
fn use_deployment_stats() -> DeploymentStatistics {
    let _service = use_surfpool_service();
    let mut stats = use_signal(|| DeploymentStatistics {
        total_deployments: 0,
        successful_deployments: 0,
        failed_deployments: 0,
        success_rate: 0.0,
    });

    use_coroutine(move |_| {
        let stats_signal = stats.clone();
        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                // Return mock statistics for now
                stats_signal.set(DeploymentStatistics {
                    total_deployments: 0,
                    successful_deployments: 0,
                    failed_deployments: 0,
                    success_rate: 0.0,
                });
            }
        }
    });

    stats()
}

// Hook for real-time balance monitoring
fn use_balance_monitor(pubkey: Pubkey) -> f64 {
    let mut balance = use_signal(|| 0.0);

    use_coroutine(move |_| {
        let balance_signal = balance.clone();
        let account_pubkey = pubkey;
        async move {
            loop {
                // Mock balance update - in real implementation, this would query RPC
                let mock_balance = (account_pubkey.to_bytes()[0] as f64) * 0.001;
                balance_signal.set(mock_balance);
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

/// Account data structure
#[derive(Debug, Clone, PartialEq)]
pub struct AccountData {
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Account builder state
#[derive(Debug, Clone, PartialEq)]
pub struct AccountBuilder {
    pub name: String,
    pub description: String,
    pub initial_balance: f64,
    pub owner_program: String,
    pub space: u64,
    pub executable: bool,
    pub custom_data: String,
    pub keypair: Option<String>,
    pub account_data: Option<AccountData>,
    pub deployment_status: crate::services::surfpool_service::DeploymentStatus,
}

impl Default for AccountBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            initial_balance: 0.1,
            owner_program: "11111111111111111111111111111111".to_string(),
            space: 0,
            executable: false,
            custom_data: String::new(),
            keypair: None,
            account_data: None,
            deployment_status: crate::services::surfpool_service::DeploymentStatus::Queued,
        }
    }
}

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
    let mut accounts = use_signal(Vec::<AccountData>::new);
    let mut active_tab = use_signal(|| "builder".to_string());
    let mut is_building = use_signal(|| false);
    let mut is_deploying = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    // Get simplified services for now
    let surfpool_service = use_surfpool_service();
    let validator_status = use_validator_status();
    let deployment_stats = use_deployment_stats();

    // Start validator action (simplified)
    let start_validator = move |_| {
        let success_msg = success_message.clone();
        use_coroutine(move |_| {
            let success = success_msg.clone();
            async move {
                // Simulate validator startup
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                success.set("SurfPool validator started successfully (mock)".to_string());
            }
        });
    };

    // Stop validator action (simplified)
    let stop_validator = move |_| {
        let success_msg = success_message.clone();
        use_coroutine(move |_| {
            let success = success_msg.clone();
            async move {
                // Simulate validator shutdown
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                success.set("SurfPool validator stopped successfully (mock)".to_string());
            }
        });
    };

    // Generate new keypair (simplified)
    let generate_keypair = move |_| {
        let mut new_builder = builder();
        // Simplified keypair generation for now
        new_builder.keypair = Some("generated_keypair_mock".to_string());
        new_builder.deployment_status = crate::services::surfpool_service::DeploymentStatus::Queued;
        builder.set(new_builder);
        success_message.set("New keypair generated successfully".to_string());
    };

    // Build account
    let build_account = move |_| {
        let mut current_builder = builder();

        if current_builder.name.is_empty() {
            error_message.set("Account name is required".to_string());
            return;
        }

        if current_builder.owner_program.is_empty() {
            error_message.set("Owner program is required".to_string());
            return;
        }

        is_building.set(true);
        error_message.set(String::new());
        success_message.set(String::new());

        use_coroutine(|_| {
            let builder_state = builder.clone();
            let is_building_signal = is_building.clone();
            let accounts_signal = accounts.clone();
            let on_account_created = props.on_account_created.clone();
            let success_msg = success_message.clone();

            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

                let mut current = builder_state();
                if let Some(_keypair) = &current.keypair {
                    let account_data = AccountData {
                        pubkey: Pubkey::new_unique(),
                        owner: current_builder
                            .owner_program
                            .parse()
                            .unwrap_or_else(|_| system_program::id()),
                        lamports: (current_builder.initial_balance * 1_000_000_000.0) as u64,
                        data: if current_builder.custom_data.is_empty() {
                            vec![0u8; current_builder.space as usize]
                        } else {
                            current_builder.custom_data.as_bytes().to_vec()
                        },
                        executable: current_builder.executable,
                        rent_epoch: 0,
                    };

                    current.account_data = Some(account_data.clone());
                    current.deployment_status =
                        crate::services::surfpool_service::DeploymentStatus::Queued;
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

        // Create simplified deployment request
        let deployment_request = DeploymentRequest::new(
            current_builder
                .account_data
                .as_ref()
                .unwrap()
                .pubkey
                .clone(),
            system_program::id(),
            (current_builder.initial_balance * 1_000_000_000.0) as u64,
            current_builder.space as u64,
            current_builder.executable,
            current_builder.account_data.as_ref().unwrap().data.clone(),
            current_builder.keypair.clone(),
        );

        // Deploy using simplified workflow
        use_coroutine(|_| {
            let builder_state = builder.clone();
            let is_deploying_signal = is_deploying.clone();
            let deployment_req = deployment_request;
            let on_deploy = props.on_deploy.clone();
            on_deployment_result = props.on_deployment_result.clone();
            let success_msg = success_message.clone();
            let error_msg = error_message.clone();

            async move {
                // Simulate deployment process
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

                let mut current = builder_state();
                current.deployment_status =
                    crate::services::surfpool_service::DeploymentStatus::Completed {
                        signature: "mock_signature_12345".to_string(),
                    };
                builder_state.set(current);

                // Create mock transaction
                let mock_transaction = Transaction::new_with_payer(&[], None);
                on_deploy.call(mock_transaction);

                // Create mock deployment result
                let deployment_result = DeploymentResult {
                    status: crate::services::surfpool_service::DeploymentStatus::Completed {
                        signature: "mock_signature_12345".to_string(),
                    },
                    signature: Some("mock_signature_12345".to_string()),
                    pubkey: current_builder
                        .account_data
                        .as_ref()
                        .unwrap()
                        .pubkey
                        .clone(),
                    timestamp: chrono::Utc::now(),
                    error: None,
                    block_height: Some(100),
                };
                on_deployment_result.call(deployment_result);

                success_msg.set("Account deployed successfully! (mock deployment)".to_string());
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
        div { style: "display: flex; flex-direction: column; height: 100vh; background: #f9fafb;",
            // Header
            div { style: "background: #ffffff; border-bottom: 1px solid #e5e7eb; padding: 16px 24px; display: flex; align-items: center; justify-content: space-between;",
                h1 { style: "font-size: 20px; font-weight: 600; color: #111827; margin: 0;",
                    "Account Explorer"
                }

                div { style: "display: flex; align-items: center; gap: 16px;",
                    // Validator status indicator
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        div { style: "width: 8px; height: 8px; border-radius: 50%; background: {};",
                            match validator_status() {
                                SurfPoolStatus::Running { .. } => "#22c55e".to_string(),
                                SurfPoolStatus::Starting => "#f59e0b".to_string(),
                                _ => "#ef4444".to_string()
                            }
                        }
                        span { style: "font-size: 14px; color: #6b7280;",
                            match validator_status() {
                                SurfPoolStatus::Running { block_height } => {
                                    format!("SurfPool Running (Block: {})", block_height)
                                }
                                SurfPoolStatus::Starting => "SurfPool Starting...".to_string(),
                                SurfPoolStatus::Stopping => "SurfPool Stopping...".to_string(),
                                SurfPoolStatus::Stopped => "SurfPool Stopped".to_string(),
                                SurfPoolStatus::Error(_) => "SurfPool Error".to_string(),
                            }
                        }
                    }

                    // Validator controls
                    div { style: "display: flex; gap: 8px;",
                        if !matches!(validator_status(), SurfPoolStatus::Running { .. }) {
                            button {
                                style: "padding: 6px 12px; background: #22c55e; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: start_validator,
                                "Start Validator"
                            }
                        }
                        if matches!(validator_status(), SurfPoolStatus::Running { .. }) {
                            button {
                                style: "padding: 6px 12px; background: #ef4444; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: stop_validator,
                                "Stop Validator"
                            }
                        }
                    }

                    // Deployment statistics
                    div { style: "display: flex; align-items: center; gap: 12px; padding: 4px 8px; background: #f3f4f6; border-radius: 4px;",
                        span { style: "font-size: 12px; color: #6b7280;",
                            format!("Deployments: {}/{} ({}%)",
                                deployment_stats().successful_deployments,
                                deployment_stats().total_deployments,
                                deployment_stats().success_rate as usize
                            )
                        }
                    }
                }
            }

            // Tab Navigation
            div { style: "background: #ffffff; border-bottom: 1px solid #e5e7eb; padding: 0 24px;",
                div { style: "display: flex; gap: 32px;",
                    button {
                        onclick: move |_| active_tab.set("builder".to_string()),
                        style: "padding: 16px 0; background: none; border: none; border-bottom: 2px solid {border_color}; font-size: 14px; font-weight: 500; color: {text_color}; cursor: pointer; transition: all 0.2s ease;",
                        if active_tab() == "builder" { "#3b82f6" } else { "transparent" },
                        if active_tab() == "builder" { "#3b82f6" } else { "#6b7280" },
                        "Build Account"
                    }
                    button {
                        onclick: move |_| active_tab.set("explorer".to_string()),
                        style: "padding: 16px 0; background: none; border: none; border-bottom: 2px solid {border_color}; font-size: 14px; font-weight: 500; color: {text_color}; cursor: pointer; transition: all 0.2s ease;",
                        if active_tab() == "explorer" { "#3b82f6" } else { "transparent" },
                        if active_tab() == "explorer" { "#3b82f6" } else { "#6b7280" },
                        "Explore Accounts"
                    }
                }
            }

            // Content Area
            div { style: "flex: 1; padding: 24px; overflow-y: auto;",
                // Error Message
                if !error_message().is_empty() {
                    div { style: "background: #fef2f2; border: 1px solid #fecaca; color: #dc2626; padding: 12px 16px; border-radius: 6px; margin-bottom: 16px; font-size: 14px;",
                        "{error_message}"
                    }
                }

                // Success Message
                if !success_message().is_empty() {
                    div { style: "background: #f0fdf4; border: 1px solid #bbf7d0; color: #166534; padding: 12px 16px; border-radius: 6px; margin-bottom: 16px; font-size: 14px;",
                        "{success_message}"
                    }
                }

                // Tab Content
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
                            "Account Name *"
                        }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.name}",
                            placeholder: "My Account"
                        }
                    }

                    // Description
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Description"
                        }
                        textarea {
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827; resize: vertical; min-height: 80px;",
                            value: "{builder.description}",
                            placeholder: "Account description...",
                            rows: 3
                        }
                    }

                    // Initial Balance
                    div {
                        label { style: "display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 6px;",
                            "Initial Balance (SOL) *"
                        }
                        input {
                            r#type: "number",
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
                            r#type: "text",
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
                            r#type: "number",
                            style: "width: 100%; padding: 8px 12px; border: 1px solid #d1d5db; border-radius: 6px; font-size: 14px; color: #111827;",
                            value: "{builder.space}",
                            min: "0"
                        }
                    }

                    // Executable
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        input {
                            r#type: "checkbox",
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
                            if matches!(builder.deployment_status, crate::services::surfpool_service::DeploymentStatus::Queued) {
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
                                    match builder.deployment_status {
                                        crate::services::surfpool_service::DeploymentStatus::Queued => "Queued",
                                        crate::services::surfpool_service::DeploymentStatus::InProgress => "In Progress",
                                        crate::services::surfpool_service::DeploymentStatus::Completed { .. } => "Completed",
                                        crate::services::surfpool_service::DeploymentStatus::Failed { .. } => "Failed",
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
        div { style: "display: flex; flex-direction: column; gap: 24px;",
            // Status Banner
            div { style: "padding: 16px; background: #eff6ff; border: 1px solid #bfdbfe; border-radius: 8px; color: #1e40af;",
                div { style: "display: flex; align-items: flex-start; gap: 12px;",
                    div {
                        div { style: "font-weight: 600; color: #1e40af; margin-bottom: 4px;",
                            "Account Explorer"
                        }
                        div { style: "font-size: 14px; color: #1e40af; line-height: 1.4;",
                            if accounts.is_empty() {
                                "No accounts built yet. Switch to the 'Build Account' tab to create your first account."
                            } else {
                                format!("Found {} account(s). Click on any account to view details.", accounts.len())
                            }
                        }

                        // Show validator status
                        div { style: "margin-top: 8px; padding: 8px; background: #fef3c7; border-radius: 4px; font-size: 12px; color: #92400e;",
                            match validator_status() {
                                SurfPoolStatus::Running { .. } => "✅ Validator is running and ready for deployments",
                                SurfPoolStatus::Starting => "⏳ Validator is starting up...",
                                SurfPoolStatus::Stopped => "⚠️ Validator is stopped. Start it to deploy accounts.",
                                _ => "❌ Validator error. Check logs for details."
                            }
                        }

                        // Import/Export controls
                        div { style: "margin-top: 16px; display: flex; gap: 8px;",
                            button {
                                style: "padding: 8px 16px; background: #6b7280; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: move |_| {
                                    let export_data = export_accounts(accounts.clone());
                                    // In real implementation, this would trigger a file download
                                    log::info!("Export data: {}", export_data);
                                },
                                "📤 Export Accounts"
                            }

                            button {
                                style: "padding: 8px 16px; background: #3b82f6; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: move |_| {
                                    // In real implementation, this would open a file dialog
                                    log::info!("Import accounts triggered");
                                },
                                "📥 Import Accounts"
                            }
                        }
                    }
                }
            }

            // Accounts List
            if !accounts.is_empty() {
                div { style: "background: #ffffff; border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden;",
                    ul { style: "list-style: none; margin: 0; padding: 0;",
                        for account in accounts.iter() {
                            let account_balance = use_balance_monitor(account.pubkey);

                            li {
                                style: "padding: 16px; border-bottom: 1px solid #f3f4f6; display: flex; align-items: center; justify-content: space-between; transition: background-color 0.2s ease;",

                                div { style: "flex: 1; min-width: 0;",
                                    p { style: "font-size: 14px; font-weight: 500; color: #3b82f6; margin: 0 0 4px 0; word-break: break-all;",
                                        "{account.pubkey}"
                                    }
                                    p { style: "font-size: 12px; color: #6b7280; margin: 0;",
                                        "Owner: {account.owner} • {account_balance:.4} SOL"
                                    }
                                }

                                div { style: "display: flex; align-items: center; gap: 8px;",
                                    if account.executable {
                                        span { style: "padding: 4px 8px; background: #dcfce7; color: #166534; border-radius: 12px; font-size: 10px; font-weight: 500;",
                                            "Executable"
                                        }
                                    }
                                    span { style: "padding: 4px 8px; background: #e0e7ff; color: #3730a3; border-radius: 12px; font-size: 10px; font-weight: 500;",
                                        "{account.data.len()} bytes"
                                    }
                                    span { style: "padding: 4px 8px; background: #fbbf24; color: #92400e; border-radius: 12px; font-size: 10px; font-weight: 500;",
                                        format!("🔄 {:.4} SOL", account_balance)
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Empty State
            if accounts.is_empty() {
                div { style: "text-align: center; padding: 48px 24px;",
                    h3 { style: "font-size: 16px; font-weight: 600; color: #111827; margin: 0 0 8px 0;",
                        "No accounts"
                    }
                    p { style: "font-size: 14px; color: #6b7280; margin: 0;",
                        "Get started by building your first account."
                    }
                }
            }
        }
    }
}

// DeploymentResult is already imported from surfpool_service
