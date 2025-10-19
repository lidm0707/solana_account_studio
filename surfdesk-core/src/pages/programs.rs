//! Program Builder Page Component
//!
//! This page provides the visual interface for creating Solana programs
//! without writing code. It includes a drag-and-drop canvas, component
//! palette, and code generation functionality.

use dioxus::prelude::*;

/// Program Builder page component
#[component]
pub fn ProgramBuilder() -> Element {
    let selected_components = use_signal(|| Vec::<String>::new());
    let program_name = use_signal(|| "my_program".to_string());
    let program_description = use_signal(|| "".to_string());
    let generated_code = use_signal(|| "// Generated code will appear here".to_string());
    let is_deployed = use_signal(|| false);

    rsx! {
        div { class: "program-builder-page",
            div { class: "page-header",
                h1 { "Program Builder" }
                p { "Create Solana programs visually without writing code" }
            }

            // Program Information
            div { class: "program-info-section",
                h2 { "Program Information" }
                div { class: "program-form",
                    div { class: "form-group",
                        label { "Program Name" }
                        input {
                            r#type: "text",
                            class: "form-control",
                            value: "{program_name}",
                            onchange: move |evt| program_name.set(evt.value())
                        }
                    }

                    div { class: "form-group",
                        label { "Description" }
                        textarea {
                            class: "form-control",
                            rows: 3,
                            placeholder: "Describe what your program does...",
                            value: "{program_description}",
                            onchange: move |evt| program_description.set(evt.value())
                        }
                    }
                }
            }

            // Main Builder Area
            div { class: "builder-main",
                // Component Palette
                div { class: "component-palette",
                    h3 { "Components" }
                    div { class: "component-categories",
                        ComponentCategory {
                            title: "Account Management",
                            components: vec![
                                ("Create Account", "👤"),
                                ("Initialize Account", "🔧"),
                                ("Close Account", "🗑️"),
                            ]
                        }
                        ComponentCategory {
                            title: "Data Storage",
                            components: vec![
                                ("Store Data", "💾"),
                                ("Update Data", "✏️"),
                                ("Delete Data", "🗑️"),
                            ]
                        }
                        ComponentCategory {
                            title: "Validation",
                            components: vec![
                                ("Validate Owner", "✅"),
                                ("Check Balance", "💰"),
                                ("Verify Signature", "🔐"),
                            ]
                        }
                        ComponentCategory {
                            title: "Math Operations",
                            components: vec![
                                ("Add", "➕"),
                                ("Subtract", "➖"),
                                ("Multiply", "✖️"),
                                ("Divide", "➗"),
                            ]
                        }
                    }
                }

                // Canvas Area
                div { class: "canvas-area",
                    div { class: "canvas-header",
                        h3 { "Program Canvas" }
                        div { class: "canvas-controls",
                            button {
                                class: "btn btn-secondary btn-small",
                                onclick: move |_| {
                                    selected_components.write().clear();
                                    generated_code.set("// Canvas cleared".to_string());
                                },
                                "🗑️ Clear"
                            }
                            button {
                                class: "btn btn-primary btn-small",
                                onclick: move |_| {
                                    generate_code(&program_name(), &selected_components(), generated_code);
                                },
                                "⚡ Generate Code"
                            }
                        }
                    }

                    div { class: "canvas",
                        if selected_components().is_empty() {
                            div { class: "canvas-empty",
                                div { class: "empty-icon", "🎨" }
                                p { "Drag components here to start building your program" }
                                p { class: "empty-hint", "Select components from the palette on the left" }
                            }
                        } else {
                            div { class: "canvas-content",
                                for (index, component) in selected_components().iter().enumerate() {
                                    div {
                                        key: "{index}",
                                        class: "canvas-component",
                                        div { class: "component-handle", "⋮⋮" }
                                        div { class: "component-content",
                                            h4 { "{component}" }
                                            p { "Component configuration will appear here" }
                                        }
                                        button {
                                            class: "component-remove",
                                            onclick: move |_| {
                                                let mut comps = selected_components.write();
                                                comps.remove(index);
                                            },
                                            "✕"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Code Preview Section
            div { class: "code-preview-section",
                div { class: "code-preview-header",
                    h3 { "Generated Code" }
                    div { class: "code-controls",
                        button {
                            class: "btn btn-secondary btn-small",
                            onclick: move |_| {
                                // TODO: Copy code to clipboard
                                tracing::info!("Copy code to clipboard");
                            },
                            "📋 Copy"
                        }
                        button {
                            class: "btn btn-secondary btn-small",
                            onclick: move |_| {
                                // TODO: Export code
                                tracing::info!("Export code");
                            },
                            "💾 Export"
                        }
                    }
                }

                div { class: "code-preview",
                    pre {
                        code { "{generated_code}" }
                    }
                }
            }

            // Deployment Section
            div { class: "deployment-section",
                h3 { "Deployment" }
                div { class: "deployment-controls",
                    div { class: "deployment-status",
                        if is_deployed() {
                            span { class: "status-success", "✅ Program deployed successfully" }
                        } else {
                            span { class: "status-pending", "⏳ Ready for deployment" }
                        }
                    }

                    div { class: "deployment-actions",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                // TODO: Implement deployment logic
                                spawn(async move {
                                    // Simulate deployment
                                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                    is_deployed.set(true);
                                });
                            },
                            "🚀 Deploy Program"
                        }

                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                // TODO: Test program
                                tracing::info!("Test program functionality");
                            },
                            "🧪 Test Program"
                        }
                    }
                }
            }

            // Templates Section
            div { class: "templates-section",
                h3 { "Program Templates" }
                div { class: "template-grid",
                    TemplateCard {
                        name: "Hello World",
                        description: "Simple program that returns a greeting",
                        icon: "👋",
                        components: vec!["Store Data".to_string()]
                    }
                    TemplateCard {
                        name: "Counter",
                        description: "Program that maintains a counter",
                        icon: "🔢",
                        components: vec!["Initialize Account".to_string(), "Update Data".to_string()]
                    }
                    TemplateCard {
                        name: "Token Transfer",
                        description: "Program that handles token transfers",
                        icon: "💸",
                        components: vec!["Validate Owner".to_string(), "Check Balance".to_string(), "Update Data".to_string()]
                    }
                    TemplateCard {
                        name: "Voting System",
                        description: "Program for decentralized voting",
                        icon: "🗳️",
                        components: vec!["Create Account".to_string(), "Store Data".to_string(), "Validate Owner".to_string()]
                    }
                }
            }
        }
    }
}

/// Component category for the palette
#[component]
fn ComponentCategory(title: String, components: Vec<(String, String)>) -> Element {
    let selected_components = use_signal(Vec::<String>::new);

    rsx! {
        div { class: "component-category",
            h4 { class: "category-title", "{title}" }
            div { class: "component-list",
                for (name, icon) in components {
                    div {
                        class: "component-item",
                        onclick: move |_| {
                            selected_components.write().push(name.clone());
                        },
                        div { class: "component-icon", "{icon}" }
                        span { class: "component-name", "{name}" }
                    }
                }
            }
        }
    }
}

/// Template card component
#[component]
fn TemplateCard(
    name: String,
    description: String,
    icon: String,
    components: Vec<String>,
) -> Element {
    rsx! {
        div { class: "template-card",
            div { class: "template-header",
                div { class: "template-icon", "{icon}" }
                h4 { class: "template-name", "{name}" }
            }
            p { class: "template-description", "{description}" }
            div { class: "template-components",
                span { "Components: {components.len()}" }
            }
            button {
                class: "btn btn-secondary btn-small",
                onclick: move |_| {
                    // TODO: Load template into canvas
                    tracing::info!("Loading template: {}", name);
                },
                "📥 Use Template"
            }
        }
    }
}

/// Generate code based on selected components
fn generate_code(program_name: &str, components: &[String], generated_code: &mut Signal<String>) {
    let mut code = format!(
        r#"// Generated Solana Program: {program_name}
// This program was created using Surfdesk no-code builder

use solana_program::{{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
}};

// Program entrypoint
entrypoint!(process_instruction);

// Main instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {{
    // Process instruction based on selected components
"#
    );

    // Add component logic
    for component in components {
        match component.as_str() {
            "Create Account" => {
                code.push_str(
                    r#"
    // Create Account logic
    // TODO: Implement account creation
    msg!("Creating account...");
"#,
                );
            }
            "Store Data" => {
                code.push_str(
                    r#"
    // Store Data logic
    // TODO: Implement data storage
    msg!("Storing data...");
"#,
                );
            }
            "Validate Owner" => {
                code.push_str(
                    r#"
    // Validate Owner logic
    // TODO: Implement owner validation
    msg!("Validating owner...");
"#,
                );
            }
            _ => {
                code.push_str(&format!(
                    r#"
    // {} logic
    // TODO: Implement {}
    msg!("Processing {}...");
"#,
                    component,
                    component.to_lowercase(),
                    component.to_lowercase()
                ));
            }
        }
    }

    code.push_str(
        r#"

    Ok(())
}
"#,
    );

    generated_code.set(code);
}
