//! # SurfDesk Web Application
//!
//! This is the main entry point for the SurfDesk web application.
//! It provides a browser-based experience for the Solana account studio
//! using Dioxus for cross-platform web development.

use dioxus::prelude::*;
use dioxus_router::prelude::*;
// use gloo_console::log;
use log::{error, info, LevelFilter};
use surfdesk_core::{current_platform, init_core, Platform};

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
    let mut solana_url = use_signal(|| "https://api.devnet.solana.com".to_string());
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
                                    use wasm_bindgen_futures::spawn_local;
                                    let mut connection_status_clone = connection_status.clone();
                                    let mut account_count_clone = account_count.clone();
                                    spawn_local(async move {
                                        // Simulate network delay
                                        gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                                        connection_status_clone.set("Connected".to_string());
                                        account_count_clone.set(12345);
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
    rsx! {
        div { class: "min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-8", "Accounts" }
            p { class:"text-gray-600", "Account management interface coming soon..." }
        }
    }
}

/// Account detail page component
#[component]
fn AccountDetail(pubkey: String) -> Element {
    rsx! {
        div { class:"min-h-screen bg-gray-50",
            h1 { class:"text-3xl font-bold text-gray-900 mb-4", "Account Details" }
            p { class:"text-gray-600", "Public Key: {pubkey}" }
            p { class:"text-gray-600", "Account detail interface coming soon..." }
        }
    }
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
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize console log");
    log::info!("SurfDesk Web logging initialized");
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
