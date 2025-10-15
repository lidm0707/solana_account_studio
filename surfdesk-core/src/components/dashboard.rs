//! # Dashboard Component Module
//!
//! This module provides a dashboard component for the SurfDesk application.
//! The dashboard component displays an overview of projects, environments,
//! accounts, and recent activity.

use super::{combine_classes, Card};
use crate::{state::AppState, types::UIState};
use dioxus::prelude::*;

/// Dashboard component properties
#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
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

/// Dashboard component
#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let mut classes = vec!["dashboard"];

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class.as_str());
    }

    let class_attr = combine_classes(&classes);

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,

            // Dashboard header
            div { class: "dashboard-header",
                h2 { class: "dashboard-title", "Dashboard" }
                p { class: "dashboard-subtitle", "Welcome to SurfDesk" }
            }

            // Stats overview
            div { class: "dashboard-stats",
                Card {
                    variant: crate::components::Variant::Contained,
                    size: crate::components::Size::Medium,
                    color: crate::components::Color::Primary,
                    elevated: true,
                    class: "stats-card",
                    div { class: "stats-content",
                        div { class: "stats-title", "Projects" }
                        div { class: "stats-value", "{props.state.read().projects.read().len()}" }
                    }
                }

                Card {
                    variant: crate::components::Variant::Contained,
                    size: crate::components::Size::Medium,
                    color: crate::components::Color::Secondary,
                    elevated: true,
                    class: "stats-card",
                    div { class: "stats-content",
                        div { class: "stats-title", "Environments" }
                        div { class: "stats-value", "{props.state.read().environments.read().len()}" }
                    }
                }

                Card {
                    variant: crate::components::Variant::Contained,
                    size: crate::components::Size::Medium,
                    color: crate::components::Color::Info,
                    elevated: true,
                    class: "stats-card",
                    div { class: "stats-content",
                        div { class: "stats-title", "Accounts" }
                        div { class: "stats-value", "{props.state.read().accounts.read().len()}" }
                    }
                }

                Card {
                    variant: crate::components::Variant::Contained,
                    size: crate::components::Size::Medium,
                    color: crate::components::Color::Success,
                    elevated: true,
                    class: "stats-card",
                    div { class: "stats-content",
                        div { class: "stats-title", "Transactions" }
                        div { class: "stats-value", "{props.state.read().transactions.read().len()}" }
                    }
                }
            }

            // Recent activity
            div { class: "dashboard-content",
                Card {
                    variant: crate::components::Variant::Default,
                    size: crate::components::Size::Large,
                    elevated: true,
                    class: "activity-card",

                    h3 { class: "card-title", "Recent Activity" }
                    div { class: "activity-list",
                        div { class: "activity-item",
                            span { class: "activity-icon", "📊" }
                            div { class: "activity-content",
                                span { class: "activity-title", "No recent activity" }
                                span { class: "activity-time", "Start working to see activity here" }
                            }
                        }
                    }
                }
            }

            // Quick actions
            div { class: "dashboard-quick-actions",
                Card {
                    variant: crate::components::Variant::Outlined,
                    size: crate::components::Size::Medium,
                    class: "quick-actions-card",

                    h3 { class: "card-title", "Quick Actions" }
                    div { class: "quick-actions-grid",
                        button { class: "quick-action-btn", "New Project" }
                        button { class: "quick-action-btn", "Add Environment" }
                        button { class: "quick-action-btn", "Import Account" }
                        button { class: "quick-action-btn", "Deploy Program" }
                    }
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
    fn test_dashboard_props() {
        // This test would need to be run in a Dioxus context
        // For now, we just verify the props can be created
        let _props = DashboardProps {
            class: Some("custom-dashboard".to_string()),
            id: Some("main-dashboard".to_string()),
            // Note: state and ui_state would need to be created in Dioxus context
        };
    }
}
