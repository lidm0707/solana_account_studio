//! # Notification Component (MVP Version)
//!
//! Simplified notification component for MVP approach.
//! Focus on compilation success over comprehensive features.

use crate::components::Size;
use dioxus::prelude::*;

/// Notification component properties (MVP simplified)
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NotificationProps {
    /// Notification title
    #[props(optional)]
    pub title: Option<String>,

    /// Notification message
    #[props(optional)]
    pub message: Option<String>,

    /// Notification type
    #[props(optional)]
    pub notification_type: Option<NotificationType>,

    /// Notification size
    #[props(optional)]
    pub size: Option<Size>,

    /// Whether notification is visible
    #[props(optional)]
    pub visible: Option<bool>,

    /// Whether notification is dismissible
    #[props(optional)]
    pub dismissible: Option<bool>,

    /// Close handler
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,

    /// Action handler
    #[props(optional)]
    pub on_action: Option<EventHandler<()>>,

    /// Action button text
    #[props(optional)]
    pub action_text: Option<String>,
}

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

impl NotificationType {
    /// Get CSS class for type
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Success => "notification-success",
            Self::Error => "notification-error",
            Self::Warning => "notification-warning",
            Self::Info => "notification-info",
        }
    }

    /// Get default icon for type
    pub fn default_icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Error => "✕",
            Self::Warning => "⚠",
            Self::Info => "ℹ",
        }
    }
}

/// Simple notification component
#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let notification_type = props.notification_type.unwrap_or(NotificationType::Info);
    let size = props.size.unwrap_or(Size::Medium);
    let visible = props.visible.unwrap_or(true);
    let dismissible = props.dismissible.unwrap_or(true);

    if !visible {
        return rsx! { "" };
    }

    let handle_close = move |_| {
        if let Some(on_close) = props.on_close {
            on_close(());
        }
    };

    let handle_action = move |_| {
        if let Some(on_action) = props.on_action {
            on_action(());
        }
    };

    rsx! {
        div {
            class: "notification {notification_type.css_class()} {size.css_class()}",

            // Icon
            span { class: "notification-icon",
                "{notification_type.default_icon()}"
            }

            // Content
            div { class: "notification-content",
                if let Some(title) = props.title {
                    h4 { class: "notification-title", "{title}" }
                }
                if let Some(message) = props.message {
                    p { class: "notification-message", "{message}" }
                }
            }

            // Actions
            div { class: "notification-actions",
                if let Some(action_text) = props.action_text {
                    button {
                        class: "notification-action",
                        onclick: handle_action,
                        "{action_text}"
                    }
                }

                if dismissible {
                    button {
                        class: "notification-close",
                        onclick: handle_close,
                        "×"
                    }
                }
            }
        }
    }
}

/// Success notification props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SuccessNotificationProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub visible: Option<bool>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

/// Success notification
#[component]
pub fn SuccessNotification(props: SuccessNotificationProps) -> Element {
    rsx! {
        Notification {
            title: props.title,
            message: props.message,
            notification_type: NotificationType::Success,
            visible: props.visible,
            on_close: props.on_close,
            dismissible: Some(true),
            action_text: None,
        }
    }
}

/// Error notification props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ErrorNotificationProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub visible: Option<bool>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

/// Error notification
#[component]
pub fn ErrorNotification(props: ErrorNotificationProps) -> Element {
    rsx! {
        Notification {
            title: props.title,
            message: props.message,
            notification_type: NotificationType::Error,
            visible: props.visible,
            on_close: props.on_close,
            dismissible: Some(true),
            action_text: None,
        }
    }
}

/// Warning notification props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct WarningNotificationProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub visible: Option<bool>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

/// Warning notification
#[component]
pub fn WarningNotification(props: WarningNotificationProps) -> Element {
    rsx! {
        Notification {
            title: props.title,
            message: props.message,
            notification_type: NotificationType::Warning,
            visible: props.visible,
            on_close: props.on_close,
            dismissible: Some(true),
            action_text: None,
        }
    }
}

/// Info notification props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InfoNotificationProps {
    pub title: String,
    pub message: String,
    #[props(optional)]
    pub visible: Option<bool>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
}

/// Info notification
#[component]
pub fn InfoNotification(props: InfoNotificationProps) -> Element {
    rsx! {
        Notification {
            title: props.title,
            message: props.message,
            notification_type: NotificationType::Info,
            visible: props.visible,
            on_close: props.on_close,
            dismissible: Some(true),
            action_text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_basic() {
        let notification = NotificationProps {
            title: Some("Test Notification".to_string()),
            message: Some("Test message".to_string()),
            notification_type: Some(NotificationType::Info),
            size: Some(Size::Medium),
            visible: Some(true),
            dismissible: Some(true),
            on_close: None,
            on_action: None,
            action_text: None,
        };

        assert_eq!(notification.title, Some("Test Notification".to_string()));
        assert_eq!(notification.message, Some("Test message".to_string()));
        assert_eq!(notification.notification_type, Some(NotificationType::Info));
        assert_eq!(notification.size, Some(Size::Medium));
        assert_eq!(notification.visible, Some(true));
        assert_eq!(notification.dismissible, Some(true));
    }

    #[test]
    fn test_notification_type_css_class() {
        assert_eq!(
            NotificationType::Success.css_class(),
            "notification-success"
        );
        assert_eq!(NotificationType::Error.css_class(), "notification-error");
        assert_eq!(
            NotificationType::Warning.css_class(),
            "notification-warning"
        );
        assert_eq!(NotificationType::Info.css_class(), "notification-info");
    }

    #[test]
    fn test_notification_type_default_icon() {
        assert_eq!(NotificationType::Success.default_icon(), "✓");
        assert_eq!(NotificationType::Error.default_icon(), "✕");
        assert_eq!(NotificationType::Warning.default_icon(), "⚠");
        assert_eq!(NotificationType::Info.default_icon(), "ℹ");
    }

    #[test]
    fn test_success_notification() {
        let success = SuccessNotificationProps {
            title: "Success".to_string(),
            message: "Operation completed successfully".to_string(),
            visible: Some(true),
            on_close: None,
        };

        assert_eq!(success.title, "Success");
        assert_eq!(success.message, "Operation completed successfully");
        assert_eq!(success.visible, Some(true));
    }

    #[test]
    fn test_error_notification() {
        let error = ErrorNotificationProps {
            title: "Error".to_string(),
            message: "An error occurred".to_string(),
            visible: Some(true),
            on_close: None,
        };

        assert_eq!(error.title, "Error");
        assert_eq!(error.message, "An error occurred");
        assert_eq!(error.visible, Some(true));
    }

    #[test]
    fn test_warning_notification() {
        let warning = WarningNotificationProps {
            title: "Warning".to_string(),
            message: "This is a warning".to_string(),
            visible: Some(false),
            on_close: None,
        };

        assert_eq!(warning.title, "Warning");
        assert_eq!(warning.message, "This is a warning");
        assert_eq!(warning.visible, Some(false));
    }

    #[test]
    fn test_info_notification() {
        let info = InfoNotificationProps {
            title: "Info".to_string(),
            message: "This is information".to_string(),
            visible: None,
            on_close: None,
        };

        assert_eq!(info.title, "Info");
        assert_eq!(info.message, "This is information");
        assert_eq!(info.visible, None);
    }
}
