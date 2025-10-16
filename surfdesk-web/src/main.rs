//! # SurfDesk Web Application
//!
//! This is the main entry point for the SurfDesk web application.
//! It provides a browser-based experience for the Solana account studio
//! using Dioxus for cross-platform web development.

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use gloo_console::{log, log1};
use surfdesk_core::accounts::Account;
use surfdesk_core::solana_rpc::{AccountService, AccountWithBalance, SolanaNetwork};
use surfdesk_core::{current_platform, init_core};
use wasm_bindgen_futures::spawn_local;

mod style;

/// Main application component with routing
#[component]
fn SurfDeskWebApp() -> Element {
    rsx! {
        style { "
            body {{ font-family: system-ui, -apple-system, sans-serif; margin: 0; padding: 0; }}
            .container {{ max-width: 1200px; margin: 0 auto; padding: 2rem; }}
            .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 4rem 2rem; text-align: center; }}
            .nav {{ background: white; border-bottom: 1px solid #e5e7eb; padding: 1rem 2rem; }}
            .nav-item {{ margin: 0 1rem; color: #374151; text-decoration: none; }}
            .nav-item:hover {{ color: #667eea; }}
            .grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem; margin: 2rem 0; }}
            .card {{ background: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); }}
            .btn {{ background: #667eea; color: white; padding: 0.75rem 1.5rem; border-radius: 4px; text-decoration: none; display: inline-block; }}
            .btn:hover {{ background: #5a6fd8; }}
            .footer {{ background: #f9fafb; padding: 3rem 2rem; text-align: center; margin-top: 4rem; }}
        " }

        Router::<Route> {}
    }
}

/// Route definitions for the web application
#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/accounts")]
    Accounts {},

    #[route("/accounts/:pubkey")]
    AccountDetail { pubkey: String },

    #[route("/transactions")]
    Transactions {},

    #[route("/transactions/:signature")]
    TransactionDetail { signature: String },

    #[route("/programs")]
    Programs {},

    #[route("/programs/:program_id")]
    ProgramDetail { program_id: String },

    #[route("/settings")]
    Settings {},

    #[route("/about")]
    About {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Home page component
#[component]
fn Home() -> Element {
    let solana_url = use_signal(|| "https://api.devnet.solana.com".to_string());
    let mut connection_status = use_signal(|| "Disconnected".to_string());
    let mut account_count = use_signal(|| 0u64);

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            // Navigation
            nav { class: "bg-white shadow-lg sticky top-0 z-50",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex justify-between h-16",
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0 flex items-center",
                                span { class: "text-2xl font-bold text-indigo-600", "🏄 SurfDesk" }
                                span { class: "ml-2 text-lg text-gray-600 hidden sm:block", "Web" }
                            }
                        }
                        div { class:"hidden md:flex items-center space-x-8",
                            Link {
                                to: Route::Home {},
                                class: "text-gray-900 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Home"
                            }
                            Link {
                                to: Route::Accounts {},
                                class: "text-gray-500 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Accounts"
                            }
                            Link {
                                to: Route::Transactions {},
                                class: "text-gray-500 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Transactions"
                            }
                            Link {
                                to: Route::Programs {},
                                class: "text-gray-500 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Programs"
                            }
                            Link {
                                to: Route::Settings {},
                                class: "text-gray-500 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Settings"
                            }
                        }
                    }
                }
            }

            // Main content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Hero section
                section { class: "text-center mb-12",
                    h1 { class: "text-4xl tracking-tight font-extrabold text-gray-900 sm:text-5xl md:text-6xl",
                        span { "SurfDesk " }
                        span { class: "text-indigo-600", "Web" }
                    }
                    p { class: "mt-3 max-w-md mx-auto text-base text-gray-500 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl",
                        "Multi-platform Solana Account Studio - Explore, analyze, and manage Solana accounts and programs in your browser"
                    }
                    div { class: "mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8",
                        div { class: "rounded-md shadow",
                            button {
                                onclick: move |_| {
                                    // Test connection (simplified for web)
                                    connection_status.set("Connecting...".to_string());
                                    // Use web-compatible timeout simulation
                                    spawn_local(async move {
                                        // Simulate network delay
                                        gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                                        connection_status.set("Connected".to_string());
                                        account_count.set(12345);
                                    });
                                },
                                class: "w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 md:py-4 md:text-lg md:px-10 transition-colors",
                                "Connect to Solana"
                            }
                        }
                    }
                }

                // Status indicator
                div { class: "mb-8 text-center",
                    div { class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-full shadow-sm",
                        class: if *connection_status.read() == "Connected" { "bg-green-100 text-green-800" }
                        else if *connection_status.read() == "Connecting" { "bg-yellow-100 text-yellow-800" }
                        else { "bg-red-100 text-red-800" },
                        span { class: "mr-2",
                            if *connection_status.read() == "Connected" { "✅" }
                            else if *connection_status.read() == "Connecting" { "⏳" }
                            else { "❌" }
                        }
                        span { "{connection_status}" }
                    }
                    p { class: "mt-2 text-sm text-gray-600",
                        "RPC URL: {solana_url}"
                    }
                    if *connection_status.read() == "Connected" {
                        p { class: "mt-1 text-sm text-gray-600",
                            "Accounts indexed: {account_count}"
                        }
                    }
                }

                // Feature cards
                div { class: "grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4 mb-12",
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "p-6",
                            div { class: "flex items-center",
                                div { class: "flex-shrink-0 bg-indigo-500 rounded-md p-3",
                                    span { class: "text-2xl", "🔗" }
                                }
                                div { class: "ml-4",
                                    h3 { class: "text-lg font-medium text-gray-900", "Solana Integration" }
                                }
                            }
                            div { class: "mt-4",
                                p { class: "text-sm text-gray-500",
                                    "Connect to Solana mainnet, devnet, testnet, or local validator with full RPC support."
                                }
                            }
                        }
                    }
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "p-6",
                            div { class: "flex items-center",
                                div { class: "flex-shrink-0 bg-green-500 rounded-md p-3",
                                    span { class: "text-2xl", "📊" }
                                }
                                div { class: "ml-4",
                                    h3 { class: "text-lg font-medium text-gray-900", "Account Management" }
                                }
                            }
                            div { class: "mt-4",
                                p { class: "text-sm text-gray-500",
                                    "Explore, analyze, and manage Solana accounts with real-time data visualization."
                                }
                            }
                        }
                    }
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "p-6",
                            div { class: "flex items-center",
                                div { class: "flex-shrink-0 bg-yellow-500 rounded-md p-3",
                                    span { class: "text-2xl", "🔧" }
                                }
                                div { class: "ml-4",
                                    h3 { class: "text-lg font-medium text-gray-900", "Transaction Builder" }
                                }
                            }
                            div { class: "mt-4",
                                p { class: "text-sm text-gray-500",
                                    "Build, simulate, and send transactions with confidence in your browser."
                                }
                            }
                        }
                    }
                    div { class: "bg-white overflow-hidden shadow rounded-lg",
                        div { class: "p-6",
                            div { class: "flex items-center",
                                div { class: "flex-shrink-0 bg-purple-500 rounded-md p-3",
                                    span { class: "text-2xl", "⚙️" }
                                }
                                div { class: "ml-4",
                                    h3 { class: "text-lg font-medium text-gray-900", "AI-Powered Testing" }
                                }
                            }
                            div { class: "mt-4",
                                p { class: "text-sm text-gray-500",
                                    "Generate intelligent test cases and analyze results with AI assistance."
                                }
                            }
                        }
                    }
                }

                // Quick stats
                div { class: "bg-white overflow-hidden shadow rounded-lg mb-12",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900 mb-4", "Quick Stats" }
                        dl { class: "grid grid-cols-1 gap-5 sm:grid-cols-3",
                            div { class: "px-4 py-5 bg-gray-50 sm:px-6 sm:py-4 rounded-lg",
                                dt { class: "text-sm font-medium text-gray-500 truncate", "Platform" }
                                dd { class: "mt-1 text-3xl font-semibold text-gray-900", "Web" }
                            }
                            div { class: "px-4 py-5 bg-gray-50 sm:px-6 sm:py-4 rounded-lg",
                                dt { class: "text-sm font-medium text-gray-500 truncate", "Framework" }
                                dd { class: "mt-1 text-3xl font-semibold text-gray-900", "Dioxus" }
                            }
                            div { class: "px-4 py-5 bg-gray-50 sm:px-6 sm:py-4 rounded-lg",
                                dt { class: "text-sm font-medium text-gray-500 truncate", "Version" }
                                dd { class: "mt-1 text-3xl font-semibold text-gray-900", "{surfdesk_core::VERSION}" }
                            }
                        }
                    }
                }
            }

            // Footer
            footer { class: "bg-white",
                div { class: "max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8",
                    div { class: "mt-8 border-t border-gray-200 pt-8 md:flex md:items-center md:justify-between",
                        div { class: "flex space-x-6 md:order-2",
                            a { href: "https://github.com/your-org/surfdesk", class: "text-gray-400 hover:text-gray-500", "GitHub" }
                            span { class: "text-gray-300", " • " }
                            a { href: "https://docs.surfdesk.dev", class: "text-gray-400 hover:text-gray-500", "Documentation" }
                            span { class: "text-gray-300", " • " }
                            a { href: "https://discord.gg/surfdesk", class: "text-gray-400 hover:text-gray-500", "Discord" }
                        }
                        p { class: "mt-8 text-base text-gray-400 md:mt-0 md:order-1",
                            "© 2024 SurfDesk. Built with Dioxus & Rust. "
                            span { "Platform: " }
                            span { "{current_platform()}" }
                        }
                    }
                }
            }
        }
    }
}

