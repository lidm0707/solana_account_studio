//! Program Deployment Wizard Component
//!
//! Provides UI for deploying raw programs to SurfPool without code generation

use crate::solana_rpc::Pubkey;
use dioxus::prelude::*;

// Hook for accessing SurfPool service (terminal strategy)
fn use_surfpool_service() -> crate::services::surfpool::SurfPoolService {
    use std::cell::RefCell;
    thread_local! {
        static SERVICE: RefCell<Option<crate::services::surfpool::SurfPoolService>> = const { RefCell::new(None) };
    }

    SERVICE.with(|service| {
        if service.borrow().is_none() {
            // Create service using terminal strategy
            match tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(crate::services::surfpool::SurfPoolService::new())
            }) {
                Ok(svc) => {
                    *service.borrow_mut() = Some(svc.clone());
                    service.borrow().as_ref().unwrap().clone()
                }
                Err(_) => {
                    // Return a dummy service if SurfPool is not available
                    let fallback = crate::services::surfpool::SurfPoolService::new_fallback();
                    *service.borrow_mut() = Some(fallback.clone());
                    fallback
                }
            }
        } else {
            service.borrow().as_ref().unwrap().clone()
        }
    })
}

/// Program Deployment Wizard Component
#[component]
pub fn ProgramDeploymentWizard() -> Element {
    let mut raw_code = use_signal(String::new);
    let mut is_deploying = use_signal(|| false);
    let mut deployed_program_id = use_signal(|| Option::<Pubkey>::None);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    // Deploy program
    let deploy_program = move |_| {
        let code = raw_code();

        if code.trim().is_empty() {
            error_message.set("Please enter raw program code".to_string());
            return;
        }

        is_deploying.set(true);
        error_message.set(String::new());
        success_message.set(String::new());

        // Real deployment using SurfPool terminal strategy
        let service = use_surfpool_service();
        let service_clone = service.clone();

        use_coroutine(move |_: dioxus::prelude::UnboundedReceiver<()>| {
            let code_clone = code.clone();
            let mut deploying = is_deploying;
            let mut deployed_id = deployed_program_id;
            let mut error_msg = error_message;
            let mut success_msg = success_message;
            let svc = service_clone.clone();

            async move {
                // Save the code to a temporary file for deployment
                let temp_file = format!(
                    "/tmp/temp_program_{}.so",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                match std::fs::write(&temp_file, &code_clone) {
                    Ok(_) => {
                        match svc.deploy_program(&temp_file).await {
                            Ok(program_pubkey) => {
                                // Parse the pubkey string into a Pubkey
                                let pubkey = Pubkey::from_string(&program_pubkey);
                                deployed_id.set(Some(pubkey));
                                success_msg.set(format!(
                                    "Program deployed successfully via SurfPool terminal: {}",
                                    program_pubkey
                                ));
                            }
                            Err(e) => {
                                error_msg.set(format!("Deployment failed: {}", e));
                            }
                        }
                        // Clean up temp file
                        let _ = std::fs::remove_file(&temp_file);
                    }
                    Err(e) => {
                        error_msg.set(format!("Failed to write program file: {}", e));
                    }
                }

                deploying.set(false);
            }
        });
    };

    rsx! {
        div { class: "program-deployment-wizard",

            // Header
            div { class: "wizard-header",
                h3 { "📦 Program Deployment Wizard" }
                p { "Deploy raw programs without code generation" }
            }

            // Deployment Status
            if let Some(ref program_id) = deployed_program_id() {
                div { class: "deployment-status",
                    h4 { "Deployed Program" }
                    div { class: "program-id",
                        strong { "Program ID: " }
                        code { "{program_id.to_string()}" }
                    }
                }
            }

            // Code Input
            div { class: "code-input-section",
                h4 { "Raw Program Code (Hex)" }
                textarea {
                    class: "code-input",
                    placeholder: "Enter raw program bytecode in hex format...",
                    value: "{raw_code()}",
                    oninput: move |evt| raw_code.set(evt.value()),
                    rows: 10,
                    disabled: is_deploying()
                }

                div { class: "input-controls",
                    button {
                        class: "btn btn-primary",
                        onclick: deploy_program,
                        disabled: is_deploying(),
                        "🚀 Deploy Program"
                    }

                    if is_deploying() {
                        span { class: "deploying-indicator", "⏳ Deploying..." }
                    }
                }
            }

            // Messages
            if !error_message().is_empty() {
                div { class: "error-message",
                    "❌ {error_message()}"
                }
            }

            if !success_message().is_empty() {
                div { class: "success-message",
                    "✅ {success_message()}"
                }
            }

            // Instructions
            div { class: "instructions",
                h4 { "Instructions" }
                ol {
                    li { "Ensure SurfPool is running on port 8999" }
                    li { "Enter raw program bytecode in hex format" }
                    li { "Click 'Deploy Program' to deploy" }
                    li { "Use the deployed program ID for account creation" }
                }
            }
        }
    }
}
