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
    pub state: dioxus::prelude::Signal<AppState>,
    /// UI state
    pub ui_state: dioxus::prelude::Signal<UIState>,
}

/// Header component
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut classes = vec!["header"];

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class.as_str());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use crate::types::UIState;

    #[test]
    fn test_header_props() {
        // This test would need to be run in a Dioxus context
        // For now, we just verify the props can be created
        let _props = HeaderProps {
            class: Some("custom-header".to_string()),
            id: Some("main-header".to_string()),
            // Note: state and ui_state would need to be created in Dioxus context
        };
    }
}
