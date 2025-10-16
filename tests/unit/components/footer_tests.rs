use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::Footer;
use surfdesk_core::platform::Platform;

#[test]
fn test_footer_renders_on_desktop() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: true,
            class: Some("test-footer".to_string()),
            id: Some("test-footer-id".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("footer"));
    assert!(rendered.contains("platform-footer"));
    assert!(rendered.contains("footer-desktop"));
    assert!(rendered.contains("test-footer"));
}

#[test]
fn test_footer_renders_on_web() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Web,
            connected: false,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("footer"));
    assert!(rendered.contains("platform-footer"));
    assert!(rendered.contains("footer-web"));
}

#[test]
fn test_footer_renders_on_terminal() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Terminal,
            connected: true,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("footer"));
    assert!(rendered.contains("platform-footer"));
    assert!(rendered.contains("footer-terminal"));
}

#[test]
fn test_footer_connected_status() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: true,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("status-connected"));
    assert!(rendered.contains("connection-status"));
    assert!(rendered.contains("Connected"));
}

#[test]
fn test_footer_disconnected_status() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: false,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("status-disconnected"));
    assert!(rendered.contains("connection-status"));
    assert!(rendered.contains("Disconnected"));
}

#[test]
fn test_footer_status_indicator() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Web,
            connected: true,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("status-indicator"));
    assert!(rendered.contains("indicator-green"));
}

#[test]
fn test_footer_platform_specific_content() {
    // Desktop footer
    let desktop_footer = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: true,
        }
    };

    let desktop_rendered = dioxus_ssr::render_element(desktop_footer);
    assert!(desktop_rendered.contains("footer-actions"));
    assert!(desktop_rendered.contains("desktop-actions"));

    // Terminal footer
    let terminal_footer = rsx! {
        Footer {
            platform: Platform::Terminal,
            connected: true,
        }
    };

    let terminal_rendered = dioxus_ssr::render_element(terminal_footer);
    assert!(terminal_rendered.contains("terminal-status"));
    assert!(terminal_rendered.contains("ascii-indicator"));
}

#[test]
fn test_footer_responsive_classes() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Web,
            connected: true,
            class: Some("custom-class responsive".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("footer"));
    assert!(rendered.contains("platform-footer"));
    assert!(rendered.contains("footer-web"));
    assert!(rendered.contains("custom-class"));
    assert!(rendered.contains("responsive"));
}

#[test]
fn test_footer_status_transitions() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: true,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("status-transition"));
    assert!(rendered.contains("transition-enabled"));
}

#[test]
fn test_footer_metadata_display() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Web,
            connected: true,
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("footer-metadata"));
    assert!(rendered.contains("version-info"));
    assert!(rendered.contains("build-info"));
}

#[test]
fn test_footer_accessibility_attributes() {
    let footer_component = rsx! {
        Footer {
            platform: Platform::Desktop,
            connected: true,
            id: Some("main-footer".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(footer_component);

    assert!(rendered.contains("role=\"contentinfo\""));
    assert!(rendered.contains("aria-label=\"Footer\""));
    assert!(rendered.contains("id=\"main-footer\""));
}