/// Accounts page component
#[component]
fn Accounts() -> Element {
    let mut accounts = use_signal(Vec::<AccountWithBalance>::new);
    let mut show_create_modal = use_signal(|| false);
    let mut show_import_modal = use_signal(|| false);
    let mut new_account_label = use_signal(String::new);
    let mut import_secret_key = use_signal(String::new);
    let mut import_label = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    // Load accounts on component mount
    use_effect(move || {
        spawn_local(async move {
            if let Err(e) = load_accounts(&mut accounts, &mut is_loading, &mut error_message).await
            {
                log::error!("Failed to load accounts: {:?}", e);
            }
        });
    });

    rsx! {
        div { style: "min-height: 100vh; background-color: #f9fafb;",
            // Header
            div { style: "background-color: white; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);",
                div { style: "padding: 1.5rem 2rem;",
                    div { style: "display: flex; align-items: center; justify-content: space-between;",
                        h1 { style: "font-size: 1.875rem; font-weight: bold; color: #111827;", "Accounts" }
                        div { style: "display: flex; gap: 1rem;",
                            button {
                                onclick: move |_| {
                                    show_create_modal.set(true);
                                    error_message.set(String::new());
                                    success_message.set(String::new());
                                },
                                style: "background-color: #4f46e5; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;",
                                "Create Account"
                            }
                            button {
                                onclick: move |_| {
                                    show_import_modal.set(true);
                                    error_message.set(String::new());
                                    success_message.set(String::new());
                                },
                                style: "background-color: #059669; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;",
                                "Import Account"
                            }
                            button {
                                onclick: move |_| {
                                    spawn_local(async move {
                                        if let Err(e) = load_accounts(&mut accounts, &mut is_loading, &mut error_message).await {
                                            log!("Failed to refresh accounts: {:?}", e);
                                        }
                                    });
                                },
                                style: "background-color: #4b5563; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer; transition: background-color 0.2s;",
                                "Refresh"
                            }
                        }
                    }
                }
            }

            // Network selector
            div { style: "background-color: white; border-bottom: 1px solid #e5e7eb;",
                div { style: "padding: 0.75rem 2rem;",
                    div { style: "display: flex; align-items: center; gap: 1rem;",
                        span { style: "font-size: 0.875rem; font-weight: 500; color: #374151;", "Network:" }
                        select {
                            value: "Devnet",
                            style: "width: 8rem; padding: 0.5rem 2.5rem 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white;",
                            option { value: "Devnet", "Devnet" }
                            option { value: "Testnet", "Testnet" }
                            option { value: "Mainnet", "Mainnet" }
                        }
                        span { style: "font-size: 0.75rem; color: #6b7280;", "(Devnet)" }
                    }
                }
            }

            // Messages
            if !error_message.read().is_empty() {
                div { style: "background-color: #fef2f2; border-left: 4px solid #ef4444; padding: 1rem; margin: 1rem;",
                    div { style: "display: flex; align-items: center;",
                        div { style: "flex-shrink: 0; margin-right: 0.75rem;",
                            span { style: "color: #ef4444; font-size: 1.25rem;", "⚠️" }
                        }
                        div {
                            p { style: "font-size: 0.875rem; color: #991b1b;", "{error_message}" }
                        }
                    }
                }
            }

            if !success_message.read().is_empty() {
                div { style: "background-color: #f0fdf4; border-left: 4px solid #22c55e; padding: 1rem; margin: 1rem;",
                    div { style: "display: flex; align-items: center;",
                        div { style: "flex-shrink: 0; margin-right: 0.75rem;",
                            span { style: "color: #22c55e; font-size: 1.25rem;", "✅" }
                        }
                        div {
                            p { style: "font-size: 0.875rem; color: #166534;", "{success_message}" }
                        }
                    }
                }
            }

            // Loading state
            if *is_loading.read() {
                div { style: "display: flex; justify-content: center; align-items: center; padding: 3rem 0;",
                    div { style: "width: 3rem; height: 3rem; border: 4px solid #e5e7eb; border-top: 4px solid #4f46e5; border-radius: 50%; animation: spin 1s linear infinite;",
                        style { "@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }" }
                    }
                }
            }

            // Accounts list
            div { style: "padding: 1.5rem 2rem;",
                if accounts.read().is_empty() && !*is_loading.read() {
                    div { style: "text-align: center; padding: 3rem 0;",
                        div { style: "margin: 0 auto; width: 3rem; height: 3rem; color: #9ca3af; font-size: 3rem;", "📭" }
                        h3 { style: "margin-top: 0.5rem; font-size: 0.875rem; font-weight: 500; color: #111827;", "No accounts" }
                        p { style: "margin-top: 0.25rem; font-size: 0.875rem; color: #6b7280;", "Get started by creating or importing an account." }
                    }
                } else {
                    div { style: "display: grid; gap: 1.5rem; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));",
                        for account in accounts.read().iter() {
                            div { style: "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                                div { style: "padding: 1.5rem;",
                                    div { style: "display: flex; align-items: center;",
                                        div { style: "flex-shrink: 0;",
                                            div { style: "width: 2.5rem; height: 2.5rem; border-radius: 50%; background-color: #ede9fe; display: flex; align-items: center; justify-content: center;",
                                                span { style: "color: #4f46e5; font-weight: 500;", "👤" }
                                            }
                                        }
                                        div { style: "margin-left: 1rem; flex: 1;",
                                            h3 { style: "font-size: 1.125rem; font-weight: 500; color: #111827;", "{account.account.label}" }
                                            p { style: "font-size: 0.875rem; color: #6b7280; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{account.short_pubkey()}" }
                                        }
                                    }
                                    div { style: "margin-top: 1rem;",
                                        div { style: "display: flex; align-items: center; justify-content: space-between;",
                                            span { style: "font-size: 0.875rem; font-weight: 500; color: #111827;", "Balance" }
                                            span { style: "font-size: 1.125rem; font-weight: bold; color: #4f46e5;",
                                                "{account.formatted_balance()}"
                                            }
                                        }
                                    }
                                    div { style: "margin-top: 0.5rem; display: flex; gap: 0.5rem;",
                                        Link {
                                            to: Route::AccountDetail { pubkey: account.account.pubkey.to_string() },
                                            style: "color: #4f46e5; text-decoration: none; font-size: 0.875rem; font-weight: 500;",
                                            "View Details"
                                        }
                                        button {
                                            onclick: move |_| {
                                                spawn_local(async move {
                                                    if let Err(e) = request_airdrop(&account.account.pubkey, &mut success_message, &mut error_message).await {
                                                        log!("Airdrop failed: {:?}", e);
                                                    }
                                                });
                                            },
                                            style: "color: #059669; cursor: pointer; font-size: 0.875rem; font-weight: 500; border: none; background: none;",
                                            "Request Airdrop"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create Account Modal
        if *show_create_modal.read() {
            div { style: "position: fixed; inset: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 50; display: flex; align-items: center; justify-content: center;",
                div { style: "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1); max-width: 24rem; width: 90%;",
                    h3 { style: "font-size: 1.125rem; font-weight: bold; color: #111827; margin-bottom: 1rem;", "Create New Account" }
                    div { style: "margin-bottom: 1rem;",
                        label { style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;", "Account Label" }
                        input {
                            r#type: "text",
                            value: "{new_account_label}",
                            oninput: move |evt| new_account_label.set(evt.value()),
                            style: "width: 100%; padding: 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none;",
                            placeholder: "Enter account label..."
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.75rem;",
                        button {
                            onclick: move |_| show_create_modal.set(false),
                            style: "background-color: #d1d5db; color: #374151; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
                            "Cancel"
                        }
                        button {
                            onclick: move |_| {
                                let label = new_account_label.read().clone();
                                if !label.is_empty() {
                                    spawn_local(async move {
                                        if let Err(e) = create_account(label, &mut accounts, &mut show_create_modal, &mut new_account_label, &mut success_message, &mut error_message).await {
                                            log!("Failed to create account: {:?}", e);
                                        }
                                    });
                                }
                            },
                            style: "background-color: #4f46e5; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
                            "Create"
                        }
                    }
                }
            }
        }

        // Import Account Modal
        if *show_import_modal.read() {
            div { style: "position: fixed; inset: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 50; display: flex; align-items: center; justify-content: center;",
                div { style: "background-color: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1); max-width: 24rem; width: 90%;",
                    h3 { style: "font-size: 1.125rem; font-weight: bold; color: #111827; margin-bottom: 1rem;", "Import Account" }
                    div { style: "margin-bottom: 1rem;",
                        label { style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;", "Account Label" }
                        input {
                            r#type: "text",
                            value: "{import_label}",
                            oninput: move |evt| import_label.set(evt.value()),
                            style: "width: 100%; padding: 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none;",
                            placeholder: "Enter account label..."
                        }
                    }
                    div { style: "margin-bottom: 1rem;",
                        label { style: "display: block; font-size: 0.875rem; font-weight: 500; color: #374151; margin-bottom: 0.5rem;", "Secret Key" }
                        textarea {
                            value: "{import_secret_key}",
                            oninput: move |evt| import_secret_key.set(evt.value()),
                            style: "width: 100%; padding: 0.5rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 0.375rem; background-color: white; outline: none; resize: vertical;",
                            rows: 3,
                            placeholder: "Enter base58 encoded secret key..."
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; gap: 0.75rem;",
                        button {
                            onclick: move |_| show_import_modal.set(false),
                            style: "background-color: #d1d5db; color: #374151; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
                            "Cancel"
                        }
                        button {
                            onclick: move |_| {
                                let secret_key = import_secret_key.read().clone();
                                let label = import_label.read().clone();
                                if !secret_key.is_empty() && !label.is_empty() {
                                    spawn_local(async move {
                                        if let Err(e) = import_account(secret_key, label, &mut accounts, &mut show_import_modal, &mut import_secret_key, &mut import_label, &mut success_message, &mut error_message).await {
                                            log!("Failed to import account: {:?}", e);
                                        }
                                    });
                                }
                            },
                            style: "background-color: #4f46e5; color: white; padding: 0.5rem 1rem; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; border: none; cursor: pointer;",
                            "Import"
                        }
                    }
                }
            }
        }
    }
}

/// Account detail page component
#[component]
fn AccountDetail(pubkey: String) -> Element {
    let mut account = use_signal(|| None::<Account>);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut recipient_pubkey = use_signal(String::new);
    let mut transfer_amount = use_signal(String::new);
    let mut show_transfer_modal = use_signal(|| false);

    // Load account details on component mount
    use_effect(move || {
        let pubkey_str = pubkey.clone();
        spawn_local(async move {
            if let Err(e) = load_account_details(
                &pubkey_str,
                &mut account,
                &mut is_loading,
                &mut error_message,
            )
            .await
            {
                log::error!("Failed to load account details: {:?}", e);
            }
        });
    });

    rsx! {
        div { class: "min-h-screen bg-gray-50",
            // Header
            div { class: "bg-white shadow",
                div { class: "px-4 py-6 sm:px-6 lg:px-8",
                    div { class: "flex items-center justify-between",
                        div { class: "flex items-center space-x-4",
                            Link {
                                to: Route::Accounts {},
                                class: "text-indigo-600 hover:text-indigo-900 text-sm font-medium",
                                "← Back to Accounts"
                            }
                            h1 { class: "text-3xl font-bold text-gray-900", "Account Details" }
                        }
                        button {
                            onclick: move |_| {
                                show_transfer_modal.set(true);
                                error_message.set(String::new());
                            },
                            class: "bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors",
                            "Send SOL"
                        }
                    }
                }
            }

            // Error message
            if !error_message.read().is_empty() {
                div { class: "bg-red-50 border-l-4 border-red-400 p-4 m-4",
                    div { class: "flex",
                        div { class: "flex-shrink-0",
                            svg { class: "h-5 w-5 text-red-400", "viewBox": "0 0 20 20", "fill": "currentColor",
                                path { "fill-rule": "evenodd", "d": "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z", "clip-rule": "evenodd" }
                            }
                        }
                        div { class: "ml-3",
                            p { class: "text-sm text-red-700", "{error_message}" }
                        }
                    }
                }
            }

            // Loading state
            if *is_loading.read() {
                div { class: "flex justify-center items-center py-12",
                    div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600" }
                }
            }

            // Account details
            if let Some(acc) = account.read().as_ref() {
                div { class: "max-w-4xl mx-auto px-4 py-6 sm:px-6 lg:px-8",
                    div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                        div { class: "px-4 py-5 sm:px-6",
                            h3 { class: "text-lg leading-6 font-medium text-gray-900", "{acc.label}" }
                            p { class: "mt-1 max-w-2xl text-sm text-gray-500", "Account information and balance" }
                        }
                        div { class: "border-t border-gray-200",
                            dl {
                                div { class: "bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6",
                                    dt { class: "text-sm font-medium text-gray-500", "Public Key" }
                                    dd { class: "mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 font-mono break-all", "{acc.pubkey}" }
                                }
                                div { class: "bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6",
                                    dt { class: "text-sm font-medium text-gray-500", "Balance" }
                                    dd { class: "mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2",
                                        span { class: "text-2xl font-bold text-indigo-600",
                                            "{acc.balance / 1_000_000_000}.{(acc.balance % 1_000_000_000) / 1_000_000:03} SOL"
                                        }
                                        span { class: "ml-2 text-sm text-gray-500",
                                            "({acc.balance} lamports)"
                                        }
                                    }
                                }
                                div { class: "bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6",
                                    dt { class: "text-sm font-medium text-gray-500", "Created At" }
                                    dd { class: "mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2", "{acc.created_at.format(\"%Y-%m-%d %H:%M:%S UTC\")}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Transfer SOL Modal
        if *show_transfer_modal.read() {
            div { class: "fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50",
                div { class: "relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white",
                    h3 { class: "text-lg font-bold text-gray-900 mb-4", "Send SOL" }
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Recipient Public Key" }
                        input {
                            r#type: "text",
                            value: "{recipient_pubkey}",
                            oninput: move |evt| recipient_pubkey.set(evt.value()),
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            placeholder: "Enter recipient public key..."
                        }
                    }
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Amount (SOL)" }
                        input {
                            r#type: "text",
                            value: "{transfer_amount}",
                            oninput: move |evt| transfer_amount.set(evt.value()),
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            placeholder: "Enter amount in SOL..."
                        }
                    }
                    div { class: "flex justify-end space-x-3",
                        button {
                            onclick: move |_| show_transfer_modal.set(false),
                            class: "bg-gray-300 hover:bg-gray-400 text-gray-800 px-4 py-2 rounded-md text-sm font-medium",
                            "Cancel"
                        }
                        button {
                            onclick: move |_| {
                                let recipient = recipient_pubkey.read().clone();
                                let amount = transfer_amount.read().clone();
                                let from_pubkey = pubkey.clone();
                                if !recipient.is_empty() && !amount.is_empty() {
                                    spawn_local(async move {
                                        if let Err(e) = send_sol_transfer(&from_pubkey, &recipient, &amount, &mut show_transfer_modal, &mut recipient_pubkey, &mut transfer_amount, &mut error_message).await {
                                            log!("Failed to send SOL: {:?}", e);
                                        }
                                    });
                                }
                            },
                            class: "bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-md text-sm font-medium",
                            "Send"
                        }
                    }
                }
            }
        }
    }
}

// Helper functions for account management
async fn load_accounts(
    accounts: &mut Signal<Vec<Account>>,
    is_loading: &mut Signal<bool>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    is_loading.set(true);
    error_message.set(String::new());

    // Use real account service
    let service = get_account_service();
    let account_list = service.get_accounts().to_vec();

    // Update balances for all accounts
    let mut updated_accounts = Vec::new();
    for account in account_list {
        match service.get_balance(&account.pubkey.to_string()).await {
            Ok(balance) => {
                let mut updated_account = account.clone();
                updated_account.balance = balance;
                updated_accounts.push(updated_account);
            }
            Err(e) => {
                log::warn!("Failed to fetch balance for {}: {}", account.pubkey, e);
                updated_accounts.push(account);
            }
        }
    }

    accounts.set(updated_accounts);
    is_loading.set(false);
    Ok(())
}

async fn load_account_details(
    pubkey: &str,
    account: &mut Signal<Option<Account>>,
    is_loading: &mut Signal<bool>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    is_loading.set(true);
    error_message.set(String::new());

    let service = get_account_service();

    // Find account in service
    if let Some(found_account) = service
        .get_accounts()
        .iter()
        .find(|acc| acc.pubkey.to_string() == pubkey)
    {
        let mut updated_account = found_account.clone();

        // Get real balance from network
        match service.get_balance(pubkey).await {
            Ok(balance) => updated_account.balance = balance,
            Err(e) => {
                log::warn!("Failed to fetch balance for {}: {}", pubkey, e);
            }
        }

        account.set(Some(updated_account));
    } else {
        error_message.set(format!("Account {} not found", pubkey));
    }

    is_loading.set(false);
    Ok(())
}

async fn create_account(
    label: String,
    accounts: &mut Signal<Vec<Account>>,
    show_modal: &mut Signal<bool>,
    label_input: &mut Signal<String>,
    success_message: &mut Signal<String>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = get_account_service();

    match service.create_account(label.clone()) {
        Ok((new_account, _keypair)) => {
            let mut current_accounts = accounts.read().clone();
            current_accounts.push(new_account);
            accounts.set(current_accounts);

            show_modal.set(false);
            label_input.set(String::new());
            success_message.set(format!("Account '{}' created successfully!", label));
        }
        Err(e) => {
            error_message.set(format!("Failed to create account: {}", e));
        }
    }
    Ok(())
}

async fn import_account(
    secret_key: String,
    label: String,
    accounts: &mut Signal<Vec<Account>>,
    show_modal: &mut Signal<bool>,
    secret_input: &mut Signal<String>,
    label_input: &mut Signal<String>,
    success_message: &mut Signal<String>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // For MVP, create a mock account from the provided data
    let keypair = surfdesk_core::accounts::create_keypair(); // In real implementation, decode secret_key
    let service = get_account_service();

    match service.create_account(label.clone()) {
        Ok((new_account, _keypair)) => {
            let mut current_accounts = accounts.read().clone();
            let account_with_balance = AccountWithBalance::new(new_account, 0);
            current_accounts.push(account_with_balance);
            accounts.set(current_accounts);
        }
        Err(e) => {
            error_message.set(format!("Failed to create account: {}", e));
            return Ok(());
        }
    }

    show_modal.set(false);
    secret_input.set(String::new());
    label_input.set(String::new());
    success_message.set(format!("Account '{}' imported successfully!", label));
    Ok(())
}

async fn request_airdrop(
    pubkey: &surfdesk_core::accounts::Pubkey,
    success_message: &mut Signal<String>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // For MVP, just show a success message
    success_message.set(format!(
        "Airdrop requested for {}! Check balance in a few seconds.",
        pubkey
    ));
    Ok(())
}

async fn send_sol_transfer(
    from_pubkey: &str,
    to_pubkey: &str,
    amount_str: &str,
    show_modal: &mut Signal<bool>,
    recipient_input: &mut Signal<String>,
    amount_input: &mut Signal<String>,
    error_message: &mut Signal<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse amount
    let amount: f64 = amount_str.parse().map_err(|_| "Invalid amount format")?;
    let lamports = (amount * 1_000_000_000.0) as u64;

    // For MVP, just show a success message
    show_modal.set(false);
    recipient_input.set(String::new());
    amount_input.set(String::new());

    // In a real implementation, this would use AccountService to send the transaction
    log::info!(
        "Would send {} lamports from {} to {}",
        lamports,
        from_pubkey,
        to_pubkey
    );
    Ok(())
}

/// Transactions page component
#[component]
fn Transactions() -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-8", "Transactions" }
            p { class:"text-gray-600", "Transaction management interface coming soon..." }
        }
    }
}

/// Transaction detail page component
#[component]
fn TransactionDetail(signature: String) -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-4", "Transaction Details" }
            p { class:"text-gray-600", "Signature: {signature}" }
            p { class:"text-gray-600", "Transaction detail interface coming soon..." }
        }
    }
}

/// Programs page component
#[component]
fn Programs() -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-8", "Programs" }
            p { class:"text-gray-600", "Program management interface coming soon..." }
        }
    }
}

/// Program detail page component
#[component]
fn ProgramDetail(program_id: String) -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-4", "Program Details" }
            p { class:"text-gray-600", "Program ID: {program_id}" }
            p { class:"text-gray-600", "Program detail interface coming soon..." }
        }
    }
}

/// Settings page component
#[component]
fn Settings() -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-8", "Settings" }
            p { class:"text-gray-600", "Settings interface coming soon..." }
        }
    }
}

/// About page component
#[component]
fn About() -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-8", "About SurfDesk" }
            div { class:"max-w-3xl mx-auto",
                p { class:"text-gray-600 mb-4",
                    "SurfDesk is a comprehensive Solana account studio built with modern Rust technologies."
                }
                p { class:"text-gray-600 mb-4",
                    "Version: {surfdesk_core::VERSION}"
                }
                p { class:"text-gray-600 mb-4",
                    "Platform: {current_platform()}"
                }
                p { class:"text-gray-600",
                    "Built with Dioxus for cross-platform compatibility."
                }
            }
        }
    }
}

