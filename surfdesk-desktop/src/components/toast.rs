//! # Notification Toast Components
//!
//! Professional toast notification system with smooth animations,
//! stacking management, and comprehensive accessibility features
//! for the SurfDesk desktop application.

use super::css_class;
use dioxus::prelude::*;
use std::time::Duration;

/// Toast position variants
#[derive(Debug, Clone, PartialEq)]
pub enum ToastPosition {
    /// Top-right corner
    TopRight,
    /// Top-left corner
    TopLeft,
    /// Bottom-right corner
    BottomRight,
    /// Bottom-left corner
    BottomLeft,
    /// Top center
    TopCenter,
    /// Bottom center
    BottomCenter,
}

/// Toast variant types
#[derive(Debug, Clone, PartialEq)]
pub enum ToastVariant {
    /// Success toast (green)
    Success,
    /// Error toast (red)
    Error,
    /// Warning toast (yellow)
    Warning,
    /// Info toast (blue)
    Info,
}

/// Individual toast notification data
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// Unique toast ID
    pub id: String,
    /// Toast message
    pub message: String,
    /// Toast variant
    pub variant: ToastVariant,
    /// Auto-dismiss duration (None for manual only)
    pub duration: Option<Duration>,
    /// Whether to show close button
    pub show_close_button: bool,
    /// Custom action button (optional)
    pub action: Option<ToastAction>,
    /// Timestamp when created
    pub timestamp: std::time::Instant,
}

/// Toast action button
#[derive(Debug, Clone, PartialEq)]
pub struct ToastAction {
    /// Button label
    pub label: String,
    /// Action handler
    pub on_click: EventHandler<()>,
}

/// Toast manager state
#[derive(Debug, Clone)]
pub struct ToastState {
    /// Active toasts
    pub toasts: Vec<Toast>,
    /// Default position for new toasts
    pub default_position: ToastPosition,
    /// Maximum number of toasts to show
    pub max_toasts: usize,
}

impl Default for ToastState {
    fn default() -> Self {
        Self {
            toasts: Vec::new(),
            default_position: ToastPosition::TopRight,
            max_toasts: 5,
        }
    }
}

/// Toast manager props
#[derive(Debug, Clone, Props)]
pub struct ToastManagerProps {
    /// Toast position
    #[props(default = ToastPosition::TopRight)]
    pub position: ToastPosition,

    /// Maximum number of toasts
    #[props(default = 5)]
    pub max_toasts: usize,

    /// Custom CSS classes
    pub class: Option<String>,
}

/// Toast manager component
#[component]
pub fn ToastManager(props: ToastManagerProps) -> Element {
    let mut toast_state = use_signal(ToastState::default);
    let mut hovering_toast = use_signal(|| None::<String>);

    // Update default position and max toasts
    use_effect(move || {
        let mut state = toast_state.write();
        state.default_position = props.position.clone();
        state.max_toasts = props.max_toasts;
    });

    // Auto-dismiss timer
    use_coroutine(|_| {
        let toast_state = toast_state.clone();
        let hovering_toast = hovering_toast.clone();

        async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;

                let now = std::time::Instant::now();
                let mut state = toast_state.write();

                // Remove expired toasts (unless being hovered)
                state.toasts.retain(|toast| {
                    if let Some(duration) = toast.duration {
                        let elapsed = now.duration_since(toast.timestamp);
                        let is_expired = elapsed >= duration;
                        let is_hovered = hovering_toast() == Some(toast.id.clone());

                        !is_expired || is_hovered
                    } else {
                        true // Manual dismiss only
                    }
                });
            }
        }
    });

    // Global context for adding toasts
    use_context_provider(|| ToastContext {
        add_toast: {
            let toast_state = toast_state.clone();
            move |toast: Toast| {
                let mut state = toast_state.write();

                // Remove oldest toast if max reached
                if state.toasts.len() >= state.max_toasts {
                    state.toasts.remove(0);
                }

                state.toasts.push(toast);
            }
        },
        remove_toast: {
            let toast_state = toast_state.clone();
            move |id: String| {
                let mut state = toast_state.write();
                state.toasts.retain(|toast| toast.id != id);
            }
        },
        clear_all: {
            let toast_state = toast_state.clone();
            move || {
                let mut state = toast_state.write();
                state.toasts.clear();
            }
        },
    });

    let position_classes = match props.position {
        ToastPosition::TopRight => "toast-container--top-right",
        ToastPosition::TopLeft => "toast-container--top-left",
        ToastPosition::BottomRight => "toast-container--bottom-right",
        ToastPosition::BottomLeft => "toast-container--bottom-left",
        ToastPosition::TopCenter => "toast-container--top-center",
        ToastPosition::BottomCenter => "toast-container--bottom-center",
    };

    let container_classes = css_class("toast-container", &[position_classes]);
    let final_class = match &props.class {
        Some(custom) => format!("{} {}", container_classes, custom),
        None => container_classes,
    };

    rsx! {
        div {
            class: "{final_class}",

            // ARIA live region for screen readers
            div {
                "aria-live": "polite",
                "aria-atomic": "true",
                class: "sr-only",
                for toast in toast_state().toasts.iter() {
                    "{toast.message}"
                }
            }

            // Render toasts
            for toast in toast_state().toasts.iter().rev() {
                NotificationToast {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    on_hover: move |is_hovering: bool| {
                        if is_hovering {
                            hovering_toast.set(Some(toast.id.clone()));
                        } else if hovering_toast() == Some(toast.id.clone()) {
                            hovering_toast.set(None);
                        }
                    },
                    on_dismiss: move || {
                        let mut state = toast_state.write();
                        state.toasts.retain(|t| t.id != toast.id);
                    },
                }
            }
        }
    }
}

