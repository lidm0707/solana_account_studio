//! Integration tests for navigation system
//! Tests cross-platform navigation behavior and state management

use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::{AppShell, Header, Sidebar};
use surfdesk_core::platform::Platform;
use surfdesk_core::state::AppState;

#[test]
fn test_navigation_desktop_integration() {
    let mut app_state = use_signal(|| AppState::default());
    app_state.write().active_section = "dashboard".to_string();

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: None,
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify navigation elements are present
    assert!(rendered.contains("main-nav"));
    assert!(rendered.contains("sidebar-nav"));
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("SurfPool"));
    assert!(rendered.contains("Accounts"));
}

#[test]
fn test_navigation_web_integration() {
    let mut app_state = use_signal(|| AppState::default());
    app_state.write().active_section = "surfpool".to_string();

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Web,
            theme: None,
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify web-specific navigation
    assert!(rendered.contains("platform-web"));
    assert!(rendered.contains("main-nav"));
    assert!(rendered.contains("responsive-layout"));
}

#[test]
fn test_navigation_terminal_integration() {
    let mut app_state = use_signal(|| AppState::default());
    app_state.write().active_section = "accounts".to_string();

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Terminal,
            theme: None,
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify terminal-specific navigation
    assert!(rendered.contains("platform-terminal"));
    assert!(rendered.contains("terminal-header"));
    assert!(rendered.contains("terminal-sidebar"));
}

#[test]
fn test_navigation_active_state() {
    let mut app_state = use_signal(|| AppState::default());

    // Test Dashboard active state
    app_state.write().active_section = "dashboard".to_string();
    let dashboard_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: None,
        }
    };
    let dashboard_rendered = dioxus_ssr::render_element(dashboard_shell);
    assert!(dashboard_rendered.contains("nav-item active"));

    // Test SurfPool active state
    app_state.write().active_section = "surfpool".to_string();
    let surfpool_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: None,
        }
    };
    let surfpool_rendered = dioxus_ssr::render_element(surfpool_shell);
    assert!(surfpool_rendered.contains("nav-item active"));
}

#[test]
fn test_navigation_sidebar_toggle() {
    let app_state = use_signal(|| AppState::default());

    let header_component = rsx! {
        Header {
            platform: Platform::Desktop,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    // Verify sidebar toggle button is present on desktop
    assert!(rendered.contains("sidebar-toggle"));
}

#[test]
fn test_navigation_responsive_behavior() {
    // Test mobile navigation (should be different)
    let mobile_shell = rsx! {
        AppShell {
            state: use_signal(|| AppState::default()),
            platform: Platform::Web,
            theme: None,
        }
    };
    let mobile_rendered = dioxus_ssr::render_element(mobile_shell);
    assert!(mobile_rendered.contains("responsive-layout"));
    assert!(mobile_rendered.contains("platform-web"));

    // Test desktop navigation
    let desktop_shell = rsx! {
        AppShell {
            state: use_signal(|| AppState::default()),
            platform: Platform::Desktop,
            theme: None,
        }
    };
    let desktop_rendered = dioxus_ssr::render_element(desktop_shell);
    assert!(desktop_rendered.contains("platform-desktop"));
    assert!(rendered.contains("sidebar-nav"));
}

#[test]
fn test_navigation_hierarchy() {
    let app_state = use_signal(|| AppState::default());

    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "accounts".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    // Verify hierarchical structure
    assert!(rendered.contains("nav-sections"));
    assert!(rendered.contains("nav-items"));
    assert!(rendered.contains("section-header"));
    assert!(rendered.contains("section-items"));
}

#[test]
fn test_navigation_accessibility() {
    let app_state = use_signal(|| AppState::default());

    let header_component = rsx! {
        Header {
            platform: Platform::Web,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: None,
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    // Verify accessibility attributes
    assert!(rendered.contains("<header"));
    assert!(rendered.contains("</header>"));
    assert!(rendered.contains("<nav"));
    assert!(rendered.contains("</nav>"));
}

#[test]
fn test_navigation_theme_consistency() {
    let app_state = use_signal(|| AppState::default());

    // Test Light theme navigation
    let light_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(surfdesk_core::components::app_shell::Theme::Light),
        }
    };
    let light_rendered = dioxus_ssr::render_element(light_shell);
    assert!(light_rendered.contains("theme-light"));

    // Test Dark theme navigation
    let dark_shell = rsx! {
        AppShell {
            state: use_signal(|| AppState::default()),
            platform: Platform::Desktop,
            theme: Some(surfdesk_core::components::app_shell::Theme::Dark),
        }
    };
    let dark_rendered = dioxus_ssr::render_element(dark_shell);
    assert!(dark_rendered.contains("theme-dark"));
}

#[test]
fn test_navigation_state_persistence() {
    let mut app_state = use_signal(|| AppState::default());

    // Simulate navigation to different sections
    let sections = vec!["dashboard", "surfpool", "accounts", "transactions"];

    for section in sections {
        app_state.write().active_section = section.to_string();

        let shell = rsx! {
            AppShell {
                state: app_state,
                platform: Platform::Desktop,
                theme: None,
            }
        };

        let rendered = dioxus_ssr::render_element(shell);

        // Verify the shell contains navigation elements
        assert!(rendered.contains("app-shell"));
        assert!(rendered.contains("platform-desktop"));
    }
}