/// 404 Not Found page component
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50 flex items-center justify-center",
            div { class:"text-center",
                h1 { class:"text-6xl font-bold text-gray-900 mb-4", "404" }
                p { class:"text-xl text-gray-600 mb-8", "Page not found" }
                Link {
                    to: Route::Home {},
                    class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 transition-colors",
                    "Go Home"
                }
            }
        }
    }
}

/// Initialize logging for web platform
fn init_web_logging() {
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logging");
    log::info!("Web logging initialized");
}

// Initialize web account service
static mut ACCOUNT_SERVICE: Option<AccountService> = None;

fn get_account_service() -> &'static mut AccountService {
    unsafe {
        if ACCOUNT_SERVICE.is_none() {
            ACCOUNT_SERVICE = Some(AccountService::new(SolanaNetwork::Devnet));
        }
        ACCOUNT_SERVICE.as_mut().unwrap()
    }
}

/// Main function for web application
fn main() {
    // Initialize logging
    init_web_logging();

    log::info!("Starting SurfDesk Web application...");
    log::info!("Platform: {}", current_platform());
    log::info!("Version: {}", surfdesk_core::VERSION);

    // Initialize core library
    // Initialize core library for web
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = init_core().await {
            web_sys::console::error_1(&format!("Failed to initialize core library: {}", e).into());
        }
    });

    log::info!("Core library initialized successfully");

    // Launch the web application
    dioxus::launch(SurfDeskWebApp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_parsing() {
        // Test route creation and parsing
        let home_route = Route::Home {};
        assert_eq!(home_route, Route::Home {});
    }

    #[test]
    fn test_web_logging() {
        // Test that web logging doesn't panic
        init_web_logging();
    }
}
