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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstructionTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub required_accounts: Vec<String>,
    pub optional_accounts: Vec<String>,
    pub args: Vec<InstructionArg>,
    pub category: String,
}

impl InstructionTemplate {
    pub fn swap() -> Self {
        Self {
            id: "swap".to_string(),
            name: "Swap".to_string(),
            description: "Swap tokens between accounts".to_string(),
            required_accounts: vec![
                "user_account".to_string(),
                "pool_account".to_string(),
                "mint_a".to_string(),
                "mint_b".to_string(),
                "authority".to_string(),
            ],
            optional_accounts: vec!["referral_account".to_string()],
            args: vec![
                InstructionArg {
                    name: "amount_in".to_string(),
                    arg_type: FieldType::U64,
                },
                InstructionArg {
                    name: "minimum_amount_out".to_string(),
                    arg_type: FieldType::U64,
                },
            ],
            category: "Token Operations".to_string(),
        }
    }

    pub fn transfer() -> Self {
        Self {
            id: "transfer".to_string(),
            name: "Transfer".to_string(),
            description: "Transfer tokens or SOL between accounts".to_string(),
            required_accounts: vec![
                "from_account".to_string(),
                "to_account".to_string(),
                "authority".to_string(),
            ],
            optional_accounts: vec![],
            args: vec![InstructionArg {
                name: "amount".to_string(),
                arg_type: FieldType::U64,
            }],
            category: "Token Operations".to_string(),
        }
    }

    pub fn add() -> Self {
        Self {
            id: "add".to_string(),
            name: "Add Liquidity".to_string(),
            description: "Add liquidity to a pool".to_string(),
            required_accounts: vec![
                "user_account".to_string(),
                "pool_account".to_string(),
                "mint_a".to_string(),
                "mint_b".to_string(),
                "authority".to_string(),
            ],
            optional_accounts: vec![],
            args: vec![
                InstructionArg {
                    name: "amount_a".to_string(),
                    arg_type: FieldType::U64,
                },
                InstructionArg {
                    name: "amount_b".to_string(),
                    arg_type: FieldType::U64,
                },
            ],
            category: "Liquidity".to_string(),
        }
    }

    pub fn initialize_account() -> Self {
        Self {
            id: "initialize_account".to_string(),
            name: "Initialize Account".to_string(),
            description: "Initialize a new account".to_string(),
            required_accounts: vec!["new_account".to_string(), "authority".to_string()],
            optional_accounts: vec![],
            args: vec![],
            category: "Account Management".to_string(),
        }
    }

    pub fn close_account() -> Self {
        Self {
            id: "close_account".to_string(),
            name: "Close Account".to_string(),
            description: "Close an account and reclaim lamports".to_string(),
            required_accounts: vec![
                "account_to_close".to_string(),
                "destination_account".to_string(),
                "authority".to_string(),
            ],
            optional_accounts: vec![],
            args: vec![],
            category: "Account Management".to_string(),
        }
    }

    pub fn store_data() -> Self {
        Self {
            id: "store_data".to_string(),
            name: "Store Data".to_string(),
            description: "Store data in an account".to_string(),
            required_accounts: vec!["data_account".to_string(), "authority".to_string()],
            optional_accounts: vec![],
            args: vec![InstructionArg {
                name: "data".to_string(),
                arg_type: FieldType::String,
            }],
            category: "Data Operations".to_string(),
        }
    }

    pub fn update_data() -> Self {
        Self {
            id: "update_data".to_string(),
            name: "Update Data".to_string(),
            description: "Update existing data in an account".to_string(),
            required_accounts: vec!["data_account".to_string(), "authority".to_string()],
            optional_accounts: vec![],
            args: vec![InstructionArg {
                name: "new_data".to_string(),
                arg_type: FieldType::String,
            }],
            category: "Data Operations".to_string(),
        }
    }

    pub fn delete_data() -> Self {
        Self {
            id: "delete_data".to_string(),
            name: "Delete Data".to_string(),
            description: "Delete data from an account".to_string(),
            required_accounts: vec!["data_account".to_string(), "authority".to_string()],
            optional_accounts: vec![],
            args: vec![],
            category: "Data Operations".to_string(),
        }
    }

    pub fn validate_owner() -> Self {
        Self {
            id: "validate_owner".to_string(),
            name: "Validate Owner".to_string(),
            description: "Validate account ownership".to_string(),
            required_accounts: vec!["account".to_string(), "expected_owner".to_string()],
            optional_accounts: vec![],
            args: vec![],
            category: "Validation".to_string(),
        }
    }

    pub fn check_balance() -> Self {
        Self {
            id: "check_balance".to_string(),
            name: "Check Balance".to_string(),
            description: "Check account balance".to_string(),
            required_accounts: vec!["account".to_string()],
            optional_accounts: vec![],
            args: vec![],
            category: "Validation".to_string(),
        }
    }

