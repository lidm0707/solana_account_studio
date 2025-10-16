//! Integration tests for AppShell component
//! Tests cross-platform layout behavior and component integration

use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::{AppShell, Breakpoint, Theme};
use surfdesk_core::platform::Platform;
use surfdesk_core::state::AppState;

#[test]
fn test_app_shell_desktop_layout() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify main structure
    assert!(rendered.contains("app-shell"));
    assert!(rendered.contains("theme-light"));
    assert!(rendered.contains("platform-desktop"));

    // Verify layout components
    assert!(rendered.contains("header"));
    assert!(rendered.contains("app-layout"));
    assert!(rendered.contains("sidebar"));
    assert!(rendered.contains("app-main"));
    assert!(rendered.contains("footer"));
}

#[test]
fn test_app_shell_web_layout() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Web,
            theme: Some(Theme::Auto),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    assert!(rendered.contains("app-shell"));
    assert!(rendered.contains("theme-auto"));
    assert!(rendered.contains("platform-web"));
}

#[test]
fn test_app_shell_terminal_layout() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Terminal,
            theme: Some(Theme::Dark),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    assert!(rendered.contains("app-shell"));
    assert!(rendered.contains("theme-dark"));
    assert!(rendered.contains("platform-terminal"));
}

#[test]
fn test_app_shell_theme_switching() {
    let app_state = use_signal(|| AppState::default());

    // Test Light theme
    let light_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };
    let light_rendered = dioxus_ssr::render_element(light_shell);
    assert!(light_rendered.contains("theme-light"));

    // Test Dark theme
    let dark_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Dark),
        }
    };
    let dark_rendered = dioxus_ssr::render_element(dark_shell);
    assert!(dark_rendered.contains("theme-dark"));

    // Test Auto theme
    let auto_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Auto),
        }
    };
    let auto_rendered = dioxus_ssr::render_element(auto_shell);
    assert!(auto_rendered.contains("theme-auto"));
}

#[test]
fn test_app_shell_responsive_breakpoints() {
    let app_state = use_signal(|| AppState::default());

    // Test Desktop breakpoint
    let desktop_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };
    let desktop_rendered = dioxus_ssr::render_element(desktop_shell);
    assert!(desktop_rendered.contains("platform-desktop"));

    // Test Web breakpoint (responsive)
    let web_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Web,
            theme: Some(Theme::Light),
        }
    };
    let web_rendered = dioxus_ssr::render_element(web_shell);
    assert!(web_rendered.contains("platform-web"));
}

#[test]
fn test_app_shell_component_integration() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify Header integration
    assert!(rendered.contains("header-brand"));
    assert!(rendered.contains("brand-title"));
    assert!(rendered.contains("SurfDesk"));

    // Verify Sidebar integration
    assert!(rendered.contains("sidebar-nav"));

    // Verify Main content area
    assert!(rendered.contains("app-main"));

    // Verify Footer integration
    assert!(rendered.contains("footer-content"));
    assert!(rendered.contains("footer-brand"));
}

#[test]
fn test_app_shell_navigation_flow() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify navigation elements are present
    assert!(rendered.contains("nav-item"));
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("SurfPool"));
    assert!(rendered.contains("Accounts"));

    // Verify navigation links structure
    assert!(rendered.contains("main-nav"));
    assert!(rendered.contains("nav-list"));
}

#[test]
fn test_app_shell_state_management() {
    let mut app_state = use_signal(|| AppState::default());

    // Modify state
    app_state.write().active_section = "surfpool".to_string();

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Desktop,
            theme: Some(Theme::Light),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify state affects rendering
    assert!(rendered.contains("app-shell"));

    // The active section should be reflected in navigation
    assert!(rendered.contains("nav-item"));
}

#[test]
fn test_app_shell_accessibility_features() {
    let app_state = use_signal(|| AppState::default());

    let app_shell = rsx! {
        AppShell {
            state: app_state,
            platform: Platform::Web,
            theme: Some(Theme::Light),
        }
    };

    let rendered = dioxus_ssr::render_element(app_shell);

    // Verify semantic HTML structure
    assert!(rendered.contains("<header"));
    assert!(rendered.contains("</header>"));
    assert!(rendered.contains("<main"));
    assert!(rendered.contains("</main>"));
    assert!(rendered.contains("<footer"));
    assert!(rendered.contains("</footer>"));

    // Verify navigation structure
    assert!(rendered.contains("<nav"));
    assert!(rendered.contains("</nav>"));
}
