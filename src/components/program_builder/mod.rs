use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    String,
    Pubkey,
    Bool,
    Vec(Box<FieldType>),
    Option(Box<FieldType>),
    Custom(String),
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
            FieldType::I8 => write!(f, "i8"),
            FieldType::I16 => write!(f, "i16"),
            FieldType::I32 => write!(f, "i32"),
            FieldType::I64 => write!(f, "i64"),
            FieldType::String => write!(f, "String"),
            FieldType::Pubkey => write!(f, "Pubkey"),
            FieldType::Bool => write!(f, "bool"),
            FieldType::Vec(inner) => write!(f, "Vec<{}>", inner),
            FieldType::Option(inner) => write!(f, "Option<{}>", inner),
            FieldType::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
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
    pub description: Option<String>,
    pub accounts: Vec<Account>,
    pub instructions: Vec<Instruction>,
}

#[component]
pub fn ProgramBuilder() -> Element {
    let mut schema = use_signal(ProgramSchema::default);
    let mut selected_account = use_signal(|| None::<usize>);
    let mut selected_instruction = use_signal(|| None::<usize>);
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

    let remove_account = move |index: usize| {
        schema.with_mut(|s| {
            s.accounts.remove(index);
        });
        selected_account.set(None);
    };

    let add_field = move |account_index: usize| {
        schema.with_mut(|s| {
            if let Some(account) = s.accounts.get_mut(account_index) {
                account.fields.push(Field {
                    name: format!("field{}", account.fields.len() + 1),
                    field_type: FieldType::String,
                    optional: false,
                });
            }
        });
    };

    let remove_field = move |account_index: usize, field_index: usize| {
        schema.with_mut(|s| {
            if let Some(account) = s.accounts.get_mut(account_index) {
                account.fields.remove(field_index);
            }
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

    let remove_instruction = move |index: usize| {
        schema.with_mut(|s| {
            s.instructions.remove(index);
        });
        selected_instruction.set(None);
    };

    let add_instruction_arg = move |instruction_index: usize| {
        schema.with_mut(|s| {
            if let Some(instruction) = s.instructions.get_mut(instruction_index) {
                instruction.args.push(InstructionArg {
                    name: format!("arg{}", instruction.args.len() + 1),
                    arg_type: FieldType::String,
                });
            }
        });
    };

    let remove_instruction_arg = move |instruction_index: usize, arg_index: usize| {
        schema.with_mut(|s| {
            if let Some(instruction) = s.instructions.get_mut(instruction_index) {
                instruction.args.remove(arg_index);
            }
        });
    };

    let copy_json = move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(clipboard) = window.navigator().clipboard() {
                let _ = clipboard.write_text(&json_output());
            }
        }
    };

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            div { class: "max-w-7xl mx-auto",
                h1 { class: "text-3xl font-bold text-gray-900 mb-8", "Solana Program Builder" }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                    // Left Panel - Controls
                    div { class: "lg:col-span-2 space-y-6",
                        // Program Information
                        div { class: "bg-white rounded-lg shadow p-6",
                            h2 { class: "text-xl font-semibold mb-4", "Program Information" }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Program Name" }
                                    input {
                                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        placeholder: "Enter program name",
                                        value: "{schema().name}",
                                        oninput: move |e| schema.with_mut(|s| s.name = e.value())
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Version" }
                                    input {
                                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        placeholder: "0.1.0",
                                        value: "{schema().version}",
                                        oninput: move |e| schema.with_mut(|s| s.version = e.value())
                                    }
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "Description (Optional)" }
                                textarea {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    placeholder: "Program description...",
                                    rows: 3,
                                    value: "{schema().description.as_deref().unwrap_or(\"\")}",
                                    oninput: move |e| schema.with_mut(|s| s.description = if e.value().is_empty() { None } else { Some(e.value()) })
                                }
                            }
                        }

                        // Accounts Section
                        div { class: "bg-white rounded-lg shadow p-6",
                            div { class: "flex justify-between items-center mb-4",
                                h2 { class: "text-xl font-semibold", "Accounts" }
                                button {
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                                    onclick: add_account,
                                    "+ Add Account"
                                }
                            }

                            div { class: "space-y-4",
                                for (account_idx, account) in schema().accounts.iter().enumerate() {
                                    div {
                                        class: "border border-gray-200 rounded-lg p-4 {if selected_account() == Some(account_idx) { \"ring-2 ring-blue-500\" } else { \"\" }}",
                                        onclick: move |_| selected_account.set(Some(account_idx)),

                                        div { class: "flex justify-between items-start mb-3",
                                            input {
                                                class: "text-lg font-medium bg-transparent border-b border-transparent hover:border-gray-300 focus:border-blue-500 focus:outline-none px-1",
                                                value: "{account.name}",
                                                oninput: move |e| schema.with_mut(|s| {
                                                    if let Some(acc) = s.accounts.get_mut(account_idx) {
                                                        acc.name = e.value();
                                                    }
                                                })
                                            }
                                            button {
                                                class: "text-red-600 hover:text-red-800 transition-colors",
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    remove_account(account_idx);
                                                },
                                                "×"
                                            }
                                        }

                                        div { class: "space-y-2",
                                            div { class: "flex justify-between items-center",
                                                h4 { class: "text-sm font-medium text-gray-700", "Fields" }
                                                button {
                                                    class: "text-sm px-2 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors",
                                                    onclick: move |e| {
                                                        e.stop_propagation();
                                                        add_field(account_idx);
                                                    },
                                                    "+ Add Field"
                                                }
                                            }

                                            if account.fields.is_empty() {
                                                p { class: "text-gray-500 text-sm italic", "No fields defined" }
                                            } else {
                                                for (field_idx, field) in account.fields.iter().enumerate() {
                                                    div { class: "flex items-center gap-2 p-2 bg-gray-50 rounded",
                                                        input {
                                                            class: "flex-1 px-2 py-1 border border-gray-300 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500",
                                                            placeholder: "Field name",
                                                            value: "{field.name}",
                                                            oninput: move |e| schema.with_mut(|s| {
                                                                if let Some(acc) = s.accounts.get_mut(account_idx) {
                                                                    if let Some(f) = acc.fields.get_mut(field_idx) {
                                                                        f.name = e.value();
                                                                    }
                                                                }
                                                            })
                                                        }
                                                        select {
                                                            class: "px-2 py-1 border border-gray-300 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500",
                                                            onchange: move |e| schema.with_mut(|s| {
                                                                if let Some(acc) = s.accounts.get_mut(account_idx) {
                                                                    if let Some(f) = acc.fields.get_mut(field_idx) {
                                                                        f.field_type = match e.value().as_str() {
                                                                            "u8" => FieldType::U8,
                                                                            "u16" => FieldType::U16,
                                                                            "u32" => FieldType::U32,
                                                                            "u64" => FieldType::U64,
                                                                            "i8" => FieldType::I8,
                                                                            "i16" => FieldType::I16,
                                                                            "i32" => FieldType::I32,
                                                                            "i64" => FieldType::I64,
                                                                            "String" => FieldType::String,
                                                                            "Pubkey" => FieldType::Pubkey,
                                                                            "bool" => FieldType::Bool,
                                                                            _ => FieldType::String,
                                                                        };
                                                                    }
                                                                }
                                                            }),
                                                            option { value: "String", "String" }
                                                            option { value: "u8", "u8" }
                                                            option { value: "u16", "u16" }
                                                            option { value: "u32", "u32" }
                                                            option { value: "u64", "u64" }
                                                            option { value: "i8", "i8" }
                                                            option { value: "i16", "i16" }
                                                            option { value: "i32", "i32" }
                                                            option { value: "i64", "i64" }
                                                            option { value: "Pubkey", "Pubkey" }
                                                            option { value: "bool", "bool" }
                                                        }
                                                        button {
                                                            class: "text-red-600 hover:text-red-800 text-sm",
                                                            onclick: move |e| {
                                                                e.stop_propagation();
                                                                remove_field(account_idx, field_idx);
                                                            },
                                                            "×"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if schema().accounts.is_empty() {
                                    div { class: "text-center py-8 text-gray-500",
                                        p { "No accounts defined yet. Click 'Add Account' to get started." }
                                    }
                                }
                            }
                        }

                        // Instructions Section
                        div { class: "bg-white rounded-lg shadow p-6",
                            div { class: "flex justify-between items-center mb-4",
                                h2 { class: "text-xl font-semibold", "Instructions" }
                                button {
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                                    onclick: add_instruction,
                                    "+ Add Instruction"
                                }
                            }

                            div { class: "space-y-4",
                                for (instruction_idx, instruction) in schema().instructions.iter().enumerate() {
                                    div {
                                        class: "border border-gray-200 rounded-lg p-4 {if selected_instruction() == Some(instruction_idx) { \"ring-2 ring-blue-500\" } else { \"\" }}",
                                        onclick: move |_| selected_instruction.set(Some(instruction_idx)),

                                        div { class: "flex justify-between items-start mb-3",
                                            input {
                                                class: "text-lg font-medium bg-transparent border-b border-transparent hover:border-gray-300 focus:border-blue-500 focus:outline-none px-1",
                                                value: "{instruction.name}",
                                                oninput: move |e| schema.with_mut(|s| {
                                                    if let Some(inst) = s.instructions.get_mut(instruction_idx) {
                                                        inst.name = e.value();
                                                    }
                                                })
                                            }
                                            button {
                                                class: "text-red-600 hover:text-red-800 transition-colors",
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    remove_instruction(instruction_idx);
                                                },
                                                "×"
                                            }
                                        }

                                        div { class: "space-y-3",
                                            // Account Selection
                                            div {
                                                h4 { class: "text-sm font-medium text-gray-700 mb-2", "Accounts" }
                                                div { class: "grid grid-cols-2 md:grid-cols-3 gap-2",
                                                    for account in schema().accounts.iter() {
                                                        label { class: "flex items-center space-x-2 text-sm",
                                                            input {
                                                                r#type: "checkbox",
                                                                checked: instruction.accounts.contains(&account.name),
                                                                onchange: move |e| schema.with_mut(|s| {
                                                                    if let Some(inst) = s.instructions.get_mut(instruction_idx) {
                                                                        if e.checked() {
                                                                            if !inst.accounts.contains(&account.name) {
                                                                                inst.accounts.push(account.name.clone());
                                                                            }
                                                                        } else {
                                                                            inst.accounts.retain(|a| a != &account.name);
                                                                        }
                                                                    }
                                                                })
                                                            }
                                                            span { "{account.name}" }
                                                        }
                                                    }
                                                }
                                            }

                                            // Arguments
                                            div {
                                                div { class: "flex justify-between items-center mb-2",
                                                    h4 { class: "text-sm font-medium text-gray-700", "Arguments" }
                                                    button {
                                                        class: "text-sm px-2 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            add_instruction_arg(instruction_idx);
                                                        },
                                                        "+ Add Arg"
                                                    }
                                                }

                                                if instruction.args.is_empty() {
                                                    p { class: "text-gray-500 text-sm italic", "No arguments defined" }
                                                } else {
                                                    for (arg_idx, arg) in instruction.args.iter().enumerate() {
                                                        div { class: "flex items-center gap-2 p-2 bg-gray-50 rounded",
                                                            input {
                                                                class: "flex-1 px-2 py-1 border border-gray-300 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500",
                                                                placeholder: "Argument name",
                                                                value: "{arg.name}",
                                                                oninput: move |e| schema.with_mut(|s| {
                                                                    if let Some(inst) = s.instructions.get_mut(instruction_idx) {
                                                                        if let Some(a) = inst.args.get_mut(arg_idx) {
                                                                            a.name = e.value();
                                                                        }
                                                                    }
                                                                })
                                                            }
                                                            select {
                                                                class: "px-2 py-1 border border-gray-300 rounded text-sm focus:outline-none focus:ring-1 focus:ring-blue-500",
                                                                onchange: move |e| schema.with_mut(|s| {
                                                                    if let Some(inst) = s.instructions.get_mut(instruction_idx) {
                                                                        if let Some(a) = inst.args.get_mut(arg_idx) {
                                                                            a.arg_type = match e.value().as_str() {
                                                                                "u8" => FieldType::U8,
                                                                                "u16" => FieldType::U16,
                                                                                "u32" => FieldType::U32,
                                                                                "u64" => FieldType::U64,
                                                                                "i8" => FieldType::I8,
                                                                                "i16" => FieldType::I16,
                                                                                "i32" => FieldType::I32,
                                                                                "i64" => FieldType::I64,
                                                                                "String" => FieldType::String,
                                                                                "Pubkey" => FieldType::Pubkey,
                                                                                "bool" => FieldType::Bool,
                                                                                _ => FieldType::String,
                                                                            };
                                                                        }
                                                                    }
                                                                }),
                                                                option { value: "String", "String" }
                                                                option { value: "u8", "u8" }
                                                                option { value: "u16", "u16" }
                                                                option { value: "u32", "u32" }
                                                                option { value: "u64", "u64" }
                                                                option { value: "i8", "i8" }
                                                                option { value: "i16", "i16" }
                                                                option { value: "i32", "i32" }
                                                                option { value: "i64", "i64" }
                                                                option { value: "Pubkey", "Pubkey" }
                                                                option { value: "bool", "bool" }
                                                            }
                                                            button {
                                                                class: "text-red-600 hover:text-red-800 text-sm",
                                                                onclick: move |e| {
                                                                    e.stop_propagation();
                                                                    remove_instruction_arg(instruction_idx, arg_idx);
                                                                },
                                                                "×"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if schema().instructions.is_empty() {
                                    div { class: "text-center py-8 text-gray-500",
                                        p { "No instructions defined yet. Click 'Add Instruction' to get started." }
                                    }
                                }
                            }
                        }
                    }

                    // Right Panel - JSON Preview
                    div { class: "lg:col-span-1",
                        div { class: "bg-white rounded-lg shadow sticky top-6",
                            div { class: "p-6",
                                div { class: "flex justify-between items-center mb-4",
                                    h2 { class: "text-xl font-semibold", "JSON Output" }
                                    button {
                                        class: "px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700 transition-colors",
                                        onclick: copy_json,
                                        "📋 Copy"
                                    }
                                }

                                div { class: "bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto",
                                    pre {
                                        class: "text-xs font-mono whitespace-pre-wrap break-all",
                                        "{json_output()}"
                                    }
                                }

                                div { class: "mt-4 text-sm text-gray-600",
                                    p { "The JSON above represents your Solana program structure. You can use this schema to generate Anchor code or integrate with other tools." }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