/// Context for managing toasts globally
#[derive(Clone)]
pub struct ToastContext {
    pub add_toast: Box<dyn Fn(Toast)>,
    pub remove_toast: Box<dyn Fn(String)>,
    pub clear_all: Box<dyn Fn()>,
}

/// Hook for using toast context
pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>()
}

/// Utility functions for creating toasts
pub fn use_toast_utils() -> ToastUtils {
    let toast_context = use_toast();
    ToastUtils {
        context: toast_context,
    }
}

/// Toast utility functions
pub struct ToastUtils {
    context: ToastContext,
}

impl ToastUtils {
    /// Show success toast
    pub fn success(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Success, Some(Duration::from_secs(3)));
    }

    /// Show error toast
    pub fn error(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Error, Some(Duration::from_secs(5)));
    }

    /// Show warning toast
    pub fn warning(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Warning, Some(Duration::from_secs(4)));
    }

    /// Show info toast
    pub fn info(&self, message: impl Into<String>) {
        self.show(message, ToastVariant::Info, Some(Duration::from_secs(3)));
    }

    /// Show custom toast
    pub fn show(
        &self,
        message: impl Into<String>,
        variant: ToastVariant,
        duration: Option<Duration>,
    ) {
        let toast = Toast {
            id: uuid::Uuid::new_v4().to_string(),
            message: message.into(),
            variant,
            duration,
            show_close_button: true,
            action: None,
            timestamp: std::time::Instant::now(),
        };

        (self.context.add_toast)(toast);
    }

    /// Show toast with action
    pub fn with_action(
        &self,
        message: impl Into<String>,
        variant: ToastVariant,
        duration: Option<Duration>,
        action: ToastAction,
    ) {
        let toast = Toast {
            id: uuid::Uuid::new_v4().to_string(),
            message: message.into(),
            variant,
            duration,
            show_close_button: true,
            action: Some(action),
            timestamp: std::time::Instant::now(),
        };

        (self.context.add_toast)(toast);
    }

    /// Remove specific toast
    pub fn dismiss(&self, id: &str) {
        (self.context.remove_toast)(id.to_string());
    }

    /// Clear all toasts
    pub fn clear(&self) {
        (self.context.clear_all)();
    }
}

/// Individual toast notification props
#[derive(Debug, Clone, Props)]
pub struct NotificationToastProps {
    /// Toast data
    pub toast: Toast,

    /// Hover handler
    pub on_hover: EventHandler<bool>,

    /// Dismiss handler
    pub on_dismiss: EventHandler<()>,
}

