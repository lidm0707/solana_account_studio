//! Layout Components for Surfdesk
//!
//! This module provides the main layout components that structure the application
//! including the main app shell, navigation, and responsive layout management.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::Route;

/// Main application layout component
#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "app-container",
            Header {}
            div { class: "app-content",
                Sidebar {}
                main { class: "main-content",
                    Outlet::<Route> {}
                }
            }
            Footer {}
        }
    }
}

/// Application header with navigation and branding
#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "header-left",
                div { class: "logo",
                    img {
                        src: "/assets/surfdesk.png",
                        alt: "Surfdesk Logo",
                        class: "logo-image"
                    }
                    h1 { class: "app-title", "Surfdesk" }
                }
            }

            nav { class: "header-nav",
                NavLink { to: Route::Home {}, "Home" }
                NavLink { to: Route::SurfpoolManager {}, "Surfpool" }
                NavLink { to: Route::ProgramBuilder {}, "Programs" }
                NavLink { to: Route::AccountManager {}, "Accounts" }
            }

            div { class: "header-right",
                div { class: "status-indicator",
                    div { class: "status-dot status-online" }
                    span { "Connected" }
                }
            }
        }
    }
}

/// Sidebar navigation with additional navigation options
#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside { class: "sidebar",
            div { class: "sidebar-header",
                h2 { "Navigation" }
            }

            nav { class: "sidebar-nav",
                ul { class: "nav-list",
                    li { class: "nav-item",
                        NavLink { to: Route::Home {},
                            div { class: "nav-link-content",
                                span { class: "nav-icon", "🏠" }
                                span { "Dashboard" }
                            }
                        }
                    }
                    li { class: "nav-item",
                        NavLink { to: Route::SurfpoolManager {},
                            div { class: "nav-link-content",
                                span { class: "nav-icon", "🌊" }
                                span { "Surfpool Manager" }
                            }
                        }
                    }
                    li { class: "nav-item",
                        NavLink { to: Route::ProgramBuilder {},
                            div { class: "nav-link-content",
                                span { class: "nav-icon", "🔧" }
                                span { "Program Builder" }
                            }
                        }
                    }
                    li { class: "nav-item",
                        NavLink { to: Route::AccountManager {},
                            div { class: "nav-link-content",
                                span { class: "nav-icon", "👛" }
                                span { "Account Manager" }
                            }
                        }
                    }
                }
            }

            div { class: "sidebar-footer",
                div { class: "quick-stats",
                    div { class: "stat-item",
                        span { class: "stat-label", "Network" }
                        span { class: "stat-value", "Local" }
                    }
                    div { class: "stat-item",
                        span { class: "stat-label", "Port" }
                        span { class: "stat-value", "8999" }
                    }
                }
            }
        }
    }
}

/// Application footer with information and links
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "app-footer",
            div { class: "footer-content",
                div { class: "footer-left",
                    p { class: "footer-text",
                        "© 2025 Surfdesk - No-Code Solana Development Platform"
                    }
                }

                div { class: "footer-right",
                    span { class: "version-info", "v0.1.0" }
                    a {
                        href: "https://github.com/surfdesk/surfdesk",
                        target: "_blank",
                        class: "footer-link",
                        "GitHub"
                    }
                }
            }
        }
    }
}

/// Navigation link component with active state handling
#[derive(Props, PartialEq, Clone)]
pub struct NavLinkProps {
    pub to: Route,
    pub children: Element,
}

#[component]
pub fn NavLink(props: NavLinkProps) -> Element {
    let navigator = use_navigator();
    let current_route = use_route::<Route>();

    let is_active = current_route == props.to;

    rsx! {
        Link {
            to: props.to,
            class: if is_active { "nav-link active" } else { "nav-link" },
            onclick: move |_| {
                navigator.push(props.to.clone());
            },
            {props.children}
        }
    }
}

/// Responsive container for different screen sizes
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct ContainerProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "default".to_string())]
    pub size: String, // default, small, large, fluid
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let class = format!("container container--{}", props.size);

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Grid system component
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct GridProps {
    #[props(default)]
    pub children: Element,
    #[props(default = 12)]
    pub columns: u32,
    #[props(default = "medium".to_string())]
    pub gap: String, // small, medium, large
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    let class = format!("grid grid--{}-cols grid--gap-{}", props.columns, props.gap);

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Grid column component
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct GridColProps {
    #[props(default)]
    pub children: Element,
    #[props(default = 1)]
    pub span: u32,
}

#[component]
pub fn GridCol(props: GridColProps) -> Element {
    let class = format!("grid-col grid-col--span-{}", props.span);

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Loading overlay component
#[derive(Props, PartialEq, Clone)]
pub struct LoadingOverlayProps {
    #[props(default = false)]
    pub visible: bool,
    #[props(default = "Loading...".to_string())]
    pub message: String,
}

#[component]
pub fn LoadingOverlay(props: LoadingOverlayProps) -> Element {
    rsx! {
        if props.visible {
            div { class: "loading-overlay",
                div { class: "loading-backdrop" }
                div { class: "loading-content",
                    div { class: "loading-spinner" }
                    p { class: "loading-message", "{props.message}" }
                }
            }
        }
    }
}

/// Error display component
#[derive(Props, PartialEq, Clone)]
pub struct ErrorDisplayProps {
    #[props(default)]
    pub error: Option<String>,
    #[props(default = false)]
    pub visible: bool,
    #[props(default)]
    pub on_dismiss: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn ErrorDisplay(props: ErrorDisplayProps) -> Element {
    rsx! {
        if props.visible && props.error.is_some() {
            div { class: "error-display",
                div { class: "error-content",
                    div { class: "error-icon", "⚠️" }
                    div { class: "error-message",
                        {props.error.unwrap_or_default()}
                    }
                    button {
                        class: "error-dismiss",
                        onclick: move |evt| {
                            if let Some(handler) = props.on_dismiss {
                                handler.call(evt);
                            }
                        },
                        "✕"
                    }
                }
            }
        }
    }
}
