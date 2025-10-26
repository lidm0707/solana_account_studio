//! Home Page Component
//!
//! This is the main dashboard page that provides an overview of the Surfdesk
//! application, showing system status, quick actions, and recent activity.
use crate::routes::Route;
use crate::services::surfpool::{Surfpool, TraitSurfpool};
use dioxus::prelude::*;
use dioxus_router::components::Link;
use dioxus_router::prelude::*;

/// Home page component - the main dashboard
#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let surfpool_status = use_signal(|| "Stopped".to_string());
    let network_info = use_signal(|| "Local Simulation".to_string());
    let mut surfpool = use_signal(|| Surfpool::new());
    let logs = use_signal(Vec::<String>::new);

    let toggle_surfpool = move |_| {
        let mut surfpool = surfpool.clone();
        let mut surfpool_status = surfpool_status.clone();
        let mut logs = logs.clone();

        spawn(async move {
            if surfpool_status() == "Running" {
                // Stop the surfpool
                match surfpool.write().stop().await {
                    Ok(_) => {
                        logs.write().insert(0, "🟥 Surfpool stopped".to_string());
                        surfpool_status.set("Stopped".into());
                    }
                    Err(e) => {
                        logs.write()
                            .insert(0, format!("Error stopping surfpool: {e}"));
                    }
                }
            } else {
                // Start the surfpool
                match surfpool.write().start().await {
                    Ok(_) => {
                        logs.write().insert(0, "🟩 Surfpool started".to_string());
                        surfpool_status.set("Running".into());
                    }
                    Err(e) => {
                        logs.write()
                            .insert(0, format!("Error starting surfpool: {e}"));
                        return;
                    }
                }

                // Monitor the surfpool with better signal management
                let mut last_output = String::new();
                loop {
                    // Check if surfpool is still running
                    let is_running = {
                        let surfpool_guard = surfpool.read();
                        surfpool_guard.is_running().await
                    };

                    if !is_running {
                        logs.write().insert(0, "🏁 Surfpool finished".to_string());
                        surfpool_status.set("Stopped".into());
                        break;
                    }

                    // Try to read output
                    match surfpool.write().read().await {
                        Ok(output) => {
                            if !output.is_empty() && output != last_output {
                                logs.write().insert(0, output.clone());
                                last_output = output;
                            }
                        }
                        Err(_) => {
                            // Just continue on read error, don't log every error to avoid spam
                        }
                    }

                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        });
    };

    rsx! {
        div {
            style: "min-height: 100vh; background-color: #f9fafb; padding: 1.5rem; font-family: system-ui, -apple-system, sans-serif;",

            div {
                style: "margin-bottom: 2rem;",
                h1 { style: "font-size: 2.25rem; font-weight: 700; color: #111827; margin-bottom: 0.5rem;", "Dashboard" }
                p { style: "font-size: 1.125rem; color: #4b5563;", "Welcome to Surfdesk - Your No-Code Solana Development Platform" }
                div {
                    style: "margin-top: 1rem;",
                    Link {
                        to: Route::ProgramBuilderPage {},
                        style: "display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background-color: #2563eb; color: white; text-decoration: none; border-radius: 0.5rem; font-weight: 500;",
                        span { style: "font-size: 1.25rem;", "🔧" }
                        span { "Open Program Builder" }
                    }
                }
            }

            // Status Cards
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;",

                StatusCard {
                    title: "Surfpool Status".to_string(),
                    value: surfpool_status(),
                    icon: "🌊".to_string(),
                    status: if surfpool_status() == "Running" { "success".to_string() } else { "warning".to_string() }
                }
                StatusCard {
                    title: "Network".to_string(),
                    value: network_info(),
                    icon: "🌐".to_string(),
                    status: "info".to_string()
                }
                StatusCard {
                    title: "Programs".to_string(),
                    value: "3".to_string(),
                    icon: "🔧".to_string(),
                    status: "success".to_string()
                }
                StatusCard {
                    title: "Accounts".to_string(),
                    value: "5".to_string(),
                    icon: "👛".to_string(),
                    status: "success".to_string()
                }
            }

            // Quick Actions
            div {
                style: "margin-bottom: 2rem;",
                h2 { style: "font-size: 1.5rem; font-weight: 700; color: #111827; margin-bottom: 1.5rem;", "Quick Actions" }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;",

                    QuickActionCard {
                        title: if surfpool_status() == "Running" { "Stop Surfpool".to_string() } else { "Start Surfpool".to_string() },
                        description: "Toggle Surfpool process".to_string(),
                        icon: if surfpool_status() == "Running" { "🟥".to_string() } else { "🟩".to_string() },
                        onclick: toggle_surfpool
                    }
                    // QuickActionCard {
                    //     title: "Create Program".to_string(),
                    //     description: "Build a new Solana program visually".to_string(),
                    //     icon: "➕".to_string(),
                    //     onclick: |_|{}
                    // }
                    // QuickActionCard {
                    //     title: "Create Account".to_string(),
                    //     description: "Generate a new Solana account".to_string(),
                    //     icon: "👤".to_string(),
                    //     action: "create_account".to_string()
                    // }
                    // QuickActionCard {
                    //     title: "Deploy Program".to_string(),
                    //     description: "Deploy program to local network".to_string(),
                    //     icon: "📤".to_string(),
                    //     action: "deploy_program".to_string()
                    // }
                }
            }

            // Recent Activity
            div {
                style: "margin-bottom: 2rem;",
                h2 { style: "font-size: 1.5rem; font-weight: 700; color: #111827; margin-bottom: 1.5rem;", "Recent Activity" }
                div {
                    style: "display: flex; flex-direction: column; gap: 1rem;",

                    for (index, activity) in logs().iter().enumerate() {
                        div {
                            key: "{index}",
                            style: "background-color: white; border-radius: 0.5rem; box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06); padding: 1rem; display: flex; align-items: center;",

                            div { style: "font-size: 1.5rem; margin-right: 1rem;", "📋" }
                            div {
                                style: "flex: 1;",
                                p { style: "color: #111827; font-weight: 500; margin: 0;", "{activity}" }
                                span { style: "color: #6b7280; font-size: 0.875rem;", "2 minutes ago" }
                            }
                        }
                    }
                }
            }

            // Getting Started
            div {
                style: "margin-bottom: 2rem;",
                h2 { style: "font-size: 1.5rem; font-weight: 700; color: #111827; margin-bottom: 1.5rem;", "Getting Started" }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;",

                    GettingStartedCard {
                        step: 1,
                        title: "Start Surfpool".to_string(),
                        description: "Initialize your local Solana development environment by starting the Surfpool simulation network.".to_string()
                    }
                    GettingStartedCard {
                        step: 2,
                        title: "Create Account".to_string(),
                        description: "Generate your first Solana account to interact with the blockchain.".to_string()
                    }
                    GettingStartedCard {
                        step: 3,
                        title: "Build Program".to_string(),
                        description: "Use the visual program builder to create your first Solana program without writing code.".to_string()
                    }
                    GettingStartedCard {
                        step: 4,
                        title: "Deploy & Test".to_string(),
                        description: "Deploy your program to the local network and test its functionality.".to_string()
                    }
                }
            }
        }
    }
}

