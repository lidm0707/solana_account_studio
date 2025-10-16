//! # Enhanced Modal Components
//!
//! Professional modal system with smooth animations, accessibility features,
//! and comprehensive interaction handling for the SurfDesk desktop application.

use super::{css_class, Size};
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;

/// Modal size variants
#[derive(Debug, Clone, PartialEq)]
pub enum ModalSize {
    /// Small modal (320px max-width)
    Small,
    /// Medium modal (480px max-width)
    Medium,
    /// Large modal (640px max-width)
    Large,
    /// Full screen modal
    Fullscreen,
    /// Auto size based on content
    Auto,
}

/// Modal variant types
#[derive(Debug, Clone, PartialEq)]
pub enum ModalVariant {
    /// Standard modal
    Default,
    /// Alert modal with warning styling
    Alert,
    /// Confirm modal with question styling
    Confirm,
    /// Success modal with positive styling
    Success,
    /// Error modal with error styling
    Error,
    /// Info modal with informational styling
    Info,
}

/// Enhanced modal component props
#[derive(Debug, Clone, Props)]
pub struct ModalProps {
    /// Whether the modal is open
    #[props(default = false)]
    pub open: bool,

    /// Modal title
    pub title: Option<String>,

    /// Modal size
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,

    /// Modal variant
    #[props(default = ModalVariant::Default)]
    pub variant: ModalVariant,

    /// Whether to show close button
    #[props(default = true)]
    pub show_close_button: bool,

    /// Whether to close on backdrop click
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Whether to close on escape key
    #[props(default = true)]
    pub close_on_escape: bool,

    /// Whether to prevent body scroll when modal is open
    #[props(default = true)]
    pub prevent_body_scroll: bool,

    /// Custom CSS classes
    pub class: Option<String>,

    /// Custom overlay CSS classes
    pub overlay_class: Option<String>,

    /// Close handler
    pub on_close: EventHandler<()>,

    /// Confirm handler (optional)
    pub on_confirm: Option<EventHandler<()>>,

    /// Cancel handler (optional)
    pub on_cancel: Option<EventHandler<()>>,

    /// Modal content
    pub children: Element,
}

