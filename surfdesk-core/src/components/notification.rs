//! # Notification Component
//!
//! Notification component for displaying alerts, messages, and
//! toast notifications to users across all platforms.

use crate::components::{combine_classes, Color, CommonProps, Size};
use dioxus::prelude::*;

/// Notification component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NotificationProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

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

    /// Auto-dismiss timeout in milliseconds
    #[props(optional)]
    pub auto_dismiss: Option<u64>,

    /// Whether to show icon
    #[props(optional)]
    pub show_icon: Option<bool>,

    /// Close handler
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,

    /// Action handler
    #[props(optional)]
    pub on_action: Option<EventHandler<()>>,

    /// Action button text
    #[props(optional)]
    pub action_text: Option<String>,

    /// Custom icon
    #[props(optional)]
    pub icon: Option<String>,
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

/// Notification component
#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let common = props.common.unwrap_or_default();
    let notification_type = props.notification_type.unwrap_or(NotificationType::Info);
    let size = props.size.unwrap_or(Size::Medium);
    let visible = props.visible.unwrap_or(true);
    let dismissible = props.dismissible.unwrap_or(true);
    let show_icon = props.show_icon.unwrap_or(true);

    if !visible {
        return rsx! { "" };
    }

    let mut classes = vec!["notification"];
    classes.push(notification_type.css_class());
    classes.push(size.css_class());

    if dismissible {
        classes.push("notification-dismissible");
    }

    if let Some(class) = &common.class {
        classes.push(class);
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

    // Auto-dismiss timer
    if let Some(timeout) = props.auto_dismiss {
        use_effect(move || {
            spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
                if let Some(on_close) = props.on_close {
                    on_close(());
                }
            });
        });
    }

    rsx! {
        div {
            class: combine_classes(&classes),
            id: common.id,

            // Icon
            if show_icon {
                span { class: "notification-icon",
                    "{props.icon.unwrap_or_else(|| notification_type.default_icon().to_string())}"
                }
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

/// Success notification
#[component]
pub fn SuccessNotification(
    message: String,
    #[props(optional)] title: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] auto_dismiss: Option<u64>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Notification {
            title: title,
            message: message,
            notification_type: NotificationType::Success,
            on_close: on_close,
            auto_dismiss: auto_dismiss,
            class: class,
        }
    }
}

/// Error notification
#[component]
pub fn ErrorNotification(
    message: String,
    #[props(optional)] title: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] auto_dismiss: Option<u64>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Notification {
            title: title,
            message: message,
            notification_type: NotificationType::Error,
            on_close: on_close,
            auto_dismiss: auto_dismiss,
            class: class,
        }
    }
}

/// Warning notification
#[component]
pub fn WarningNotification(
    message: String,
    #[props(optional)] title: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] auto_dismiss: Option<u64>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Notification {
            title: title,
            message: message,
            notification_type: NotificationType::Warning,
            on_close: on_close,
            auto_dismiss: auto_dismiss,
            class: class,
        }
    }
}

/// Info notification
#[component]
pub fn InfoNotification(
    message: String,
    #[props(optional)] title: Option<String>,
    #[props(optional)] on_close: Option<EventHandler<()>>,
    #[props(optional)] auto_dismiss: Option<u64>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Notification {
            title: title,
            message: message,
            notification_type: NotificationType::Info,
            on_close: on_close,
            auto_dismiss: auto_dismiss,
            class: class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_default_props() {
        let props = NotificationProps {
            common: None,
            title: None,
            message: None,
            notification_type: None,
            size: None,
            visible: None,
            dismissible: None,
            auto_dismiss: None,
            show_icon: None,
            on_close: None,
            on_action: None,
            action_text: None,
            icon: None,
        };

        assert_eq!(
            props.notification_type.unwrap_or(NotificationType::Info),
            NotificationType::Info
        );
        assert_eq!(props.size.unwrap_or(Size::Medium), Size::Medium);
        assert!(props.visible.unwrap_or(true));
        assert!(props.dismissible.unwrap_or(true));
        assert!(props.show_icon.unwrap_or(true));
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
        let notification = SuccessNotification {
            message: "Success!".to_string(),
            title: Some("Operation Complete".to_string()),
            on_close: None,
            auto_dismiss: Some(3000),
            class: Some("success-notif".to_string()),
        };

        assert_eq!(notification.message, "Success!");
        assert_eq!(notification.title, Some("Operation Complete".to_string()));
        assert_eq!(notification.auto_dismiss, Some(3000));
        assert_eq!(notification.class, Some("success-notif".to_string()));
    }

    #[test]
    fn test_error_notification() {
        let notification = ErrorNotification {
            message: "Error occurred".to_string(),
            title: None,
            on_close: None,
            auto_dismiss: None,
            class: None,
        };

        assert_eq!(notification.message, "Error occurred");
        assert!(notification.title.is_none());
    }

    #[test]
    fn test_warning_notification() {
        let notification = WarningNotification {
            message: "Warning message".to_string(),
            title: Some("Warning".to_string()),
            on_close: None,
            auto_dismiss: Some(5000),
            class: None,
        };

        assert_eq!(notification.message, "Warning message");
        assert_eq!(notification.title, Some("Warning".to_string()));
        assert_eq!(notification.auto_dismiss, Some(5000));
    }

    #[test]
    fn test_info_notification() {
        let notification = InfoNotification {
            message: "Info message".to_string(),
            title: None,
            on_close: None,
            auto_dismiss: None,
            class: Some("info".to_string()),
        };

        assert_eq!(notification.message, "Info message");
        assert_eq!(notification.class, Some("info".to_string()));
    }
}