/// Status card component for displaying system status
#[component]
fn StatusCard(
    title: String,
    value: String,
    icon: String,
    status: String, // success, warning, error, info
) -> Element {
    let border_color = match status.as_str() {
        "success" => "#10b981",
        "warning" => "#f59e0b",
        "error" => "#ef4444",
        "info" => "#3b82f6",
        _ => "#6b7280",
    };

    rsx! {
        div {
            style: "background-color: white; border-radius: 0.5rem; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06); padding: 1.5rem; border-left: 4px solid {border_color};",

            div {
                style: "display: flex; align-items: center; margin-bottom: 1rem;",
                span { style: "font-size: 1.5rem; margin-right: 0.75rem;", "{icon}" }
                h3 { style: "font-size: 1.125rem; font-weight: 600; color: #374151; margin: 0;", "{title}" }
            }
            div {
                style: "font-size: 1.5rem; font-weight: 700; color: #111827;",
                "{value}"
            }
        }
    }
}

/// Quick action card for common operations
#[component]
fn QuickActionCard(
    title: String,
    description: String,
    icon: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            style: "background-color: white; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1); padding: 1.5rem; text-align: center;",
            div { style: "font-size: 2rem; margin-bottom: 0.5rem;", "{icon}" }
            h3 { style: "font-size: 1.25rem; font-weight: 600;", "{title}" }
            p { style: "font-size: 0.875rem; color: #555;", "{description}" }
            button {
                style: "margin-top: 0.5rem; background-color: #2563eb; color: white; padding: 0.5rem 1rem; border-radius: 0.25rem; cursor: pointer;",
                onclick: move |evt| onclick.call(evt),
                "{title}"
            }
        }
    }
}

/// Getting started card for onboarding
#[component]
fn GettingStartedCard(step: u32, title: String, description: String) -> Element {
    rsx! {
        div {
            style: "background-color: white; border-radius: 0.5rem; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06); padding: 1.5rem;",

            div {
                style: "width: 2.5rem; height: 2.5rem; background-color: #2563eb; color: white; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 700; margin-bottom: 1rem;",
                "{step}"
            }
            div {
                h3 { style: "font-size: 1.125rem; font-weight: 600; color: #111827; margin: 0 0 0.5rem 0;", "{title}" }
                p { style: "color: #6b7280; font-size: 0.875rem; margin: 0;", "{description}" }
            }
        }
    }
}