    pub fn verify_signature() -> Self {
        Self {
            id: "verify_signature".to_string(),
            name: "Verify Signature".to_string(),
            description: "Verify a signature".to_string(),
            required_accounts: vec!["message_account".to_string(), "signer_account".to_string()],
            optional_accounts: vec![],
            args: vec![InstructionArg {
                name: "signature".to_string(),
                arg_type: FieldType::String,
            }],
            category: "Validation".to_string(),
        }
    }

    pub fn get_all_templates() -> Vec<Self> {
        vec![
            Self::swap(),
            Self::transfer(),
            Self::add(),
            Self::initialize_account(),
            Self::close_account(),
            Self::store_data(),
            Self::update_data(),
            Self::delete_data(),
            Self::validate_owner(),
            Self::check_balance(),
            Self::verify_signature(),
        ]
    }
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
    let mut show_templates = use_signal(|| false);
    let instruction_templates = use_signal(|| InstructionTemplate::get_all_templates());

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

    let add_swap_instruction = move |_| {
        schema.with_mut(|s| {
            s.instructions.push(Instruction {
                name: "Swap".to_string(),
                accounts: vec![
                    "user_account".to_string(),
                    "pool_account".to_string(),
                    "mint_a".to_string(),
                    "mint_b".to_string(),
                    "authority".to_string(),
                ],
                args: vec![
                    InstructionArg {
                        name: "amount_in".to_string(),
                        arg_type: FieldType::U64,
                    },
                    InstructionArg {
                        name: "minimum_amount_out".to_string(),
                        arg_type: FieldType::U64,
                    },
                ],
            });
        });
        show_templates.set(false);
    };

    let add_transfer_instruction = move |_| {
        schema.with_mut(|s| {
            s.instructions.push(Instruction {
                name: "Transfer".to_string(),
                accounts: vec![
                    "from_account".to_string(),
                    "to_account".to_string(),
                    "authority".to_string(),
                ],
                args: vec![InstructionArg {
                    name: "amount".to_string(),
                    arg_type: FieldType::U64,
                }],
            });
        });
        show_templates.set(false);
    };

    let add_liquidity_instruction = move |_| {
        schema.with_mut(|s| {
            s.instructions.push(Instruction {
                name: "Add Liquidity".to_string(),
                accounts: vec![
                    "user_account".to_string(),
                    "pool_account".to_string(),
                    "mint_a".to_string(),
                    "mint_b".to_string(),
                    "authority".to_string(),
                ],
                args: vec![
                    InstructionArg {
                        name: "amount_a".to_string(),
                        arg_type: FieldType::U64,
                    },
                    InstructionArg {
                        name: "amount_b".to_string(),
                        arg_type: FieldType::U64,
                    },
                ],
            });
        });
        show_templates.set(false);
    };

    let add_initialize_instruction = move |_| {
        schema.with_mut(|s| {
            s.instructions.push(Instruction {
                name: "Initialize Account".to_string(),
                accounts: vec!["new_account".to_string(), "authority".to_string()],
                args: vec![],
            });
        });
        show_templates.set(false);
    };

    let toggle_templates = move |_| {
        show_templates.set(!show_templates());
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
                            div {
                                style: "display: flex; gap: 0.5rem;",
                                button {
                                    style: "padding: 0.5rem 1rem; background-color: #3b82f6; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                                    onclick: toggle_templates,
                                    "📦 Templates"
                                }
                                button {
                                    style: "padding: 0.5rem 1rem; background-color: #10b981; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                                    onclick: add_instruction,
                                    "+ Custom"
                                }
                            }
                        }

