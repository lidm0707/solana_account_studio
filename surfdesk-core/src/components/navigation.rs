//! Navigation Components for Surfdesk
//!
//! This module provides navigation-related components including breadcrumbs,
//! navigation menus, tabs, and other navigation UI elements.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// Breadcrumb navigation component
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbProps {
    #[props(default)]
    pub items: Vec<BreadcrumbItem>,
    #[props(default)]
    pub separator: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub active: bool,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let separator = props.separator.unwrap_or(">".to_string());

    rsx! {
        nav { class: "breadcrumb",
            ol { class: "breadcrumb-list",
                for (index, item) in props.items.iter().enumerate() {
                    li {
                        key: "{index}",
                        class: "breadcrumb-item {if item.active { 'active' } else { '' }}",

                        if let Some(href) = &item.href {
                            Link {
                                to: href.clone(),
                                class: "breadcrumb-link",
                                "{item.label}"
                            }
                        } else {
                            span { class: "breadcrumb-text", "{item.label}" }
                        }

                        if index < props.items.len() - 1 {
                            span { class: "breadcrumb-separator", "{separator}" }
                        }
                    }
                }
            }
        }
    }
}

/// Tab navigation component
#[derive(Props, PartialEq, Clone)]
pub struct TabsProps {
    #[props(default)]
    pub tabs: Vec<TabItem>,
    #[props(default = 0)]
    pub active_tab: usize,
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
    #[props(default = "horizontal".to_string())]
    pub orientation: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub disabled: bool,
    pub badge: Option<String>,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    rsx! {
        div { class: "tabs tabs--{props.orientation}",
            div { class: "tabs-list",
                for (index, tab) in props.tabs.iter().enumerate() {
                    button {
                        key: "{tab.id}",
                        class: "tabs-trigger {if index == props.active_tab { 'active' } else { '' }} {if tab.disabled { 'disabled' } else { '' }}",
                        disabled: tab.disabled,
                        onclick: move |_| {
                            if let Some(handler) = props.on_change {
                                handler.call(index);
                            }
                        },

                        if let Some(icon) = &tab.icon {
                            span { class: "tabs-icon", "{icon}" }
                        }

                        span { class: "tabs-label", "{tab.label}" }

                        if let Some(badge) = &tab.badge {
                            span { class: "tabs-badge", "{badge}" }
                        }
                    }
                }
            }
        }
    }
}

/// Sidebar navigation component
#[derive(Props, PartialEq, Clone)]
pub struct SidebarNavProps {
    #[props(default)]
    pub items: Vec<NavItem>,
    #[props(default)]
    pub active_item: Option<String>,
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    #[props(default = false)]
    pub collapsible: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub href: Option<String>,
    pub children: Vec<NavItem>,
    pub active: bool,
    pub badge: Option<String>,
}

#[component]
pub fn SidebarNav(props: SidebarNavProps) -> Element {
    rsx! {
        nav { class: "sidebar-nav",
            ul { class: "nav-list",
                for item in &props.items {
                    NavItemComponent {
                        item: item.clone(),
                        active_id: props.active_item.clone(),
                        on_select: props.on_select,
                        level: 0,
                    }
                }
            }
        }
    }
}

#[component]
fn NavItemComponent(
    item: NavItem,
    active_id: Option<String>,
    on_select: Option<EventHandler<String>>,
    level: u32,
) -> Element {
    let is_active = active_id
        .as_ref()
        .map(|id| id == &item.id)
        .unwrap_or(item.active);
    let has_children = !item.children.is_empty();
    let is_expanded = use_signal(|| is_active);

    rsx! {
        li { class: "nav-item nav-item--level-{level} {if is_active { 'active' } else { '' }}",
            div { class: "nav-item-content",
                if let Some(href) = &item.href {
                    Link {
                        to: href.clone(),
                        class: "nav-link {if is_active { 'active' } else { '' }}",
                        onclick: move |_| {
                            if let Some(handler) = on_select {
                                handler.call(item.id.clone());
                            }
                        },

                        if let Some(icon) = &item.icon {
                            span { class: "nav-icon", "{icon}" }
                        }

                        span { class: "nav-label", "{item.label}" }

                        if let Some(badge) = &item.badge {
                            span { class: "nav-badge", "{badge}" }
                        }
                    }
                } else {
                    button {
                        class: "nav-button {if is_active { 'active' } else { '' }}",
                        onclick: move |_| {
                            if has_children && props.collapsible {
                                is_expanded.set(!is_expanded());
                            }
                            if let Some(handler) = on_select {
                                handler.call(item.id.clone());
                            }
                        },

                        if let Some(icon) = &item.icon {
                            span { class: "nav-icon", "{icon}" }
                        }

                        span { class: "nav-label", "{item.label}" }

                        if let Some(badge) = &item.badge {
                            span { class: "nav-badge", "{badge}" }
                        }

                        if has_children {
                            span { class: "nav-arrow {if is_expanded() { 'expanded' } else { '' }}", "▼" }
                        }
                    }
                }
            }

            if has_children && is_expanded() {
                ul { class: "nav-children",
                    for child in &item.children {
                        NavItemComponent {
                            item: child.clone(),
                            active_id: active_id.clone(),
                            on_select: on_select,
                            level: level + 1,
                        }
                    }
                }
            }
        }
    }
}

