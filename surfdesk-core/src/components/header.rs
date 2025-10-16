//! # Header Component Module
//!
//! This module provides a header component for the SurfDesk application.
//! The header component is built with Dioxus 0.6+ and supports multiple
//! layouts and configurations across all platforms.

use super::combine_classes;
use crate::platform::Platform;
// use crate::types::UIState; // Temporarily unused
use dioxus::prelude::*;

/// Header component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeaderProps {
    /// Current platform
    pub platform: Platform,
    /// Active section
    pub active_section: String,
    /// Sidebar toggle handler (optional)
    pub on_sidebar_toggle: Option<EventHandler<MouseEvent>>,
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
}

/// Navigation item for the header
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    /// Item ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon (emoji or symbol)
    pub icon: String,
    /// Whether item is active
    pub active: bool,
    /// Click handler
    pub on_click: Option<EventHandler<String>>,
}

/// Theme toggle component
#[component]
fn ThemeToggle() -> Element {
    let mut current_theme = use_signal(|| "auto".to_string());

    let on_toggle = move |_| {
        let themes = vec!["auto", "light", "dark"];
        let current_pos = themes
            .iter()
            .position(|&t| t == *current_theme.read())
            .unwrap_or(0);
        let next_pos = (current_pos + 1) % themes.len();
        current_theme.set(themes[next_pos].to_string());
    };

    rsx! {
        button {
            class: "theme-toggle",
            onclick: on_toggle,
            title: "Toggle theme",
            match current_theme.read().as_str() {
                "light" => "☀️",
                "dark" => "🌙",
                _ => "🌓"
            }
        }
    }
}

/// User menu component
#[component]
fn UserMenu() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { class: "user-menu",
            button {
                class: "user-menu-button",
                onclick: move |_| menu_open.set(!*menu_open.read()),
                "👤"
            }

            if *menu_open.read() {
                div { class: "user-menu-dropdown",
                    a { href: "#", "Profile" }
                    a { href: "#", "Settings" }
                    div { class: "divider" }
                    a { href: "#", "Logout" }
                }
            }
        }
    }
}

/// Header component with multi-platform navigation
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let mut classes = vec!["header", "platform-header"];

    // Add platform-specific classes
    match props.platform {
        Platform::Desktop => classes.push("header-desktop"),
        Platform::Web => classes.push("header-web"),
        Platform::Terminal => classes.push("header-terminal"),
    }

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    // Navigation items
    let nav_items = vec![
        NavItem {
            id: "dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            active: props.active_section == "dashboard",
            on_click: None,
        },
        NavItem {
            id: "surfpool".to_string(),
            label: "SurfPool".to_string(),
            icon: "🏊".to_string(),
            active: props.active_section == "surfpool",
            on_click: None,
        },
        NavItem {
            id: "programs".to_string(),
            label: "Programs".to_string(),
            icon: "📦".to_string(),
            active: props.active_section == "programs",
            on_click: None,
        },
        NavItem {
            id: "accounts".to_string(),
            label: "Accounts".to_string(),
            icon: "👥".to_string(),
            active: props.active_section == "accounts",
            on_click: None,
        },
    ];

    rsx! {
        header {
            class: "{class_attr}",
            id: props.id,

            // Desktop/Web layout
            if !matches!(props.platform, Platform::Terminal) {
                div { class: "header-content",

                    // Left side: Logo and navigation
                    div { class: "header-left",

                        // Sidebar toggle button (desktop/tablet)
                        if let Some(toggle_handler) = props.on_sidebar_toggle {
                            button {
                                class: "sidebar-toggle",
                                onclick: move |e| toggle_handler.call(e),
                                "☰"
                            }
                        }

                        // Logo/Brand
                        div { class: "header-brand",
                            h1 { class: "brand-title",
                                span { class: "brand-icon", "🏄" }
                                span { class: "brand-text", "SurfDesk" }
                            }
                        }

                        // Navigation items
                        nav { class: "main-nav",
                            ul { class: "nav-list",
                                {
                                    // Create items with proper lifetime
                                    let items = vec![
                                        rsx! {
                                            li {
                                                key: "dashboard",
                                                class: if props.active_section == "dashboard" { "nav-item active" } else { "nav-item" },

                                                a {
                                                    href: "#",
                                                    span { class: "nav-icon", "📊" }
                                                    span { class: "nav-label", "Dashboard" }
                                                }
                                            }
                                        },
                                        rsx! {
                                            li {
                                                key: "surfpool",
                                                class: if props.active_section == "surfpool" { "nav-item active" } else { "nav-item" },

                                                a {
                                                    href: "#",
                                                    span { class: "nav-icon", "🏊" }
                                                    span { class: "nav-label", "SurfPool" }
                                                }
                                            }
                                        },
                                        rsx! {
                                            li {
                                                key: "accounts",
                                                class: if props.active_section == "accounts" { "nav-item active" } else { "nav-item" },

                                                a {
                                                    href: "#",
                                                    span { class: "nav-icon", "👥" }
                                                    span { class: "nav-label", "Accounts" }
                                                }
                                            }
                                        },
                                    ];
                                    items
                                }
                            }
                        }
                    }

                    // Right side: Actions and user menu
                    div { class: "header-right",

                        // Theme toggle
                        ThemeToggle {}

                        // Notifications
                        button {
                            class: "header-button notification-button",
                            title: "Notifications",
                            "🔔"
                        }

                        // User menu
                        UserMenu {}
                    }
                }
            }

            // Terminal layout (simplified)
            if matches!(props.platform, Platform::Terminal) {
                div { class: "terminal-header",
                    div { class: "terminal-title",
                        span { "🏄 SurfDesk v0.1.0" }
                    }
                    div { class: "terminal-status",
                        span { "Status: Ready" }
                    }
                }
            }
        }
    }
}
