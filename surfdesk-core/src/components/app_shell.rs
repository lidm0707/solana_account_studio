//! # App Shell Component Module
//!
//! This module provides the main application shell component that wraps
//! the entire SurfDesk application with navigation, layout, and platform
//! specific adaptations.

use dioxus::prelude::*;

/// App shell component
#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "app-shell",
            header { class: "app-header",
                h1 { "SurfDesk" }
            }
            main { class: "app-main",
                p { "App shell component coming soon..." }
            }
            footer { class: "app-footer",
                p { "© 2024 SurfDesk" }
            }
        }
    }
}
