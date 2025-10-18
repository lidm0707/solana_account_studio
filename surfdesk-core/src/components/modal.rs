//! # Modal Component (MVP Version)
//!
//! Simplified modal component for MVP approach.
//! Focus on compilation success over comprehensive features.

use crate::components::{Color, Size};
use dioxus::prelude::*;

/// Modal component properties (MVP simplified)
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ModalProps {
    /// Modal title
    #[props(optional)]
    pub title: Option<String>,

    /// Modal message
    #[props(optional)]
    pub message: Option<String>,

    /// Whether modal is open
    #[props(optional)]
    pub open: Option<bool>,

    /// Modal size
    #[props(optional)]
    pub size: Option<Size>,

    /// Modal color
    #[props(optional)]
    pub color: Option<Color>,

    /// Close handler
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,

    /// Confirm handler
    #[props(optional)]
    pub on_confirm: Option<EventHandler<()>>,

    /// Confirm button text
    #[props(optional)]
    pub confirm_text: Option<String>,

    /// Cancel button text
    #[props(optional)]
    pub cancel_text: Option<String>,

    /// Modal content
    children: Element,
}

/// Simple modal component
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let open = props.open.unwrap_or(false);
    let size = props.size.unwrap_or(Size::Medium);
    let color = props.color.unwrap_or(Color::Primary);
    let confirm_text = props.confirm_text.unwrap_or_else(|| "Confirm".to_string());
    let cancel_text = props.cancel_text.unwrap_or_else(|| "Cancel".to_string());

    if !open {
        return rsx! { "" };
    }

    let handle_close = move |_| {
        if let Some(on_close) = props.on_close {
            on_close(());
        }
    };

    let handle_confirm = move |_| {
        if let Some(on_confirm) = props.on_confirm {
            on_confirm(());
        }
    };

    rsx! {
        div {
            class: "modal-overlay",
            onclick: handle_close,

            div {
                class: "modal {size.css_class()} {color.css_class()}",
                onclick: move |evt| evt.stop_propagation(),

                // Modal header
                if let Some(title) = props.title {
                    div { class: "modal-header",
                        h3 { class: "modal-title", "{title}" }
                        button {
                            class: "modal-close",
                            onclick: handle_close,
                            "×"
                        }
                    }
                }

                // Modal body
                div { class: "modal-body",
                    if let Some(message) = props.message {
                        p { class: "modal-message", "{message}" }
                    }
                    {props.children}
                }

                // Modal actions
                div { class: "modal-actions",
                    button {
                        class: "btn-cancel",
                        onclick: handle_close,
                        "{cancel_text}"
                    }

                    button {
                        class: "btn-confirm",
                        onclick: handle_confirm,
                        "{confirm_text}"
                    }
                }
            }
        }
    }
}

/// Alert modal props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertModalProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub open: Option<bool>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

/// Alert modal
#[component]
pub fn AlertModal(props: AlertModalProps) -> Element {
    rsx! {
        Modal {
            title: props.title,
            message: props.message,
            open: props.open,
            on_close: props.on_close,
            confirm_text: "OK".to_string(),
            cancel_text: None,
        }
    }
}

/// Confirm modal props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ConfirmModalProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub open: Option<bool>,
    #[props(optional)]
    pub on_confirm: Option<EventHandler<()>>,
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
    #[props(optional)]
    pub confirm_text: Option<String>,
    #[props(optional)]
    pub cancel_text: Option<String>,
}

/// Confirm modal
#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    rsx! {
        Modal {
            title: props.title,
            message: props.message,
            open: props.open,
            on_close: props.on_cancel,
            on_confirm: props.on_confirm,
            confirm_text: props.confirm_text,
            cancel_text: props.cancel_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_basic() {
        let modal = ModalProps {
            title: Some("Test Modal".to_string()),
            message: Some("Test message".to_string()),
            open: Some(true),
            size: Some(Size::Medium),
            color: Some(Color::Primary),
            on_close: None,
            on_confirm: None,
            confirm_text: Some("OK".to_string()),
            cancel_text: Some("Cancel".to_string()),
            children: rsx! { "Test content" },
        };

        assert_eq!(modal.title, Some("Test Modal".to_string()));
        assert_eq!(modal.message, Some("Test message".to_string()));
        assert_eq!(modal.open, Some(true));
    }

    #[test]
    fn test_alert_modal() {
        let alert = AlertModalProps {
            title: "Alert".to_string(),
            message: "This is an alert".to_string(),
            open: Some(true),
            on_close: None,
        };

        assert_eq!(alert.title, "Alert");
        assert_eq!(alert.message, "This is an alert");
        assert_eq!(alert.open, Some(true));
    }

    #[test]
    fn test_confirm_modal() {
        let confirm = ConfirmModalProps {
            title: "Confirm".to_string(),
            message: "Are you sure?".to_string(),
            open: Some(true),
            on_confirm: None,
            on_cancel: None,
            confirm_text: Some("Yes".to_string()),
            cancel_text: Some("No".to_string()),
        };

        assert_eq!(confirm.title, "Confirm");
        assert_eq!(confirm.message, "Are you sure?");
        assert_eq!(confirm.open, Some(true));
        assert_eq!(confirm.confirm_text, Some("Yes".to_string()));
        assert_eq!(confirm.cancel_text, Some("No".to_string()));
    }
}
