use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::Header;
use surfdesk_core::platform::Platform;

#[test]
fn test_header_renders_on_desktop() {
    let header_component = rsx! {
        Header {
            platform: Platform::Desktop,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: None,
            class: Some("test-header".to_string()),
            id: Some("test-header-id".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("header"));
    assert!(rendered.contains("platform-header"));
    assert!(rendered.contains("header-desktop"));
    assert!(rendered.contains("test-header"));
}

#[test]
fn test_header_renders_on_web() {
    let header_component = rsx! {
        Header {
            platform: Platform::Web,
            active_section: "surfpool".to_string(),
            on_sidebar_toggle: None,
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("header"));
    assert!(rendered.contains("platform-header"));
    assert!(rendered.contains("header-web"));
}

#[test]
fn test_header_renders_on_terminal() {
    let header_component = rsx! {
        Header {
            platform: Platform::Terminal,
            active_section: "accounts".to_string(),
            on_sidebar_toggle: None,
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("header"));
    assert!(rendered.contains("platform-header"));
    assert!(rendered.contains("header-terminal"));
}

#[test]
fn test_header_with_sidebar_toggle() {
    let header_component = rsx! {
        Header {
            platform: Platform::Desktop,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("sidebar-toggle"));
}

#[test]
fn test_header_navigation_items() {
    let header_component = rsx! {
        Header {
            platform: Platform::Desktop,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: None,
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("main-nav"));
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("SurfPool"));
    assert!(rendered.contains("Accounts"));
}

#[test]
fn test_header_brand_identity() {
    let header_component = rsx! {
        Header {
            platform: Platform::Desktop,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: None,
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("header-brand"));
    assert!(rendered.contains("brand-title"));
    assert!(rendered.contains("brand-icon"));
    assert!(rendered.contains("🏄"));
    assert!(rendered.contains("SurfDesk"));
}

#[test]
fn test_header_responsive_classes() {
    let header_component = rsx! {
        Header {
            platform: Platform::Web,
            active_section: "dashboard".to_string(),
            on_sidebar_toggle: None,
            class: Some("custom-class responsive".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(header_component);

    assert!(rendered.contains("header"));
    assert!(rendered.contains("platform-header"));
    assert!(rendered.contains("header-web"));
    assert!(rendered.contains("custom-class"));
    assert!(rendered.contains("responsive"));
}