/// Enhanced modal component
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut is_closing = use_signal(|| false);
    let mut previous_focus = use_signal(|| None as Option<web_sys::Element>);
    let modal_ref = use_signal(|| None as Option<web_sys::HtmlElement>);

    // Handle modal close with animation
    let handle_close = move |should_animate: bool| {
        if should_animate {
            is_closing.set(true);
            // Use setTimeout to allow animation to complete
            use_coroutine(|_| {
                let on_close = props.on_close.clone();
                async move {
                    gloo_timers::callback::Timeout::new(200, move || {
                        on_close.call(());
                    })
                    .forget();
                }
            });
        } else {
            props.on_close.call(());
        }
    };

    // Handle backdrop click
    let handle_backdrop_click = move |evt: MouseEvent| {
        if props.close_on_backdrop_click {
            evt.stop_propagation();
            handle_close(true);
        }
    };

    // Handle escape key
    use_effect(move || {
        if props.open && props.close_on_escape {
            let handle_close_clone = handle_close.clone();
            let listener = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
                if evt.key() == "Escape" {
                    evt.prevent_default();
                    handle_close_clone(true);
                }
            }) as Box<dyn Fn(web_sys::KeyboardEvent)>);

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();

            listener.forget();
        }

        async move {}
    });

    // Handle focus trapping
    use_effect(move || {
        if props.open {
            // Store current focus
            if let Some(element) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .active_element()
            {
                previous_focus.set(Some(element));
            }

            // Focus first focusable element in modal
            if let Some(modal) = modal_ref() {
                if let Some(first_focusable) = get_first_focusable_element(&modal) {
                    first_focusable.focus().unwrap();
                }
            }

            // Prevent body scroll
            if props.prevent_body_scroll {
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .style()
                    .set_property("overflow", "hidden")
                    .unwrap();
            }
        } else {
            // Restore focus
            if let Some(element) = previous_focus.take() {
                let _ = element.focus();
            }

            // Restore body scroll
            if props.prevent_body_scroll {
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .style()
                    .remove_property("overflow")
                    .unwrap();
            }
        }

        async move {}
    });

    if !props.open && !is_closing() {
        return rsx! { "" };
    }

    // Build CSS classes
    let overlay_classes = css_class("modal-overlay", &[]);
    let overlay_final_class = match &props.overlay_class {
        Some(custom) => format!("{} {}", overlay_classes, custom),
        None => overlay_classes,
    };

    let modal_classes = css_class(
        "modal",
        &[
            match props.size {
                ModalSize::Small => "small",
                ModalSize::Medium => "medium",
                ModalSize::Large => "large",
                ModalSize::Fullscreen => "fullscreen",
                ModalSize::Auto => "auto",
            },
            match props.variant {
                ModalVariant::Default => "default",
                ModalVariant::Alert => "alert",
                ModalVariant::Confirm => "confirm",
                ModalVariant::Success => "success",
                ModalVariant::Error => "error",
                ModalVariant::Info => "info",
            },
            if is_closing() { "closing" } else { "opening" },
        ],
    );

    let modal_final_class = match &props.class {
        Some(custom) => format!("{} {}", modal_classes, custom),
        None => modal_classes,
    };

    rsx! {
        div {
            class: "{overlay_final_class}",
            onclick: handle_backdrop_click,
            "aria-hidden": "false",
            "aria-modal": "true",
            "role": "dialog",

            div {
                class: "{modal_final_class}",
                r#ref: move |elem: web_sys::HtmlElement| {
                    modal_ref.set(Some(elem));
                },
                onclick: move |evt: MouseEvent| {
                    evt.stop_propagation();
                },
                "aria-labelledby": if props.title.is_some() { "modal-title" } else { "" },
                "aria-describedby": "modal-content",

                // Modal header
                div { class: "modal-header",
                    if let Some(title) = &props.title {
                        h2 {
                            class: "modal-title",
                            id: "modal-title",
                            "{title}"
                        }
                    }

                    if props.show_close_button {
                        button {
                            class: "modal-close-button",
                            "aria-label": "Close modal",
                            onclick: move |_: MouseEvent| {
                                handle_close(true);
                            },
                            r#type: "button",

                            span { class: "close-icon", "×" }
                        }
                    }
                }

                // Modal body
                div {
                    class: "modal-body",
                    id: "modal-content",
                    {props.children}
                }

                // Modal footer
                div { class: "modal-footer",
                    if props.on_confirm.is_some() || props.on_cancel.is_some() {
                        rsx! {
                            div { class: "modal-actions",
                                if let Some(on_cancel) = props.on_cancel {
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_: MouseEvent| {
                                            on_cancel.call(());
                                        },
                                        r#type: "button",
                                        "Cancel"
                                    }
                                }

                                if let Some(on_confirm) = props.on_confirm {
                                    button {
                                        class: "btn btn-primary",
                                        onclick: move |_: MouseEvent| {
                                            on_confirm.call(());
                                        },
                                        r#type: "button",
                                        "Confirm"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Alert modal props
#[derive(Debug, Clone, Props)]
pub struct AlertModalProps {
    /// Whether the modal is open
    #[props(default = false)]
    pub open: bool,

    /// Alert title
    pub title: String,

    /// Alert message
    pub message: String,

    /// Alert variant
    #[props(default = ModalVariant::Alert)]
    pub variant: ModalVariant,

    /// Custom confirm button text
    #[props(default = "OK".to_string())]
    pub confirm_text: String,

    /// Close handler
    pub on_close: EventHandler<()>,
}

/// Alert modal component
#[component]
pub fn AlertModal(props: AlertModalProps) -> Element {
    rsx! {
        Modal {
            open: props.open,
            title: Some(props.title),
            variant: props.variant,
            size: ModalSize::Small,
            show_close_button: true,
            close_on_backdrop_click: true,
            close_on_escape: true,
            on_close: props.on_close,
            on_confirm: Some(props.on_close),

            div { class: "modal-content-wrapper",
                p { class: "modal-message", "{props.message}" }
            }
        }
    }
}

/// Confirm modal props
#[derive(Debug, Clone, Props)]
pub struct ConfirmModalProps {
    /// Whether the modal is open
    #[props(default = false)]
    pub open: bool,

    /// Confirm title
    pub title: String,

    /// Confirm message
    pub message: String,

    /// Custom confirm button text
    #[props(default = "Confirm".to_string())]
    pub confirm_text: String,

    /// Custom cancel button text
    #[props(default = "Cancel".to_string())]
    pub cancel_text: String,

    /// Close handler
    pub on_close: EventHandler<()>,

    /// Confirm handler
    pub on_confirm: EventHandler<()>,
}

/// Confirm modal component
#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    rsx! {
        Modal {
            open: props.open,
            title: Some(props.title),
            variant: ModalVariant::Confirm,
            size: ModalSize::Small,
            show_close_button: true,
            close_on_backdrop_click: true,
            close_on_escape: true,
            on_close: props.on_close,
            on_cancel: Some(props.on_close),
            on_confirm: Some(props.on_confirm),

            div { class: "modal-content-wrapper",
                p { class: "modal-message", "{props.message}" }
            }
        }
    }
}

/// Utility function to get first focusable element
fn get_first_focusable_element(container: &web_sys::HtmlElement) -> Option<web_sys::Element> {
    let selectors = [
        "button:not([disabled])",
        "input:not([disabled])",
        "select:not([disabled])",
        "textarea:not([disabled])",
        "a[href]",
        "[tabindex]:not([tabindex=\"-1\"])",
    ]
    .join(", ");

    container.query_selector(&selectors).ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_size() {
        assert_eq!(css_class("modal", &["small"]), "modal modal--small");
        assert_eq!(css_class("modal", &["medium"]), "modal modal--medium");
        assert_eq!(css_class("modal", &["large"]), "modal modal--large");
    }

    #[test]
    fn test_modal_variant() {
        let variants = vec![
            ModalVariant::Default,
            ModalVariant::Alert,
            ModalVariant::Confirm,
            ModalVariant::Success,
            ModalVariant::Error,
            ModalVariant::Info,
        ];

        for variant in variants {
            let class = match variant {
                ModalVariant::Default => "default",
                ModalVariant::Alert => "alert",
                ModalVariant::Confirm => "confirm",
                ModalVariant::Success => "success",
                ModalVariant::Error => "error",
                ModalVariant::Info => "info",
            };
            assert_eq!(
                css_class("modal", &[class]),
                format!("modal modal--{}", class)
            );
        }
    }
}