                        // Template Selection Modal
                        if show_templates() {
                            div {
                                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                                div {
                                    style: "background-color: white; border-radius: 0.5rem; padding: 2rem; max-width: 600px; max-height: 80vh; overflow-y: auto; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);",

                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",
                                        h3 {
                                            style: "font-size: 1.5rem; font-weight: 600; color: #1f2937;",
                                            "Instruction Templates"
                                        }
                                        button {
                                            style: "padding: 0.5rem; background-color: #ef4444; color: white; border: none; border-radius: 0.375rem; cursor: pointer;",
                                            onclick: move |_| show_templates.set(false),
                                            "×"
                                        }
                                    }

                                    // Token Operations
                                    div {
                                        style: "margin-bottom: 2rem;",
                                        h4 {
                                            style: "font-size: 1.125rem; font-weight: 600; color: #374151; margin-bottom: 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid #e5e7eb;",
                                            "Token Operations"
                                        }

                                        div {
                                            style: "display: grid; gap: 0.75rem;",
                                            div {
                                                style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; cursor: pointer; transition: all 0.2s;",
                                                onclick: add_swap_instruction,
                                                div {
                                                    style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 0.5rem;",
                                                    h5 {
                                                        style: "font-size: 1rem; font-weight: 600; color: #1f2937; margin: 0;",
                                                        "Swap"
                                                    }
                                                    span {
                                                        style: "padding: 0.25rem 0.5rem; background-color: #dbeafe; color: #1e40af; font-size: 0.75rem; border-radius: 0.25rem;",
                                                        "Token Operations"
                                                    }
                                                }
                                                p {
                                                    style: "color: #6b7280; font-size: 0.875rem; margin: 0 0 0.75rem 0;",
                                                    "Swap tokens between accounts"
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151;",
                                                    strong { "Required: " }
                                                    span { "user_account, pool_account, mint_a, mint_b, authority" }
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151; margin-top: 0.25rem;",
                                                    strong { "Args: " }
                                                    span { "amount_in: u64, minimum_amount_out: u64" }
                                                }
                                            }

                                            div {
                                                style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; cursor: pointer; transition: all 0.2s;",
                                                onclick: add_transfer_instruction,
                                                div {
                                                    style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 0.5rem;",
                                                    h5 {
                                                        style: "font-size: 1rem; font-weight: 600; color: #1f2937; margin: 0;",
                                                        "Transfer"
                                                    }
                                                    span {
                                                        style: "padding: 0.25rem 0.5rem; background-color: #dbeafe; color: #1e40af; font-size: 0.75rem; border-radius: 0.25rem;",
                                                        "Token Operations"
                                                    }
                                                }
                                                p {
                                                    style: "color: #6b7280; font-size: 0.875rem; margin: 0 0 0.75rem 0;",
                                                    "Transfer tokens or SOL between accounts"
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151;",
                                                    strong { "Required: " }
                                                    span { "from_account, to_account, authority" }
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151; margin-top: 0.25rem;",
                                                    strong { "Args: " }
                                                    span { "amount: u64" }
                                                }
                                            }
                                        }
                                    }

                                    // Liquidity Operations
                                    div {
                                        style: "margin-bottom: 2rem;",
                                        h4 {
                                            style: "font-size: 1.125rem; font-weight: 600; color: #374151; margin-bottom: 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid #e5e7eb;",
                                            "Liquidity Operations"
                                        }

                                        div {
                                            style: "display: grid; gap: 0.75rem;",
                                            div {
                                                style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; cursor: pointer; transition: all 0.2s;",
                                                onclick: add_liquidity_instruction,
                                                div {
                                                    style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 0.5rem;",
                                                    h5 {
                                                        style: "font-size: 1rem; font-weight: 600; color: #1f2937; margin: 0;",
                                                        "Add Liquidity"
                                                    }
                                                    span {
                                                        style: "padding: 0.25rem 0.5rem; background-color: #dcfce7; color: #166534; font-size: 0.75rem; border-radius: 0.25rem;",
                                                        "Liquidity"
                                                    }
                                                }
                                                p {
                                                    style: "color: #6b7280; font-size: 0.875rem; margin: 0 0 0.75rem 0;",
                                                    "Add liquidity to a pool"
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151;",
                                                    strong { "Required: " }
                                                    span { "user_account, pool_account, mint_a, mint_b, authority" }
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151; margin-top: 0.25rem;",
                                                    strong { "Args: " }
                                                    span { "amount_a: u64, amount_b: u64" }
                                                }
                                            }
                                        }
                                    }

                                    // Account Management
                                    div {
                                        style: "margin-bottom: 2rem;",
                                        h4 {
                                            style: "font-size: 1.125rem; font-weight: 600; color: #374151; margin-bottom: 1rem; padding-bottom: 0.5rem; border-bottom: 1px solid #e5e7eb;",
                                            "Account Management"
                                        }

                                        div {
                                            style: "display: grid; gap: 0.75rem;",
                                            div {
                                                style: "border: 1px solid #e5e7eb; border-radius: 0.375rem; padding: 1rem; cursor: pointer; transition: all 0.2s;",
                                                onclick: add_initialize_instruction,
                                                div {
                                                    style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 0.5rem;",
                                                    h5 {
                                                        style: "font-size: 1rem; font-weight: 600; color: #1f2937; margin: 0;",
                                                        "Initialize Account"
                                                    }
                                                    span {
                                                        style: "padding: 0.25rem 0.5rem; background-color: #fef3c7; color: #92400e; font-size: 0.75rem; border-radius: 0.25rem;",
                                                        "Account Management"
                                                    }
                                                }
                                                p {
                                                    style: "color: #6b7280; font-size: 0.875rem; margin: 0 0 0.75rem 0;",
                                                    "Initialize a new account"
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: #374151;",
                                                    strong { "Required: " }
                                                    span { "new_account, authority" }
                                                }
                                            }
                                        }
                                    }
                                }
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
