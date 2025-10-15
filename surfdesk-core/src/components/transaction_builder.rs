//! # Transaction Builder Component Module
//!
//! This module provides a transaction builder component for creating
//! and managing Solana transactions in the SurfDesk application.

use super::combine_classes;
use crate::{state::AppState, types::UIState};
use dioxus::prelude::*;

/// Transaction builder component properties
#[derive(Props, Clone, PartialEq)]
pub struct TransactionBuilderProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Application state
    pub state: dioxus::prelude::Signal<AppState>,
    /// UI state
    pub ui_state: dioxus::prelude::Signal<UIState>,
}

/// Transaction builder component
#[component]
pub fn TransactionBuilder(props: TransactionBuilderProps) -> Element {
    let mut classes = vec!["transaction-builder"];

    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,

            // Transaction builder header
            div { class: "transaction-builder-header",
                h2 { class: "builder-title", "Transaction Builder" }
                p { class: "builder-description", "Create and build Solana transactions" }
            }

            // Transaction form
            div { class: "transaction-form",
                // Sender field
                div { class: "form-field",
                    label { class: "form-label", "Sender" }
                    input {
                        class: "form-input",
                        placeholder: "Enter sender public key",
                        type: "text"
                    }
                }

                // Recipient field
                div { class: "form-field",
                    label { class: "form-label", "Recipient" }
                    input {
                        class: "form-input",
                        placeholder: "Enter recipient public key",
                        type: "text"
                    }
                }

                // Amount field
                div { class: "form-field",
                    label { class: "form-label", "Amount (SOL)" }
                    input {
                        class: "form-input",
                        placeholder: "0.0",
                        type: "number",
                        step: "0.000000001"
                    }
                }

                // Instruction type
                div { class: "form-field",
                    label { class: "form-label", "Instruction Type" }
                    select { class: "form-select",
                        option { value: "transfer", "Transfer" }
                        option { value: "custom", "Custom Instruction" }
                    }
                }

                // Action buttons
                div { class: "form-actions",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // Handle transaction building
                            log::debug!("Building transaction");
                        },
                        "Build Transaction"
                    }

                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            // Handle transaction simulation
                            log::debug!("Simulating transaction");
                        },
                        "Simulate"
                    }

                    button {
                        class: "btn btn-success",
                        onclick: move |_| {
                            // Handle transaction sending
                            log::debug!("Sending transaction");
                        },
                        "Send Transaction"
                    }
                }
            }

            // Transaction preview
            div { class: "transaction-preview",
                h3 { class: "preview-title", "Transaction Preview" }
                div { class: "preview-content",
                    div { class: "preview-item",
                        span { class: "preview-label", "Status:" }
                        span { class: "preview-value", "Ready to build" }
                    }

                    div { class: "preview-item",
                        span { class: "preview-label", "Fee:" }
                        span { class: "preview-value", "Calculating..." }
                    }

                    div { class: "preview-item",
                        span { class: "preview-label", "Compute Units:" }
                        span { class: "preview-value", "Calculating..." }
                    }
                }
            }
        }
    }
}
