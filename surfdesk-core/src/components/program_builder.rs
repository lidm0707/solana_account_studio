//! Program Builder Component
//!
//! This component provides a visual interface for designing Solana programs
//! and generating JSON schemas automatically.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldType {
    U8,
    U16,
    U32,
    U64,
    String,
    Pubkey,
    Bool,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::String
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::U8 => write!(f, "u8"),
            FieldType::U16 => write!(f, "u16"),
            FieldType::U32 => write!(f, "u32"),
            FieldType::U64 => write!(f, "u64"),
            FieldType::String => write!(f, "String"),
            FieldType::Pubkey => write!(f, "Pubkey"),
            FieldType::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Account {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct InstructionArg {
    pub name: String,
    pub arg_type: FieldType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Instruction {
    pub name: String,
    pub accounts: Vec<String>,
    pub args: Vec<InstructionArg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ProgramSchema {
    pub name: String,
    pub version: String,
    pub accounts: Vec<Account>,
    pub instructions: Vec<Instruction>,
}

#[component]
pub fn ProgramBuilder() -> Element {
    let mut schema = use_signal(ProgramSchema::default);
    let mut json_output = use_signal(|| String::new());

    // Update JSON output whenever schema changes
    use_effect(move || {
        if let Ok(json) = serde_json::to_string_pretty(&schema()) {
            json_output.set(json);
        }
    });

    let add_account = move |_| {
        schema.with_mut(|s| {
            s.accounts.push(Account {
                name: format!("Account{}", s.accounts.len() + 1),
                fields: vec![],
            });
        });
    };

    let add_instruction = move |_| {
        schema.with_mut(|s| {
            s.instructions.push(Instruction {
                name: format!("instruction{}", s.instructions.len() + 1),
                accounts: vec![],
                args: vec![],
            });
        });
    };

    rsx! {
        div {
            style: "min-height: 100vh; background-color: #f9fafb; padding: 2rem;",

            h1 {
                style: "font-size: 2rem; font-weight: bold; color: #1f2937; margin-bottom: 2rem;",
                "Solana Program Builder"
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;",

                // Left Panel - Controls
                div {
                    style: "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);",

                    // Program Info
                    div {
                        style: "margin-bottom: 2rem;",
                        h2 {
                            style: "font-size: 1.25rem; font-weight: 600; color: #1f2937; margin-bottom: 1rem;",
                            "Program Information"
                        }
                        div {
                            style: "margin-bottom: 1rem;",
                            label {
                                style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;",
                                "Program Name"
                            }
                            input {
                                style: "width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                                placeholder: "Enter program name",
                                value: "{schema().name}",
                                oninput: move |e| schema.with_mut(|s| s.name = e.value())
                            }
                        }
                        div {
                            label {
                                style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;",
                                "Version"
                            }
                            input {
                                style: "width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                                placeholder: "0.1.0",
                                value: "{schema().version}",
                                oninput: move |e| schema.with_mut(|s| s.version = e.value())
                            }
                        }
                    }

                    // Accounts Section
                    div {
                        style: "margin-bottom: 2rem;",
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;",
                            h2 {
                                style: "font-size: 1.25rem; font-weight: 600; color: #1f2937;",
                                "Accounts"
                            }
                            button {
                                style: "padding: 0.5rem 1rem; background-color: #3b82f6; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                                onclick: add_account,
                                "+ Add Account"
                            }
                        }

                        div {
                            style: "space-y: 1rem;",
                            for (account_idx, account) in schema().accounts.iter().enumerate() {
                                div {
                                    key: "{account_idx}",
                                    style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1rem;",
                                    input {
                                        style: "width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem; margin-bottom: 0.5rem;",
                                        value: "{account.name}",
                                        oninput: move |e| schema.with_mut(|s| {
                                            if let Some(acc) = s.accounts.get_mut(account_idx) {
                                                acc.name = e.value();
                                            }
                                        })
                                    }
                                    p {
                                        style: "font-size: 0.875rem; color: #6b7280;",
                                        "Fields: {account.fields.len()}"
                                    }
                                }
                            }
                        }
                    }

                    // Instructions Section
                    div {
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;",
                            h2 {
                                style: "font-size: 1.25rem; font-weight: 600; color: #1f2937;",
                                "Instructions"
                            }
                            button {
                                style: "padding: 0.5rem 1rem; background-color: #3b82f6; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                                onclick: add_instruction,
                                "+ Add Instruction"
                            }
                        }

                        div {
                            for (instruction_idx, instruction) in schema().instructions.iter().enumerate() {
                                div {
                                    key: "{instruction_idx}",
                                    style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; margin-bottom: 1rem;",
                                    input {
                                        style: "width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem; margin-bottom: 0.5rem;",
                                        value: "{instruction.name}",
                                        oninput: move |e| schema.with_mut(|s| {
                                            if let Some(inst) = s.instructions.get_mut(instruction_idx) {
                                                inst.name = e.value();
                                            }
                                        })
                                    }
                                    p {
                                        style: "font-size: 0.875rem; color: #6b7280;",
                                        "Accounts: {instruction.accounts.len()}, Args: {instruction.args.len()}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Right Panel - JSON Preview
                div {
                    style: "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);",

                    h2 {
                        style: "font-size: 1.25rem; font-weight: 600; color: #1f2937; margin-bottom: 1rem;",
                        "JSON Output"
                    }

                    div {
                        style: "background-color: #1f2937; color: #f9fafb; padding: 1rem; border-radius: 0.375rem; font-family: monospace; font-size: 0.875rem; white-space: pre-wrap; max-height: 600px; overflow-y: auto;",
                        "{json_output()}"
                    }

                    button {
                        style: "margin-top: 1rem; padding: 0.5rem 1rem; background-color: #10b981; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                        onclick: move |_| {
                            // Copy to clipboard functionality would go here
                            // For now, just log the action
                            tracing::info!("Copy JSON to clipboard requested");
                        },
                        "Copy JSON"
                    }
                }
            }
        }
    }
}
