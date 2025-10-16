use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::components::Modal;
use surfdesk_core::platform::Platform;

#[test]
fn test_modal_renders_when_open() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
            class: Some("test-modal".to_string()),
            id: Some("test-modal-id".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal"));
    assert!(rendered.contains("modal-open"));
    assert!(rendered.contains("platform-modal"));
    assert!(rendered.contains("modal-desktop"));
    assert!(rendered.contains("test-modal"));
}

#[test]
fn test_modal_does_not_render_when_closed() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: false,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(!rendered.contains("modal"));
    assert!(!rendered.contains("modal-open"));
}

#[test]
fn test_modal_renders_on_web() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Web,
            is_open: true,
            title: Some("Web Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal"));
    assert!(rendered.contains("modal-web"));
    assert!(rendered.contains("web-modal"));
}

#[test]
fn test_modal_renders_on_terminal() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Terminal,
            is_open: true,
            title: Some("Terminal Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal"));
    assert!(rendered.contains("modal-terminal"));
    assert!(rendered.contains("terminal-modal"));
}

#[test]
fn test_modal_with_title() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Modal Title".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal-title"));
    assert!(rendered.contains("Modal Title"));
}

#[test]
fn test_modal_without_title() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: None,
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(!rendered.contains("modal-title"));
    assert!(rendered.contains("modal-content"));
}

#[test]
fn test_modal_with_close_button() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal-close"));
    assert!(rendered.contains("close-button"));
}

#[test]
fn test_modal_overlay() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal-overlay"));
    assert!(rendered.contains("overlay-backdrop"));
}

#[test]
fn test_modal_responsive_classes() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Web,
            is_open: true,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
            class: Some("custom-class responsive".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal"));
    assert!(rendered.contains("platform-modal"));
    assert!(rendered.contains("modal-web"));
    assert!(rendered.contains("custom-class"));
    assert!(rendered.contains("responsive"));
}

#[test]
fn test_modal_content_rendering() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Test Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal-content"));
    assert!(rendered.contains("content-wrapper"));
}

#[test]
fn test_modal_platform_specific_adaptations() {
    // Desktop modal
    let desktop_modal = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Desktop Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let desktop_rendered = dioxus_ssr::render_element(desktop_modal);
    assert!(desktop_rendered.contains("desktop-modal"));
    assert!(desktop_rendered.contains("backdrop-blur"));

    // Terminal modal
    let terminal_modal = rsx! {
        Modal {
            platform: Platform::Terminal,
            is_open: true,
            title: Some("Terminal Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let terminal_rendered = dioxus_ssr::render_element(terminal_modal);
    assert!(terminal_rendered.contains("terminal-modal"));
    assert!(terminal_rendered.contains("ascii-border"));
}

#[test]
fn test_modal_accessibility_attributes() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Accessible Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
            id: Some("accessible-modal".to_string()),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("role=\"dialog\""));
    assert!(rendered.contains("aria-modal=\"true\""));
    assert!(rendered.contains("aria-labelledby=\"accessible-modal-title\""));
    assert!(rendered.contains("id=\"accessible-modal\""));
}

#[test]
fn test_modal_transitions() {
    let modal_component = rsx! {
        Modal {
            platform: Platform::Web,
            is_open: true,
            title: Some("Animated Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
        }
    };

    let rendered = dioxus_ssr::render_element(modal_component);

    assert!(rendered.contains("modal-transition"));
    assert!(rendered.contains("transition-fade"));
    assert!(rendered.contains("transition-scale"));
}

#[test]
fn test_modal_size_variants() {
    let small_modal = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Small Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
            class: Some("modal-small".to_string()),
        }
    };

    let small_rendered = dioxus_ssr::render_element(small_modal);
    assert!(small_rendered.contains("modal-small"));
    assert!(small_rendered.contains("size-small"));

    let large_modal = rsx! {
        Modal {
            platform: Platform::Desktop,
            is_open: true,
            title: Some("Large Modal".to_string()),
            on_close: Some(Box::new(|_| {})),
            class: Some("modal-large".to_string()),
        }
    };

    let large_rendered = dioxus_ssr::render_element(large_modal);
    assert!(large_rendered.contains("modal-large"));
    assert!(large_rendered.contains("size-large"));
}
