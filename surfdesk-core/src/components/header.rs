//! # Header Component Module
//!
//! This module provides a header component for the SurfDesk application.
//! The header component is built with Dioxus 0.6+ and supports multiple
//! layouts and configurations across all platforms.

use super::combine_classes;
use crate::{state::AppState, types::UIState};
use dioxus::prelude::*;

/// Header component properties
#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Application state
    pub state: Signal<AppState>,
    /// UI state
    pub ui_state: Signal<UIState>,
}

/// Header component
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut classes = vec!["header"];

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    rsx! {
        header {
            class: "{class_attr}",
            id: props.id,

            div { class: "header-content",
                div { class: "header-title",
                    h1 { "SurfDesk" }
                }

                div { class: "header-actions",
                    // User menu or actions would go here
                    button { class: "header-button", "⚙️" }
                }
            }
        }
    }
}
