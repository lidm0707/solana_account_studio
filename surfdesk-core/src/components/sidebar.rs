//! # Sidebar Component Module
//!
//! This module provides a sidebar component for the SurfDesk application.
//! The sidebar supports adaptive layouts for different platforms and screen sizes,
//! with collapsible functionality and responsive behavior.

use crate::components::app_shell::Breakpoint;
use crate::components::combine_classes;
use crate::platform::Platform;
use dioxus::prelude::*;

/// Sidebar component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SidebarProps {
    /// Current platform
    pub platform: Platform,
    /// Whether sidebar is collapsed
    pub collapsed: bool,
    /// Currently active section
    pub active_section: String,
    /// Section change handler
    pub on_section_change: Option<EventHandler<String>>,
    /// Current breakpoint for responsive behavior
    pub current_breakpoint: Breakpoint,
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
}

/// Sidebar navigation item
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    /// Item ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon (emoji or symbol)
    pub icon: String,
    /// Whether item is active
    pub active: bool,
    /// Whether item has sub-items
    pub has_children: bool,
    /// Sub-items (if any)
    pub children: Vec<SidebarItem>,
    /// Badge count (optional)
    pub badge: Option<u32>,
}

/// Sidebar navigation section
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarSection {
    /// Section title
    pub title: String,
    /// Section items
    pub items: Vec<SidebarItem>,
    /// Whether section is collapsible
    pub collapsible: bool,
    /// Whether section is collapsed
    pub collapsed: bool,
    /// Whether to always show items (even when collapsed)
    pub always_show: bool,
}

impl Default for SidebarSection {
    fn default() -> Self {
        Self {
            title: String::new(),
            items: Vec::new(),
            collapsible: true,
            collapsed: false,
            always_show: false,
        }
    }
}

