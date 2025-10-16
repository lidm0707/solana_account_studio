use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::Sidebar;
use surfdesk_core::platform::Platform;

#[test]
fn test_sidebar_renders_on_desktop() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: None,
            class: Some("test-sidebar".to_string()),
            id: Some("test-sidebar-id".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar"));
    assert!(rendered.contains("platform-sidebar"));
    assert!(rendered.contains("sidebar-desktop"));
    assert!(rendered.contains("test-sidebar"));
}

#[test]
fn test_sidebar_renders_on_web() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Web,
            is_collapsed: false,
            active_section: "surfpool".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar"));
    assert!(rendered.contains("platform-sidebar"));
    assert!(rendered.contains("sidebar-web"));
}

#[test]
fn test_sidebar_renders_on_terminal() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Terminal,
            is_collapsed: false,
            active_section: "accounts".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar"));
    assert!(rendered.contains("platform-sidebar"));
    assert!(rendered.contains("sidebar-terminal"));
}

#[test]
fn test_sidebar_collapsed_state() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: true,
            active_section: "dashboard".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar-collapsed"));
    assert!(rendered.contains("collapsed-sidebar"));
}

#[test]
fn test_sidebar_navigation_sections() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar-nav"));
    assert!(rendered.contains("nav-section"));
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("SurfPool"));
    assert!(rendered.contains("Accounts"));
    assert!(rendered.contains("Transactions"));
}

#[test]
fn test_sidebar_active_section_highlighting() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "surfpool".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("nav-item-active"));
    assert!(rendered.contains("active-surfpool"));
}

#[test]
fn test_sidebar_collapsible_functionality() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar-toggle"));
    assert!(rendered.contains("collapse-button"));
}

#[test]
fn test_sidebar_responsive_classes() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Web,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: None,
            class: Some("custom-class responsive".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("sidebar"));
    assert!(rendered.contains("platform-sidebar"));
    assert!(rendered.contains("sidebar-web"));
    assert!(rendered.contains("custom-class"));
    assert!(rendered.contains("responsive"));
}

#[test]
fn test_sidebar_hierarchical_structure() {
    let sidebar_component = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "accounts".to_string(),
            on_section_change: None,
        }
    };

    let rendered = dioxus_ssr::render_element(sidebar_component);

    assert!(rendered.contains("nav-hierarchy"));
    assert!(rendered.contains("nav-sections"));
    assert!(rendered.contains("nav-items"));
}

#[test]
fn test_sidebar_platform_specific_adaptations() {
    // Test desktop sidebar
    let desktop_sidebar = rsx! {
        Sidebar {
            platform: Platform::Desktop,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: None,
        }
    };

    let desktop_rendered = dioxus_ssr::render_element(desktop_sidebar);
    assert!(desktop_rendered.contains("desktop-sidebar"));
    assert!(desktop_rendered.contains("native-scroll"));

    // Test terminal sidebar
    let terminal_sidebar = rsx! {
        Sidebar {
            platform: Platform::Terminal,
            is_collapsed: false,
            active_section: "dashboard".to_string(),
            on_section_change: None,
        }
    };

    let terminal_rendered = dioxus_ssr::render_element(terminal_sidebar);
    assert!(terminal_rendered.contains("terminal-sidebar"));
    assert!(terminal_rendered.contains("ascii-art"));
}
