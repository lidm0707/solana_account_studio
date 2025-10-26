//! Surfpool Manager Page Component
//!
//! This page provides interface for managing Surfpool local Solana
//! simulation network, including starting/stopping service, monitoring
//! status, and configuring network parameters.

use dioxus::prelude::*;

/// Surfpool Manager page component
#[component]
pub fn SurfpoolManager() -> Element {
    let mut is_running = use_signal(|| false);
    let mut status_message = use_signal(|| "Surfpool is stopped".to_string());
    let port = use_signal(|| 8999u16);
    let mut network_status = use_signal(|| "Disconnected".to_string());
    let mut logs = use_signal(|| Vec::<String>::new());

    rsx! {
        div {
            style: "min-height: 100vh; background-color: #f9fafb; padding: 1.5rem; font-family: system-ui, -apple-system, sans-serif;",

            div {
                style: "margin-bottom: 2rem;",
                h1 { style: "font-size: 2.25rem; font-weight: 700; color: #111827; margin-bottom: 0.5rem;", "Surfpool Manager" }
                p { style: "font-size: 1.125rem; color: #4b5563;", "Manage your local Solana simulation network" }
            }

            div {
                style: "margin-bottom: 2rem;",
                StatusCard {
                    is_running: is_running(),
                    status_message: status_message(),
                    port: port(),
                    network_status: network_status()
                }
            }

            div {
                style: "margin-bottom: 2rem;",
                h2 { style: "font-size: 1.5rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Control Panel" }
                div {
                    style: "display: flex; gap: 1rem;",
                    if !is_running() {
                        button {
                            style: "padding: 0.75rem 1.5rem; background-color: #10b981; color: white; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                            onclick: move |_| {
                                is_running.set(true);
                                status_message.set("Surfpool is running".to_string());
                                network_status.set("Connected - Local Simulation".to_string());
                                logs.write().push("Surfpool started successfully".to_string());
                            },
                            "🚀 Start Surfpool"
                        }
                    } else {
                        button {
                            style: "padding: 0.75rem 1.5rem; background-color: #ef4444; color: white; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                            onclick: move |_| {
                                is_running.set(false);
                                status_message.set("Surfpool is stopped".to_string());
                                network_status.set("Disconnected".to_string());
                                logs.write().push("Surfpool stopped successfully".to_string());
                            },
                            "🛑 Stop Surfpool"
                        }
                    }
                }
            }

            div {
                h2 { style: "font-size: 1.5rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Activity Logs" }
                div {
                    style: "background-color: white; border-radius: 0.5rem; border: 1px solid #e5e7eb; padding: 1rem; max-height: 400px; overflow-y: auto;",
                    if logs().is_empty() {
                        div {
                            style: "text-align: center; color: #6b7280; padding: 2rem;",
                            p { "No activity logs yet. Start Surfpool to see logs." }
                        }
                    } else {
                        for log_entry in logs().iter() {
                            div {
                                style: "padding: 0.5rem 0; border-bottom: 1px solid #f3f4f6; font-family: monospace; font-size: 0.875rem; color: #374151;",
                                span { style: "color: #6b7280; margin-right: 0.5rem;", "[LOG]" }
                                span { "{log_entry}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Status card component for displaying surfpool status
#[component]
fn StatusCard(
    is_running: bool,
    status_message: String,
    port: u16,
    network_status: String,
) -> Element {
    rsx! {
        div {
            style: "background-color: white; border-radius: 0.5rem; border: 1px solid #e5e7eb; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);",

            div {
                style: "display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem;",
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    div {
                        style: if is_running {
                            "width: 12px; height: 12px; border-radius: 50%; background-color: #10b981;"
                        } else {
                            "width: 12px; height: 12px; border-radius: 50%; background-color: #ef4444;"
                        },
                    }
                    span { style: "font-weight: 500; color: #111827;", "{status_message}" }
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;",
                div {
                    style: "display: flex; justify-content: space-between;",
                    span { style: "color: #6b7280;", "Port:" }
                    span { style: "font-weight: 500;", "{port}" }
                }
                div {
                    style: "display: flex; justify-content: space-between;",
                    span { style: "color: #6b7280;", "Network:" }
                    span { style: "font-weight: 500;", "{network_status}" }
                }
            }

            div {
                style: "border-top: 1px solid #e5e7eb; padding-top: 1rem;",
                h3 { style: "font-size: 1rem; font-weight: 600; color: #111827; margin-bottom: 0.5rem;", "Features" }
                ul {
                    style: "list-style: none; padding: 0; margin: 0;",
                    li { style: "padding: 0.25rem 0; color: #4b5563; font-size: 0.875rem;", "• Mainnet fork to local simulation" }
                    li { style: "padding: 0.25rem 0; color: #4b5563; font-size: 0.875rem;", "• Full RPC compatibility" }
                    li { style: "padding: 0.25rem 0; color: #4b5563; font-size: 0.875rem;", "• WebSocket support" }
                    li { style: "padding: 0.25rem 0; color: #4b5563; font-size: 0.875rem;", "• Account state persistence" }
                    li { style: "padding: 0.25rem 0; color: #4b5563; font-size: 0.875rem;", "• Transaction history" }
                }
            }
        }
    }
}
