//! # Modal Component
//!
//! Modal component for displaying dialogs, confirmations, and
//! overlays that require user interaction.

use crate::components::{combine_classes, Button, CommonProps, Size, Variant};
use dioxus::prelude::*;

/// Modal component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ModalProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Modal title
    #[props(optional)]
    pub title: Option<String>,

    /// Modal content
    #[props(optional)]
    pub content: Option<String>,

    /// Modal variant
    #[props(optional)]
    pub variant: Option<ModalVariant>,

    /// Modal size
    #[props(optional)]
    pub size: Option<Size>,

    /// Whether modal is open
    #[props(optional)]
    pub open: Option<bool>,

    /// Whether modal can be closed by clicking outside
    #[props(optional)]
    pub close_on_outside_click: Option<bool>,

    /// Whether modal can be closed with escape key
    #[props(optional)]
    pub close_on_escape: Option<bool>,

    /// Whether to show close button
    #[props(optional)]
    pub show_close_button: Option<bool>,

    /// Close handler
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,

    /// Confirm handler
    #[props(optional)]
    pub on_confirm: Option<EventHandler<()>>,

    /// Cancel handler
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,

    /// Confirm button text
    #[props(optional)]
    pub confirm_text: Option<String>,

    /// Cancel button text
    #[props(optional)]
    pub cancel_text: Option<String>,

    /// Whether to show actions
    #[props(optional)]
    pub show_actions: Option<bool>,

    /// Modal content
    children: Element,
}

/// Modal variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalVariant {
    Default,
    Dialog,
    Alert,
    Confirm,
    Info,
    Warning,
    Error,
}

impl ModalVariant {
    /// Get CSS class for variant
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Default => "modal-default",
            Self::Dialog => "modal-dialog",
            Self::Alert => "modal-alert",
            Self::Confirm => "modal-confirm",
            Self::Info => "modal-info",
            Self::Warning => "modal-warning",
            Self::Error => "modal-error",
        }
    }
}

/// Modal component
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let common = props.common.unwrap_or_default();
    let variant = props.variant.unwrap_or(ModalVariant::Default);
    let size = props.size.unwrap_or(Size::Medium);
    let open = props.open.unwrap_or(false);
    let close_on_outside_click = props.close_on_outside_click.unwrap_or(true);
    let close_on_escape = props.close_on_escape.unwrap_or(true);
    let show_close_button = props.show_close_button.unwrap_or(true);
    let show_actions = props.show_actions.unwrap_or(true);
    let confirm_text = props.confirm_text.unwrap_or_else(|| "Confirm".to_string());
    let cancel_text = props.cancel_text.unwrap_or_else(|| "Cancel".to_string());

    if !open {
        return rsx! { "" };
    }

    let mut classes = vec!["modal"];
    classes.push(variant.css_class());
    classes.push(size.css_class());

    if let Some(class) = &common.class {
        classes.push(class);
    }

    let handle_close = move |_| {
        if let Some(on_close) = props.on_close {
            on_close(());
        }
    };

    let handle_outside_click = move |_| {
        if close_on_outside_click {
            if let Some(on_close) = props.on_close {
                on_close(());
            }
        }
    };

    let handle_confirm = move |_| {
        if let Some(on_confirm) = props.on_confirm {
            on_confirm(());
        }
    };

    let handle_cancel = move |_| {
        if let Some(on_cancel) = props.on_cancel {
            on_cancel(());
        }
        if let Some(on_close) = props.on_close {
            on_close(());
        }
    };

    rsx! {
        div {
            class: combine_classes(&["modal-overlay", &classes.join(" ")]),
            id: common.id,
            onclick: handle_outside_click,

            div {
                class: "modal-content",
                onclick: move |evt| evt.stop_propagation(),

                // Modal header
                if props.title.is_some() || show_close_button {
                    div { class: "modal-header",
                        if let Some(title) = props.title {
                            h2 { class: "modal-title", "{title}" }
                        }

                        if show_close_button {
                            button {
                                class: "modal-close",
                                onclick: handle_close,
                                "×"
                            }
                        }
                    }
                }

                // Modal body
                div { class: "modal-body",
                    if let Some(content) = props.content {
                        p { class: "modal-text", "{content}" }
                    }
                    {props.children}
                }

                // Modal actions
                if show_actions {
                    div { class: "modal-actions",
                        Button {
                            variant: Variant::Text,
                            onclick: handle_cancel,
                            "{cancel_text}"
                        }

                        Button {
                            variant: Variant::Contained,
                            onclick: handle_confirm,
                            "{confirm_text}"
                        }
                    }
                }
            }
        }
    }
}

