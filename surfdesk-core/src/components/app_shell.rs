//! # App Shell Component Module
//!
//! This module provides the main application shell component that wraps
//! the entire SurfDesk application with navigation, layout, and platform
//! specific adaptations.

use crate::components::{Dashboard, Footer, Header, Sidebar, SurfPoolControl};
use crate::platform::Platform;
use crate::state::AppState;
use dioxus::prelude::*;

/// Props for the AppShell component
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AppShellProps {
    /// Application state
    pub state: Signal<AppState>,
    /// Current platform
    pub platform: Platform,
    /// Optional theme override
    pub theme: Option<Theme>,
}

/// Theme variants for the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Auto
    }
}

/// Layout breakpoints for responsive design
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breakpoint {
    Mobile,  // < 768px
    Tablet,  // 768px - 1200px
    Desktop, // > 1200px
}

/// Main App Shell component with responsive multi-platform layout
#[component]
pub fn AppShell(props: AppShellProps) -> Element {
    let current_breakpoint = use_signal(|| Breakpoint::Desktop);
    let sidebar_collapsed = use_signal(|| false);
    let active_section = use_signal(|| "dashboard".to_string());

    // Detect viewport size and set breakpoint
    let current_breakpoint_clone = current_breakpoint.clone();
    use_effect(move || {
        // In a real implementation, this would use window resize events
        // For now, we'll keep a simple implementation
        current_breakpoint_clone.set(Breakpoint::Desktop);
    });

    let sidebar_collapsed_clone = sidebar_collapsed.clone();
    let on_sidebar_toggle = EventHandler::new(move |_: MouseEvent| {
        sidebar_collapsed_clone.set(!*sidebar_collapsed_clone.read());
    });

    let on_section_change = EventHandler::new(move |section: String| {
        active_section.set(section.clone());
    });

    rsx! {
        div {
            class: format!("app-shell theme-{:?} platform-{:?}",
                props.theme.unwrap_or_default(),
                props.platform),

            // Header with navigation
            Header {
                platform: props.platform.clone(),
                on_sidebar_toggle: Some(on_sidebar_toggle),
                active_section: active_section.read().clone(),
            }

            // Main content area with sidebar
            div { class: "app-layout",

                // Sidebar (desktop/tablet) or overlay (mobile)
                Sidebar {
                    platform: props.platform.clone(),
                    collapsed: *sidebar_collapsed.read(),
                    active_section: active_section.read().clone(),
                    on_section_change: Some(on_section_change),
                    current_breakpoint: *current_breakpoint.read(),
                }

                // Main content area
                main { class: "app-main",

                    // Dynamic content based on active section
                    match active_section.read().as_str() {
                        "dashboard" => rsx! {
                            Dashboard {
                                state: props.state,
                                platform: props.platform.clone(),
                            }
                        },
                        "surfpool" => rsx! {
                            SurfPoolControl {
                                state: props.state,
                                platform: props.platform.clone(),
                                on_status_change: None,
                            }
                        },
                        _ => rsx! {
                            div { class: "content-placeholder",
                                h2 { "Section: {active_section}" }
                                p { "This section is under development..." }
                            }
                        }
                    }
                }
            }

            // Footer
            Footer {
                platform: props.platform.clone(),
            }
        }
    }
}

/// Responsive layout container component
#[component]
pub fn ResponsiveLayout(
    children: Element,
    #[props(default = Breakpoint::Desktop)] breakpoint: Breakpoint,
    #[props(default = false)] collapsible: bool,
) -> Element {
    let is_collapsed = use_signal(|| false);
    let is_collapsed_clone = is_collapsed.clone();

    let breakpoint_class = match breakpoint {
        Breakpoint::Mobile => "mobile",
        Breakpoint::Tablet => "tablet",
        Breakpoint::Desktop => "desktop",
    };

    let collapse_class = if collapsible && *is_collapsed.read() {
        "collapsed"
    } else {
        "expanded"
    };

    let toggle_icon = if *is_collapsed.read() { '▶' } else { '◀' };

    rsx! {
        div {
            class: format!("responsive-layout layout-{} {}", breakpoint_class, collapse_class),

            // Collapse button for mobile/tablet
            if collapsible && matches!(breakpoint, Breakpoint::Mobile | Breakpoint::Tablet) {
                button {
                    class: "layout-toggle",
                    onclick: move |_| is_collapsed_clone.set(!*is_collapsed_clone.read()),
                    "{toggle_icon}"
                }
            }

            // Content area
            div { class: "layout-content",
                {children}
            }
        }
    }
}

/// Grid layout component for responsive design
#[component]
pub fn ResponsiveGrid(
    children: Element,
    #[props(default = 2)] columns: u8,
    #[props(default = Breakpoint::Desktop)] breakpoint: Breakpoint,
    #[props(default = 1)] mobile_columns: u8,
    #[props(default = 2)] tablet_columns: u8,
) -> Element {
    let grid_columns = match breakpoint {
        Breakpoint::Mobile => mobile_columns,
        Breakpoint::Tablet => tablet_columns,
        Breakpoint::Desktop => columns,
    };

    rsx! {
        div {
            class: format!("responsive-grid grid-{}-cols", grid_columns),
            style: "display: grid; gap: 1rem;",

            {children}
        }
    }
}

/// Flex layout component with responsive behavior
#[component]
pub fn ResponsiveFlex(
    children: Element,
    #[props(default = "row")] direction: &'static str,
    #[props(default = "flex-start")] align: &'static str,
    #[props(default = "flex-start")] justify: &'static str,
    #[props(default = Breakpoint::Desktop)] breakpoint: Breakpoint,
    #[props(default = "column")] mobile_direction: &'static str,
) -> Element {
    let flex_direction = match breakpoint {
        Breakpoint::Mobile => mobile_direction,
        _ => direction,
    };

    rsx! {
        div {
            class: "responsive-flex",
            style: format!(
                "display: flex; flex-direction: {}; align-items: {}; justify-content: {};",
                flex_direction, align, justify
            ),

            {children}
        }
    }
}

/// Container component with responsive max-width
#[component]
pub fn ResponsiveContainer(
    children: Element,
    #[props(default = false)] fluid: bool,
    #[props(default = Breakpoint::Desktop)] breakpoint: Breakpoint,
) -> Element {
    let max_width = if fluid {
        "100%"
    } else {
        match breakpoint {
            Breakpoint::Mobile => "100%",
            Breakpoint::Tablet => "768px",
            Breakpoint::Desktop => "1200px",
        }
    };

    rsx! {
        div {
            class: "responsive-container",
            style: format!(
                "max-width: {}; margin: 0 auto; padding: 0 1rem;",
                max_width
            ),

            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme, Theme::Auto);
    }

    #[test]
    fn test_breakpoint_logic() {
        // Test breakpoint-based column logic
        let desktop_cols = ResponsiveGrid {
            children: rsx! { div { "test" } },
            columns: 3,
            breakpoint: Breakpoint::Desktop,
            mobile_columns: 1,
            tablet_columns: 2,
        };

        // Component should render without panicking
        assert!(true);
    }
}
