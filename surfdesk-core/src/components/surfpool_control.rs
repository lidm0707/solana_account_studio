//! # SurfPool Control Component
//!
//! This component provides a basic UI for controlling SurfPool validators
//! across all platforms (desktop, web, and terminal).

use crate::platform::Platform;
use crate::state::AppState;
use crate::surfpool::{use_surfpool_controller, ControllerStatus};
use dioxus::prelude::*;

/// Props for the SurfPool control component
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SurfPoolControlProps {
    /// Application state
    pub state: Signal<AppState>,
    /// Current platform
    pub platform: Platform,
    /// Optional callback when status changes
    pub on_status_change: Option<EventHandler<ControllerStatus>>,
}

/// Main SurfPool control component
#[component]
pub fn SurfPoolControl(props: SurfPoolControlProps) -> Element {
    let controller = use_surfpool_controller(props.platform.clone());
    let status = use_signal(|| ControllerStatus::Stopped);
    let error_message = use_signal(|| Option::<String>::None);

    // Manual status checking (simplified approach)
    let check_status = move |_| {
        let controller = controller.read().clone();
        let mut status = status.clone();

        spawn(async move {
            match controller.get_status().await {
                new_status => {
                    status.set(new_status.clone());

                    // Call the callback if provided
                    if let Some(handler) = &props.on_status_change {
                        handler.call(new_status);
                    }
                }
            }
        });
    };

    let on_start = move |_| {
        let controller = controller.read().clone();
        let mut status = status.clone();
        let mut error_message = error_message.clone();

        spawn(async move {
            match controller.start().await {
                Ok(()) => {
                    status.set(ControllerStatus::Running);
                    error_message.set(None);
                }
                Err(e) => {
                    status.set(ControllerStatus::Error(e.to_string()));
                    error_message.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_stop = move |_| {
        let controller = controller.read().clone();
        let mut status = status.clone();
        let mut error_message = error_message.clone();

        spawn(async move {
            match controller.stop().await {
                Ok(()) => {
                    status.set(ControllerStatus::Stopped);
                    error_message.set(None);
                }
                Err(e) => {
                    status.set(ControllerStatus::Error(e.to_string()));
                    error_message.set(Some(e.to_string()));
                }
            }
        });
    };

    let current_status = status.read().clone();

    rsx! {
        div { class: "surfpool-control p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg",

            // Header
            div { class: "flex items-center justify-between mb-6",
                h2 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                    "SurfPool Validator Control"
                }

                div { class: "flex items-center space-x-2",
                    StatusIndicator { status: current_status.clone() }
                }
            }

            // Error message
            if let Some(ref error) = *error_message.read() {
                div { class: "mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                    div { class: "flex items-center",
                        span { class: "text-red-600 dark:text-red-400 font-medium",
                            "Error: {error}"
                        }
                    }
                }
            }

            // Control buttons
            ControlButtons {
                status: current_status.clone(),
                on_start,
                on_stop,
                on_check: check_status,
            }
        }
    }
}

/// Status indicator component
#[component]
fn StatusIndicator(status: ControllerStatus) -> Element {
    let (color, text, pulsing) = match &status {
        ControllerStatus::Stopped => ("gray", "Stopped", false),
        ControllerStatus::Starting => ("yellow", "Starting...", true),
        ControllerStatus::Running => ("green", "Running", false),
        ControllerStatus::Stopping => ("orange", "Stopping...", true),
        ControllerStatus::Error(_) => ("red", "Error", true),
    };

    rsx! {
        div { class: "flex items-center space-x-2",
            div {
                class: if pulsing {
                    format!("w-3 h-3 bg-{}-500 rounded-full animate-pulse", color)
                } else {
                    format!("w-3 h-3 bg-{}-500 rounded-full", color)
                }
            }
            span { class: "text-sm font-medium text-gray-600 dark:text-gray-300",
                "{text}"
            }
        }
    }
}

/// Control buttons component
#[component]
fn ControlButtons(
    status: ControllerStatus,
    on_start: EventHandler<MouseEvent>,
    on_stop: EventHandler<MouseEvent>,
    on_check: EventHandler<MouseEvent>,
) -> Element {
    let is_running = matches!(status, ControllerStatus::Running);
    let is_transitioning = matches!(
        status,
        ControllerStatus::Starting | ControllerStatus::Stopping
    );

    rsx! {
        div { class: "flex flex-wrap gap-3 mb-6",

            // Start button
            button {
                class: "px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors",
                disabled: is_running || is_transitioning,
                onclick: on_start,
                "Start Validator"
            }

            // Stop button
            button {
                class: "px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors",
                disabled: !is_running || is_transitioning,
                onclick: on_stop,
                "Stop Validator"
            }

            // Check Status button
            button {
                class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors",
                onclick: on_check,
                "Check Status"
            }
        }
    }
}
