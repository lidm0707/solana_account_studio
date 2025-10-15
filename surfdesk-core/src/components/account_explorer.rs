//! # Account Explorer Component Module
//!
//! This module provides the account explorer component for browsing and managing
//! Solana accounts in the SurfDesk application.

use dioxus::prelude::*;

/// Account explorer component
#[component]
pub fn AccountExplorer() -> Element {
    rsx! {
        div { class: "account-explorer",
            h2 { "Account Explorer" }
            p { "Account explorer component coming soon..." }
        }
    }
}