/// Alert modal
#[component]
pub fn AlertModal(
    title: String,
    message: String,
    open: bool,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Modal {
            title: title,
            content: message,
            variant: ModalVariant::Alert,
            open: open,
            show_actions: false,
            show_close_button: true,
            on_close: on_close,
            class: class,
        }
    }
}

/// Confirm modal
#[component]
pub fn ConfirmModal(
    title: String,
    message: String,
    open: bool,
    #[props(optional)] on_confirm: Option<EventHandler<()>>,
    #[props(optional)] on_cancel: Option<EventHandler<()>>,
    #[props(optional)] confirm_text: Option<String>,
    #[props(optional)] cancel_text: Option<String>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Modal {
            title: title,
            content: message,
            variant: ModalVariant::Confirm,
            open: open,
            show_actions: true,
            show_close_button: false,
            on_confirm: on_confirm,
            on_cancel: on_cancel,
            confirm_text: confirm_text,
            cancel_text: cancel_text,
            class: class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_default_props() {
        let props = ModalProps {
            common: None,
            title: None,
            content: None,
            variant: None,
            size: None,
            open: None,
            close_on_outside_click: None,
            close_on_escape: None,
            show_close_button: None,
            on_close: None,
            on_confirm: None,
            on_cancel: None,
            confirm_text: None,
            cancel_text: None,
            show_actions: None,
            children: rsx! { "Test content" },
        };

        assert_eq!(
            props.variant.unwrap_or(ModalVariant::Default),
            ModalVariant::Default
        );
        assert_eq!(props.size.unwrap_or(Size::Medium), Size::Medium);
        assert!(!props.open.unwrap_or(false));
        assert!(props.close_on_outside_click.unwrap_or(true));
        assert!(props.close_on_escape.unwrap_or(true));
        assert!(props.show_close_button.unwrap_or(true));
        assert!(props.show_actions.unwrap_or(true));
        assert_eq!(
            props.confirm_text.unwrap_or_else(|| "Confirm".to_string()),
            "Confirm"
        );
        assert_eq!(
            props.cancel_text.unwrap_or_else(|| "Cancel".to_string()),
            "Cancel"
        );
    }

    #[test]
    fn test_modal_variant_css_class() {
        assert_eq!(ModalVariant::Default.css_class(), "modal-default");
        assert_eq!(ModalVariant::Dialog.css_class(), "modal-dialog");
        assert_eq!(ModalVariant::Alert.css_class(), "modal-alert");
        assert_eq!(ModalVariant::Confirm.css_class(), "modal-confirm");
        assert_eq!(ModalVariant::Info.css_class(), "modal-info");
        assert_eq!(ModalVariant::Warning.css_class(), "modal-warning");
        assert_eq!(ModalVariant::Error.css_class(), "modal-error");
    }

    #[test]
    fn test_alert_modal() {
        let modal = AlertModal {
            title: "Alert".to_string(),
            message: "This is an alert".to_string(),
            open: true,
            on_close: None,
            class: Some("alert-modal".to_string()),
        };

        assert_eq!(modal.title, "Alert");
        assert_eq!(modal.message, "This is an alert");
        assert!(modal.open);
        assert_eq!(modal.class, Some("alert-modal".to_string()));
    }

    #[test]
    fn test_confirm_modal() {
        let modal = ConfirmModal {
            title: "Confirm".to_string(),
            message: "Are you sure?".to_string(),
            open: true,
            on_confirm: None,
            on_cancel: None,
            confirm_text: Some("Yes".to_string()),
            cancel_text: Some("No".to_string()),
            class: None,
        };

        assert_eq!(modal.title, "Confirm");
        assert_eq!(modal.message, "Are you sure?");
        assert!(modal.open);
        assert_eq!(modal.confirm_text, Some("Yes".to_string()));
        assert_eq!(modal.cancel_text, Some("No".to_string()));
    }
}