/// Pagination component
#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    pub current_page: u32,
    pub total_pages: u32,
    #[props(default)]
    pub on_page_change: Option<EventHandler<u32>>,
    #[props(default = 5)]
    pub visible_pages: u32,
    #[props(default = false)]
    pub show_first_last: bool,
    #[props(default = false)]
    pub show_prev_next: bool,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let page_numbers =
        generate_page_numbers(props.current_page, props.total_pages, props.visible_pages);

    rsx! {
        nav { class: "pagination",
            ul { class: "pagination-list",
                // First page
                if props.show_first_last && props.current_page > 1 {
                    li { class: "pagination-item",
                        button {
                            class: "pagination-link",
                            onclick: move |_| {
                                if let Some(handler) = props.on_page_change {
                                    handler.call(1);
                                }
                            },
                            "First"
                        }
                    }
                }

                // Previous page
                if props.show_prev_next && props.current_page > 1 {
                    li { class: "pagination-item",
                        button {
                            class: "pagination-link",
                            onclick: move |_| {
                                if let Some(handler) = props.on_page_change {
                                    handler.call(props.current_page - 1);
                                }
                            },
                            "Previous"
                        }
                    }
                }

                // Page numbers
                for page in page_numbers {
                    li { class: "pagination-item",
                        button {
                            key: "{page}",
                            class: "pagination-link {if page == props.current_page { 'active' } else { '' }}",
                            onclick: move |_| {
                                if let Some(handler) = props.on_page_change {
                                    handler.call(page);
                                }
                            },
                            "{page}"
                        }
                    }
                }

                // Next page
                if props.show_prev_next && props.current_page < props.total_pages {
                    li { class: "pagination-item",
                        button {
                            class: "pagination-link",
                            onclick: move |_| {
                                if let Some(handler) = props.on_page_change {
                                    handler.call(props.current_page + 1);
                                }
                            },
                            "Next"
                        }
                    }
                }

                // Last page
                if props.show_first_last && props.current_page < props.total_pages {
                    li { class: "pagination-item",
                        button {
                            class: "pagination-link",
                            onclick: move |_| {
                                if let Some(handler) = props.on_page_change {
                                    handler.call(props.total_pages);
                                }
                            },
                            "Last"
                        }
                    }
                }
            }
        }
    }
}

/// Generate page numbers with ellipsis for pagination
fn generate_page_numbers(current: u32, total: u32, visible: u32) -> Vec<u32> {
    if total <= visible {
        return (1..=total).collect();
    }

    let mut pages = Vec::new();
    let half_visible = visible / 2;

    // Always include first page
    pages.push(1);

    // Calculate range around current page
    let start = if current <= half_visible + 1 {
        2
    } else {
        current - half_visible
    };

    let end = if current + half_visible >= total - 1 {
        total - 1
    } else {
        current + half_visible
    };

    // Add ellipsis if needed
    if start > 2 {
        pages.push(0); // Use 0 as ellipsis marker
    }

    // Add middle pages
    for page in start..=end {
        pages.push(page);
    }

    // Add ellipsis if needed
    if end < total - 1 {
        pages.push(0); // Use 0 as ellipsis marker
    }

    // Always include last page
    if total > 1 {
        pages.push(total);
    }

    pages.retain(|&page| page != 0 || page == 0); // Keep ellipsis markers
    pages.into_iter().filter(|&page| page != 0).collect()
}

/// Menu component for dropdown menus
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    #[props(default)]
    pub items: Vec<MenuItem>,
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    #[props(default = "bottom-left".to_string())]
    pub position: String,
    #[props(default = false)]
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub disabled: bool,
    pub separator: bool,
    pub children: Vec<MenuItem>,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    rsx! {
        if props.visible {
            div { class: "menu menu--{props.position}",
                ul { class: "menu-list",
                    for item in &props.items {
                        MenuItemComponent {
                            item: item.clone(),
                            on_select: props.on_select,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenuItemComponent(item: MenuItem, on_select: Option<EventHandler<String>>) -> Element {
    rsx! {
        if item.separator {
            li { class: "menu-separator" }
        } else {
            li { class: "menu-item {if item.disabled { 'disabled' } else { '' }}",
                button {
                    class: "menu-button",
                    disabled: item.disabled,
                    onclick: move |_| {
                        if !item.disabled {
                            if let Some(handler) = on_select {
                                handler.call(item.id.clone());
                            }
                        }
                    },

                    if let Some(icon) = &item.icon {
                        span { class: "menu-icon", "{icon}" }
                    }

                    span { class: "menu-label", "{item.label}" }
                }
            }
        }
    }
}

/// Step navigation component for multi-step processes
#[derive(Props, PartialEq, Clone)]
pub struct StepNavProps {
    pub steps: Vec<StepNavItem>,
    #[props(default = 0)]
    pub current_step: usize,
    #[props(default)]
    pub on_step_change: Option<EventHandler<usize>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StepNavItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub error: bool,
}

#[component]
pub fn StepNav(props: StepNavProps) -> Element {
    rsx! {
        nav { class: "step-nav",
            ol { class: "step-nav-list",
                for (index, step) in props.steps.iter().enumerate() {
                    li {
                        key: "{step.id}",
                        class: "step-nav-item {if index == props.current_step { 'active' } else { '' }} {if step.completed { 'completed' } else { '' }} {if step.error { 'error' } else { '' }}",

                        button {
                            class: "step-nav-button",
                            onclick: move |_| {
                                if let Some(handler) = props.on_step_change {
                                    handler.call(index);
                                }
                            },

                            div { class: "step-nav-indicator",
                                if step.completed {
                                    "✓"
                                } else {
                                    "{index + 1}"
                                }
                            }

                            div { class: "step-nav-content",
                                h3 { class: "step-nav-title", "{step.title}" }
                                if let Some(description) = &step.description {
                                    p { class: "step-nav-description", "{description}" }
                                }
                            }
                        }

                        if index < props.steps.len() - 1 {
                            div { class: "step-nav-connector" }
                        }
                    }
                }
            }
        }
    }
}