/// Sidebar component with cross-platform navigation
#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let mut classes = vec!["sidebar"];

    // Add platform-specific classes
    match props.platform {
        Platform::Desktop => classes.push("sidebar-desktop"),
        Platform::Web => classes.push("sidebar-web"),
        Platform::Terminal => classes.push("sidebar-terminal"),
    }

    // Add state classes
    if props.collapsed {
        classes.push("sidebar-collapsed");
    }

    // Add responsive classes
    match props.current_breakpoint {
        Breakpoint::Mobile => classes.push("sidebar-mobile"),
        Breakpoint::Tablet => classes.push("sidebar-tablet"),
        Breakpoint::Desktop => classes.push("sidebar-desktop"),
    }

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    // Define sidebar sections
    let sections = &[
        SidebarSection {
            title: "Main".to_string(),
            items: vec![
                SidebarItem {
                    id: "dashboard".to_string(),
                    label: "Dashboard".to_string(),
                    icon: "📊".to_string(),
                    active: props.active_section == "dashboard",
                    has_children: false,
                    children: vec![],
                    badge: None,
                },
                SidebarItem {
                    id: "surfpool".to_string(),
                    label: "SurfPool".to_string(),
                    icon: "🏊".to_string(),
                    active: props.active_section == "surfpool",
                    has_children: false,
                    children: vec![],
                    badge: None,
                },
            ],
            collapsible: false,
            collapsed: false,
            always_show: false,
        },
        SidebarSection {
            title: "Development".to_string(),
            items: vec![
                SidebarItem {
                    id: "programs".to_string(),
                    label: "Programs".to_string(),
                    icon: "📦".to_string(),
                    active: props.active_section == "programs",
                    has_children: false,
                    children: vec![],
                    badge: Some(3), // Example badge count
                },
                SidebarItem {
                    id: "accounts".to_string(),
                    label: "Accounts".to_string(),
                    icon: "👥".to_string(),
                    active: props.active_section == "accounts",
                    has_children: false,
                    children: vec![],
                    badge: None,
                },
                SidebarItem {
                    id: "transactions".to_string(),
                    label: "Transactions".to_string(),
                    icon: "📝".to_string(),
                    active: props.active_section == "transactions",
                    has_children: false,
                    children: vec![],
                    badge: None,
                },
            ],
            collapsible: true,
            collapsed: false,
            always_show: false,
        },
        SidebarSection {
            title: "Tools".to_string(),
            items: vec![
                SidebarItem {
                    id: "ai-assistant".to_string(),
                    label: "AI Assistant".to_string(),
                    icon: "🤖".to_string(),
                    active: props.active_section == "ai-assistant",
                    has_children: false,
                    children: vec![],
                    badge: None,
                },
                SidebarItem {
                    id: "network".to_string(),
                    label: "Network".to_string(),
                    icon: "🌐".to_string(),
                    active: props.active_section == "network",
                    has_children: true,
                    children: vec![
                        SidebarItem {
                            id: "network-mainnet".to_string(),
                            label: "Mainnet".to_string(),
                            icon: "🔴".to_string(),
                            active: false,
                            has_children: false,
                            children: vec![],
                            badge: None,
                        },
                        SidebarItem {
                            id: "network-devnet".to_string(),
                            label: "Devnet".to_string(),
                            icon: "🔵".to_string(),
                            active: false,
                            has_children: false,
                            children: vec![],
                            badge: None,
                        },
                        SidebarItem {
                            id: "network-testnet".to_string(),
                            label: "Testnet".to_string(),
                            icon: "🟡".to_string(),
                            active: false,
                            has_children: false,
                            children: vec![],
                            badge: None,
                        },
                    ],
                    badge: None,
                },
            ],
            collapsible: true,
            collapsed: false,
            always_show: false,
        },
    ];

    let on_item_click = move |item_id: String| {
        if let Some(handler) = &props.on_section_change {
            handler.call(item_id);
        }
    };

    // Extract sidebar sections iterables
    let desktop_sections = sections.iter();
    let terminal_sections = sections.iter();

    let toggle_icon = if props.collapsed { '▶' } else { '◀' };

    rsx! {
        aside {
            class: "{class_attr}",
            id: props.id,

            // Desktop/Web layout
            if !matches!(props.platform, Platform::Terminal) {
                div { class: "sidebar-content",

                    // Sidebar header
                    div { class: "sidebar-header",
                        if !props.collapsed {
                            h2 { class: "sidebar-title", "Navigation" }
                        }
                        button {
                            class: "sidebar-collapse-toggle",
                            onclick: move |_: dioxus::events::MouseEvent| {
                                if let Some(handler) = &props.on_section_change {
                                    handler.call("toggle-sidebar".to_string());
                                }
                            },
                            "{toggle_icon}"
                        }
                    }

                    // Navigation sections
                    nav { class: "sidebar-nav",
                        for section in desktop_sections {
                            SidebarSectionComponent {
                                section: section.clone(),
                                collapsed: props.collapsed,
                                on_item_click: on_item_click,
                            }
                        }
                    }

                    // Sidebar footer (only when expanded)
                    if !props.collapsed {
                        div { class: "sidebar-footer",
                            div { class: "status-indicator",
                                span { class: "status-dot status-online" }
                                span { "Connected" }
                            }
                        }
                    }
                }
            }

            // Terminal layout (simplified)
            if matches!(props.platform, Platform::Terminal) {
                div { class: "terminal-sidebar",
                    div { class: "terminal-nav",
                        for section in terminal_sections {
                            TerminalSidebarSection {
                                section: section.clone(),
                                active_section: props.active_section.clone(),
                                on_item_click: on_item_click,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Sidebar section component
#[component]
fn SidebarSectionComponent(
    section: SidebarSection,
    collapsed: bool,
    on_item_click: EventHandler<String>,
) -> Element {
    let section_collapsed = use_signal(|| section.collapsed);
    let mut section_collapsed_clone = section_collapsed;

    let on_section_toggle = move |_: dioxus::events::MouseEvent| {
        let current = *section_collapsed_clone.read();
        section_collapsed_clone.set(!current);
    };

    // Extract header rendering
    let section_header = if !collapsed {
        let toggle_icon = if *section_collapsed.read() {
            '▶'
        } else {
            '▼'
        };

        rsx! {
            div { class: "section-header",
                if section.collapsible {
                    button {
                        class: "section-toggle",
                        onclick: on_section_toggle,
                        "{toggle_icon}"
                    }
                }
                h3 { class: "section-title", "{section.title}" }
            }
        }
    } else {
        rsx! {}
    };

    // Extract section items iterator
    let section_items = section.items.iter();

    rsx! {
        div { class: "sidebar-section",
            {section_header}

            // Section items
            div { class: "section-items",
                // Hide items if section is collapsed (and not sidebar collapsed)
                if !collapsed && (!section.collapsible || !*section_collapsed.read()) {
                    for item in section_items {
                        SidebarItemComponent {
                            item: item.clone(),
                            collapsed: collapsed,
                            on_click: on_item_click,
                        }
                    }
                }
            }
        }
    }
}

/// Sidebar item component
#[component]
fn SidebarItemComponent(
    item: SidebarItem,
    collapsed: bool,
    on_click: EventHandler<String>,
) -> Element {
    let mut item_expanded = use_signal(|| false);

    let on_item_click_handler = move |_: dioxus::events::MouseEvent| {
        on_click.call(item.id.clone());
    };

    let on_expand_toggle = move |_: dioxus::events::MouseEvent| {
        let current = *item_expanded.read();
        item_expanded.set(!current);
    };

    // Extract children items iterator
    let child_items = item.children.iter();

    let expand_icon = if *item_expanded.read() { '▼' } else { '▶' };

    rsx! {
        div { class: if item.active { "sidebar-item active" } else { "sidebar-item" },

            // Main item
            div { class: "item-content",
                onclick: on_item_click_handler,

                // Item icon
                span { class: "item-icon", "{item.icon}" }

                // Item label (hidden when collapsed)
                if !collapsed {
                    span { class: "item-label", "{item.label}" }
                }

                // Badge (if any)
                if !collapsed {
                    if let Some(badge_count) = item.badge {
                        span { class: "item-badge", "{badge_count}" }
                    }
                }

                // Expand indicator (if has children)
                if !collapsed && item.has_children {
                    button {
                        class: "item-expand",
                        onclick: on_expand_toggle,
                        "{expand_icon}"
                    }
                }
            }

            // Sub-items (if expanded)
            if !collapsed && item.has_children && *item_expanded.read() {
                div { class: "item-children",
                    for child in child_items {
                        SidebarItemComponent {
                            item: child.clone(),
                            collapsed: collapsed,
                            on_click: on_click,
                        }
                    }
                }
            }
        }
    }
}

/// Terminal sidebar section component
#[component]
fn TerminalSidebarSection(
    section: SidebarSection,
    active_section: String,
    on_item_click: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "terminal-section",
            div { class: "terminal-section-header",
                "── {section.title} ──"
            }
            // Terminal items rendering
            for item in section.items.iter() {
                TerminalSidebarItem {
                    item: item.clone(),
                    active_section: active_section.clone(),
                    on_click: on_item_click,
                }
            }
        }
    }
}

/// Terminal sidebar item component
#[component]
fn TerminalSidebarItem(
    item: SidebarItem,
    active_section: String,
    on_click: EventHandler<String>,
) -> Element {
    let on_item_click_handler = move |_: dioxus::events::MouseEvent| {
        on_click.call(item.id.clone());
    };

    rsx! {
        div {
            class: if item.active { "terminal-item active" } else { "terminal-item" },
            onclick: on_item_click_handler,

            span { " {item.icon} {item.label}" }

            if let Some(badge_count) = item.badge {
                span { class: "terminal-badge", " [{badge_count}]" }
            }
        }
    }
}
