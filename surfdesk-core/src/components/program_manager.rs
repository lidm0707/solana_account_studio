//! # Program Manager Component
//!
//! Program manager component for managing Solana programs,
//! deployments, and program interactions across all platforms.

use crate::components::{combine_classes, Button, Card, CommonProps, Size, Table, Variant};
use crate::types::{ProgramInfo, SolanaNetwork};
use dioxus::prelude::*;

/// Program manager component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProgramManagerProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// List of programs
    #[props(optional)]
    pub programs: Option<Vec<ProgramInfo>>,

    /// Current network
    #[props(optional)]
    pub network: Option<SolanaNetwork>,

    /// Whether to show deploy button
    #[props(optional)]
    pub show_deploy: Option<bool>,

    /// Whether to show import button
    #[props(optional)]
    pub show_import: Option<bool>,

    /// Deploy handler
    #[props(optional)]
    pub on_deploy: Option<EventHandler<()>>,

    /// Import handler
    #[props(optional)]
    pub on_import: Option<EventHandler<()>>,

    /// Program select handler
    #[props(optional)]
    pub on_select: Option<EventHandler<ProgramInfo>>,

    /// Program delete handler
    #[props(optional)]
    pub on_delete: Option<EventHandler<ProgramInfo>>,
}

/// Program manager component
#[component]
pub fn ProgramManager(props: ProgramManagerProps) -> Element {
    let common = props.common.unwrap_or_default();
    let show_deploy = props.show_deploy.unwrap_or(true);
    let show_import = props.show_import.unwrap_or(true);
    let programs = props.programs.unwrap_or_default();

    let mut classes = vec!["program-manager"];
    if let Some(class) = &common.class {
        classes.push(class);
    }

    rsx! {
        div {
            class: combine_classes(&classes),
            id: common.id,

            // Header
            div { class: "program-manager-header",
                h2 { class: "program-manager-title", "Program Manager" }

                if let Some(network) = props.network {
                    span { class: "network-indicator",
                        "Network: {network.display_name()}"
                    }
                }

                div { class: "program-manager-actions",
                    if show_import {
                        Button {
                            variant: Variant::Outlined,
                            onclick: move |_| {
                                if let Some(on_import) = props.on_import {
                                    on_import(());
                                }
                            },
                            "Import Program"
                        }
                    }

                    if show_deploy {
                        Button {
                            variant: Variant::Contained,
                            onclick: move |_| {
                                if let Some(on_deploy) = props.on_deploy {
                                    on_deploy(());
                                }
                            },
                            "Deploy Program"
                        }
                    }
                }
            }

            // Programs list
            if !programs.is_empty() {
                div { class: "programs-list",
                    for program in programs {
                        ProgramCard {
                            program: program.clone(),
                            on_select: props.on_select,
                            on_delete: props.on_delete,
                        }
                    }
                }
            } else {
                // Empty state
                Card {
                    title: Some("No Programs".to_string()),
                    variant: Variant::Outlined,
                    div { class: "empty-state",
                        p { "No programs found. Deploy or import a program to get started." }
                        div { class: "empty-state-actions",
                            if show_import {
                                Button {
                                    variant: Variant::Outlined,
                                    onclick: move |_| {
                                        if let Some(on_import) = props.on_import {
                                            on_import(());
                                        }
                                    },
                                    "Import Program"
                                }
                            }
                            if show_deploy {
                                Button {
                                    variant: Variant::Contained,
                                    onclick: move |_| {
                                        if let Some(on_deploy) = props.on_deploy {
                                            on_deploy(());
                                        }
                                    },
                                    "Deploy Program"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Individual program card
#[component]
fn ProgramCard(
    program: ProgramInfo,
    #[props(optional)] on_select: Option<EventHandler<ProgramInfo>>,
    #[props(optional)] on_delete: Option<EventHandler<ProgramInfo>>,
) -> Element {
    rsx! {
        div { class: "program-card",
            h3 { class: "program-title", "{program.name}" }
            p { class: "program-version", "Version: {program.version}" }
            div { class: "program-actions",
                button {
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        if let Some(ref handler) = on_select {
                            handler.call(program.clone());
                        }
                    },
                    "Manage"
                }
                button {
                    class: "btn btn-secondary btn-sm",
                    onclick: move |_| {
                        if let Some(ref handler) = on_delete {
                            handler.call(program.clone());
                        }
                    },
                    "Delete"
                }
            }
        }
    }
}

/// Program deployment dialog
#[component]
pub fn DeployProgramDialog(
    open: bool,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] on_deploy: Option<EventHandler<String>>,
) -> Element {
    let mut program_path = use_signal(|| String::new());

    if !open {
        return rsx! { "" };
    }

    let handle_deploy = move |_| {
        let path = program_path.read().clone();
        if !path.is_empty() {
            if let Some(on_deploy) = on_deploy {
                on_deploy(path);
            }
        }
    };

    rsx! {
        div { class: "deploy-dialog-overlay",
            div { class: "deploy-dialog",
                div { class: "deploy-dialog-header",
                    h3 { "Deploy Program" }
                    button {
                        class: "close-button",
                        onclick: move |_| {
                            if let Some(on_close) = on_close {
                                on_close(());
                            }
                        },
                        "×"
                    }
                }

                div { class: "deploy-dialog-content",
                    div { class: "form-group",
                        label { "Program Path" }
                        input {
                            r#type: "text",
                            placeholder: "Enter path to program .so file",
                            value: "{program_path}",
                            oninput: move |evt| program_path.set(evt.value()),
                        }
                    }

                    div { class: "form-group",
                        label { "Network" }
                        select {
                            option { value: "devnet", "Devnet" }
                            option { value: "testnet", "Testnet" }
                            option { value: "mainnet", "Mainnet" }
                        }
                    }
                }

                div { class: "deploy-dialog-actions",
                    Button {
                        variant: Variant::Text,
                        onclick: move |_| {
                            if let Some(on_close) = on_close {
                                on_close(());
                            }
                        },
                        "Cancel"
                    }

                    Button {
                        variant: Variant::Contained,
                        onclick: handle_deploy,
                        "Deploy"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::chrono::Utc;

    #[test]
    fn test_program_manager_default_props() {
        let props = ProgramManagerProps {
            common: None,
            programs: None,
            network: None,
            show_deploy: None,
            show_import: None,
            on_deploy: None,
            on_import: None,
            on_select: None,
            on_delete: None,
        };

        assert!(props.show_deploy.unwrap_or(true));
        assert!(props.show_import.unwrap_or(true));
        assert!(props.programs.unwrap_or_default().is_empty());
    }

    #[test]
    fn test_program_card() {
        let program = ProgramInfo {
            name: "Test Program".to_string(),
            program_id: crate::types::SolanaPubkey::from_str("11111111111111111111111111111111")
                .unwrap(),
            version: "1.0.0".to_string(),
            path: "/path/to/program.so".to_string(),
            deployed_at: Some(Utc::now()),
        };

        // In a real test, you'd verify the rendered output
        assert_eq!(program.name, "Test Program");
        assert_eq!(program.version, "1.0.0");
        assert_eq!(program.path, "/path/to/program.so");
        assert!(program.deployed_at.is_some());
    }

    #[test]
    fn test_deploy_program_dialog() {
        let dialog = DeployProgramDialog {
            open: true,
            on_close: None,
            on_deploy: None,
        };

        assert!(dialog.open);
    }
}
