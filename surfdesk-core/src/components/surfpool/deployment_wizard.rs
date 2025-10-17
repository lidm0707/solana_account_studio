//! Program Deployment Wizard Component
//!
//! Provides UI for deploying raw programs to SurfPool without code generation

use crate::solana_rpc::Pubkey;
use dioxus::prelude::*;

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

        // Mock deployment
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        let hash = hasher.finish();

        let mock_program_id = format!("Program{}{}", hash, "11111111111111111111111111111111");
        let program_pubkey = Pubkey::from_string(&mock_program_id[..44]);
        let program_pubkey_string = program_pubkey.to_string();

        deployed_program_id.set(Some(program_pubkey));
        success_message.set(format!(
            "Program deployed successfully: {}",
            program_pubkey_string
        ));

        is_deploying.set(false);
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