/// Individual toast notification component
#[component]
pub fn NotificationToast(props: NotificationToastProps) -> Element {
    let mut is_visible = use_signal(|| true);
    let mut progress = use_signal(|| 100.0);

    // Handle dismiss animation
    let handle_dismiss = move || {
        is_visible.set(false);
        // Use timeout for animation
        use_coroutine(|_| {
            let on_dismiss = props.on_dismiss.clone();
            async move {
                gloo_timers::callback::Timeout::new(300, move || {
                    on_dismiss.call(());
                })
                .forget();
            }
        });
    };

    // Handle click to dismiss
    let handle_click = move |evt: MouseEvent| {
        // Only dismiss if clicking on the toast itself, not action buttons
        if evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .map(|el| el.class_name().contains("toast-action"))
            .unwrap_or(false)
        {
            return; // Don't dismiss if clicking action button
        }
        handle_dismiss();
    };

    // Update progress bar for auto-dismiss
    use_effect(use_coroutine(|_| {
        let toast = props.toast.clone();
        let progress = progress.clone();
        let hovering_toast = use_signal(|| false);

        async move {
            if let Some(duration) = toast.duration {
                let total_ms = duration.as_millis() as f64;
                let update_interval = Duration::from_millis(50);
                let step = (50.0 / total_ms) * 100.0; // Progress per update

                let mut interval = tokio::time::interval(update_interval);
                let start_time = std::time::Instant::now();

                loop {
                    interval.tick().await;

                    if !hovering_toast() {
                        let elapsed = start_time.elapsed().as_millis() as f64;
                        let remaining = ((total_ms - elapsed) / total_ms * 100.0).max(0.0);
                        progress.set(remaining);

                        if remaining <= 0.0 {
                            break;
                        }
                    }
                }
            }
        }
    }));

    let variant_classes = match props.toast.variant {
        ToastVariant::Success => "success",
        ToastVariant::Error => "error",
        ToastVariant::Warning => "warning",
        ToastVariant::Info => "info",
    };

    let toast_classes = css_class("toast", &[variant_classes]);
    let final_classes = if is_visible() {
        format!("{} toast--visible", toast_classes)
    } else {
        format!("{} toast--hiding", toast_classes)
    };

    let icon = match props.toast.variant {
        ToastVariant::Success => "✓",
        ToastVariant::Error => "✕",
        ToastVariant::Warning => "⚠",
        ToastVariant::Info => "ℹ",
    };

    rsx! {
        div {
            class: "{final_classes}",
            onclick: handle_click,
            onmouseenter: move |_| props.on_hover.call(true),
            onmouseleave: move |_| props.on_hover.call(false),
            "role": "alert",
            "aria-live": "assertive",
            "data-toast-id": "{props.toast.id}",

            // Toast icon
            div { class: "toast-icon",
                span { class: "toast-icon-symbol", "{icon}" }
            }

            // Toast content
            div { class: "toast-content",
                div { class: "toast-message", "{props.toast.message}" }

                // Action button (if present)
                if let Some(action) = &props.toast.action {
                    button {
                        class: "toast-action",
                        onclick: move |evt: MouseEvent| {
                            evt.stop_propagation();
                            action.on_click.call(());
                        },
                        r#type: "button",
                        "{action.label}"
                    }
                }
            }

            // Close button
            if props.toast.show_close_button {
                button {
                    class: "toast-close",
                    onclick: move |evt: MouseEvent| {
                        evt.stop_propagation();
                        handle_dismiss();
                    },
                    r#type: "button",
                    "aria-label": "Dismiss notification",
                    "×"
                }
            }

            // Progress bar (for auto-dismiss)
            if props.toast.duration.is_some() {
                div {
                    class: "toast-progress",
                    div {
                        class: "toast-progress-bar",
                        style: "width: {progress()}%"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_creation() {
        let toast = Toast {
            id: "test-123".to_string(),
            message: "Test message".to_string(),
            variant: ToastVariant::Success,
            duration: Some(Duration::from_secs(3)),
            show_close_button: true,
            action: None,
            timestamp: std::time::Instant::now(),
        };

        assert_eq!(toast.id, "test-123");
        assert_eq!(toast.message, "Test message");
        assert_eq!(toast.variant, ToastVariant::Success);
    }

    #[test]
    fn test_toast_position_classes() {
        assert_eq!(
            css_class("toast-container", &["toast-container--top-right"]),
            "toast-container toast-container--top-right"
        );
    }

    #[test]
    fn test_toast_variant_classes() {
        let variants = vec![
            (ToastVariant::Success, "success"),
            (ToastVariant::Error, "error"),
            (ToastVariant::Warning, "warning"),
            (ToastVariant::Info, "info"),
        ];

        for (variant, expected_class) in variants {
            assert_eq!(
                css_class("toast", &[expected_class]),
                format!("toast toast--{}", expected_class)
            );
        }
    }
}
