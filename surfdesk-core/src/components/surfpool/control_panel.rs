//! SurfPool Control Panel Component
//!
//! Provides UI controls for managing SurfPool processes including:
//! - Installation detection
//! - Process start/stop
//! - Mainnet fork management

use crate::platform::Platform;
use crate::surfpool::{ControllerStatus, ProcessStatus, SurfPoolController};
use dioxus::prelude::*;

/// SurfPool Control Panel Component
#[component]
pub fn SurfPoolControlPanel() -> Element {
    let mut status = use_signal(|| ControllerStatus::Stopped);
    let mut process_status = use_signal(|| Option::<ProcessStatus>::None);
    let mut error_message = use_signal(String::new);
    let mut success_message = use_signal(String::new);

    // Get platform
    let platform = Platform::current();

    // Check installation
    let check_installation = move |_| {
        let mut error_signal = error_message;
        let mut success_signal = success_message;

        use_coroutine(
            move |_: dioxus::prelude::UnboundedReceiver<()>| async move {
                match SurfPoolController::check_installation().await {
                    Ok(true) => {
                        success_signal.set("SurfPool is installed and ready to use".to_string());
                    }
                    Ok(false) => {
                        error_signal.set(
                            "SurfPool is not installed. Please install it with: cargo install surfpool"
                                .to_string(),
                        );
                    }
                    Err(e) => {
                        error_signal.set(format!("Error checking SurfPool installation: {}", e));
                    }
                }
            },
        );
    };

    // Start SurfPool
    let start_surfpool = move |_| {
        let platform = platform;
        let mut status_signal = status;
        let mut process_status_signal = process_status;
        let mut error_signal = error_message;
        let mut success_signal = success_message;

        use_coroutine(
            move |_: dioxus::prelude::UnboundedReceiver<()>| async move {
                // Clear messages
                error_signal.set(String::new());
                success_signal.set(String::new());

                // Check installation first
                match SurfPoolController::check_installation().await {
                    Ok(true) => {
                        // Create and start controller
                        match SurfPoolController::new(platform).await {
                            Ok(controller) => match controller.start().await {
                                Ok(_) => {
                                    status_signal.set(ControllerStatus::Running);
                                    if let Ok(proc_status) = controller.get_process_status().await {
                                        process_status_signal.set(Some(proc_status));
                                    }
                                    success_signal.set(
                                        "SurfPool started successfully with mainnet fork on port 8999"
                                            .to_string(),
                                    );
                                }
                                Err(e) => {
                                    error_signal.set(format!("Failed to start SurfPool: {}", e));
                                }
                            },
                            Err(e) => {
                                error_signal
                                    .set(format!("Failed to create SurfPool controller: {}", e));
                            }
                        }
                    }
                    Ok(false) => {
                        error_signal
                            .set("SurfPool is not installed. Please install it first".to_string());
                    }
                    Err(e) => {
                        error_signal.set(format!("Error checking SurfPool installation: {}", e));
                    }
                }
            },
        );
    };

    // Stop SurfPool
    let stop_surfpool = move |_| {
        let platform = platform;
        let mut status_signal = status;
        let mut process_status_signal = process_status;
        let mut error_signal = error_message;
        let mut success_signal = success_message;

        use_coroutine(
            move |_: dioxus::prelude::UnboundedReceiver<()>| async move {
                // Clear messages
                error_signal.set(String::new());
                success_signal.set(String::new());

                // Create controller and stop
                match SurfPoolController::new(platform).await {
                    Ok(controller) => match controller.stop_process().await {
                        Ok(_) => {
                            status_signal.set(ControllerStatus::Stopped);
                            process_status_signal.set(None);
                            success_signal.set("SurfPool stopped successfully".to_string());
                        }
                        Err(e) => {
                            error_signal.set(format!("Failed to stop SurfPool: {}", e));
                        }
                    },
                    Err(e) => {
                        error_signal.set(format!("Failed to create SurfPool controller: {}", e));
                    }
                }
            },
        );
    };

    rsx! {
        div { class: "surfpool-control-panel",

            // Header
            div { class: "panel-header",
                h3 { "🏊 SurfPool Control Panel" }
                p { "Manage local Solana validator with mainnet forking" }
            }

            // Status Display
            div { class: "status-section",
                h4 { "Current Status" }
                div { class: "status-display",
                    span { class: "status-label", "Status: " }
                    span {
                        class: match status() {
                            ControllerStatus::Running => "status-running",
                            ControllerStatus::Stopped => "status-stopped",
                            ControllerStatus::Starting => "status-starting",
                            ControllerStatus::Stopping => "status-stopping",
                            ControllerStatus::Error(_) => "status-error",
                        },
                        "{status():?}"
                    }
                }

                if let Some(ref proc_status) = process_status() {
                    div { class: "process-details",
                        div { "RPC Port: {proc_status.rpc_port}" }
                        div { "WebSocket Port: {proc_status.ws_port}" }
                        if let Some(pid) = proc_status.pid {
                            div { "Process ID: {pid}" }
                        }
                    }
                }
            }

            // Control Buttons
            div { class: "control-section",
                h4 { "Controls" }

                div { class: "button-group",
                    button {
                        class: "btn btn-primary",
                        onclick: start_surfpool,
                        disabled: status() == ControllerStatus::Running,
                        "🚀 Start SurfPool"
                    }

                    button {
                        class: "btn btn-danger",
                        onclick: stop_surfpool,
                        disabled: status() == ControllerStatus::Stopped,
                        "⏹️ Stop SurfPool"
                    }

                    button {
                        class: "btn btn-secondary",
                        onclick: check_installation,
                        "🔍 Check Installation"
                    }
                }
            }

            // Messages
            if !error_message().is_empty() {
                div { class: "error-message",
                    "❌ {error_message()}"
                }
            }

            if !success_message().is_empty() {
                div { class: "success-message",
                    "✅ {success_message()}"
                }
            }

            // Information Section
            div { class: "info-section",
                h4 { "SurfPool Information" }
                div { class: "info-grid",
                    div { class: "info-item",
                        strong { "Network: " }
                        "Mainnet Fork"
                    }
                    div { class: "info-item",
                        strong { "RPC Port: " }
                        "8999"
                    }
                    div { class: "info-item",
                        strong { "WebSocket Port: " }
                        "9000"
                    }
                    div { class: "info-item",
                        strong { "Purpose: " }
                        "Local development with mainnet state"
                    }
                }

                div { class: "installation-instructions",
                    h5 { "Installation Instructions:" }
                    pre { "
1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. Install SurfPool: cargo install surfpool
3. Start using the controls above
                    " }
                }
            }
        }
    }
}
