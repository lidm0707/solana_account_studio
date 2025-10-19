//! Common UI Components for Surfdesk
//!
//! This module provides reusable UI components that are used throughout
//! the application, including buttons, cards, inputs, modals, and other
//! common interface elements.

use dioxus::prelude::*;

/// Button component with multiple variants and states
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct ButtonProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "primary".to_string())]
    pub variant: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    #[props(default = "medium".to_string())]
    pub size: String,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub href: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = format!(
        "btn btn--{} btn--{} {} {}",
        props.variant,
        props.size,
        if props.disabled { "btn--disabled" } else { "" },
        if props.loading { "btn--loading" } else { "" }
    );

    rsx! {
        if let Some(href) = props.href {
            a {
                href: "{href}",
                class: "{class}",
                ..props.attributes,
                {props.children}
            }
        } else {
            button {
                class: "{class}",
                disabled: props.disabled || props.loading,
                onclick: move |evt| {
                    if let Some(handler) = props.onclick {
                        handler.call(evt);
                    }
                },
                ..props.attributes,
                if props.loading {
                    div { class: "btn-spinner" }
                }
                {props.children}
            }
        }
    }
}

/// Card component for containing content
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct CardProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "default".to_string())]
    pub variant: String,
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub subtitle: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_class = format!("card card--{}", props.variant);
    let class = if let Some(extra_class) = props.class {
        format!("{} {}", base_class, extra_class)
    } else {
        base_class
    };

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,

            if let Some(title) = props.title {
                div { class: "card__header",
                    h3 { class: "card__title", "{title}" }
                    if let Some(subtitle) = props.subtitle {
                        p { class: "card__subtitle", "{subtitle}" }
                    }
                }
            }

            div { class: "card__content",
                {props.children}
            }
        }
    }
}

/// Input component with validation support
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct InputProps {
    #[props(default = "text".to_string())]
    pub input_type: String,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default = false)]
    pub readonly: bool,
    #[props(default)]
    pub error: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let has_error = props.error.is_some();
    let input_class = format!(
        "input {} {}",
        if has_error { "input--error" } else { "" },
        if props.disabled {
            "input--disabled"
        } else {
            ""
        }
    );

    rsx! {
        div { class: "input-wrapper",
            input {
                r#type: "{props.input_type}",
                class: "{input_class}",
                placeholder: props.placeholder.unwrap_or_default(),
                value: props.value.unwrap_or_default(),
                disabled: props.disabled,
                required: props.required,
                readonly: props.readonly,
                oninput: move |evt| {
                    if let Some(handler) = props.oninput {
                        handler.call(evt);
                    }
                },
                onchange: move |evt| {
                    if let Some(handler) = props.onchange {
                        handler.call(evt);
                    }
                },
                ..props.attributes
            }

            if let Some(error) = props.error {
                div { class: "input-error",
                    span { class: "input-error-icon", "⚠️" }
                    span { class: "input-error-text", "{error}" }
                }
            }
        }
    }
}

/// Modal component for dialogs and overlays
#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    #[props(default)]
    pub children: Element,
    #[props(default = false)]
    pub visible: bool,
    #[props(default)]
    pub onclose: Option<EventHandler<MouseEvent>>,
    #[props(default = false)]
    pub close_on_outside_click: bool,
    #[props(default)]
    pub title: Option<String>,
    #[props(default = "medium".to_string())]
    pub size: String,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    rsx! {
        if props.visible {
            div {
                class: "modal-overlay",
                onclick: move |evt| {
                    if props.close_on_outside_click {
                        if let Some(handler) = props.onclose {
                            handler.call(evt);
                        }
                    }
                },

                div {
                    class: "modal modal--{props.size}",
                    onclick: move |evt| evt.stop_propagation(),

                    if let Some(title) = props.title {
                        div { class: "modal-header",
                            h2 { class: "modal-title", "{title}" }
                            button {
                                class: "modal-close",
                                onclick: move |evt| {
                                    if let Some(handler) = props.onclose {
                                        handler.call(evt);
                                    }
                                },
                                "✕"
                            }
                        }
                    }

                    div { class: "modal-body",
                        {props.children}
                    }
                }
            }
        }
    }
}

/// Loading spinner component
#[derive(Props, PartialEq, Clone)]
pub struct LoadingSpinnerProps {
    #[props(default = "Loading...".to_string())]
    pub message: String,
    #[props(default = "medium".to_string())]
    pub size: String,
}

#[component]
pub fn LoadingSpinner(props: LoadingSpinnerProps) -> Element {
    rsx! {
        div { class: "loading-spinner loading-spinner--{props.size}",
            div { class: "spinner-circle" }
            if !props.message.is_empty() {
                p { class: "loading-message", "{props.message}" }
            }
        }
    }
}

/// Status indicator component
#[derive(Props, PartialEq, Clone)]
pub struct StatusIndicatorProps {
    pub status: String, // success, warning, error, info
    #[props(default)]
    pub message: Option<String>,
    #[props(default = "small".to_string())]
    pub size: String,
}

#[component]
pub fn StatusIndicator(props: StatusIndicatorProps) -> Element {
    let icon = match props.status.as_str() {
        "success" => "✅",
        "warning" => "⚠️",
        "error" => "❌",
        "info" => "ℹ️",
        _ => "⚪",
    };

    rsx! {
        div { class: "status-indicator status-indicator--{props.status} status-indicator--{props.size}",
            span { class: "status-icon", "{icon}" }
            if let Some(message) = props.message {
                span { class: "status-message", "{message}" }
            }
        }
    }
}

/// Badge component for status labels
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct BadgeProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "default".to_string())]
    pub variant: String,
    #[props(default = "small".to_string())]
    pub size: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class = format!("badge badge--{} badge--{}", props.variant, props.size);

    rsx! {
        span {
            class: "{class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Progress bar component
#[derive(Props, PartialEq, Clone)]
pub struct ProgressBarProps {
    pub value: f64, // 0.0 to 1.0
    #[props(default = false)]
    pub show_percentage: bool,
    #[props(default = "medium".to_string())]
    pub size: String,
    #[props(default)]
    pub label: Option<String>,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let percentage = (props.value * 100.0).round();
    let clamped_value = props.value.clamp(0.0, 1.0);

    rsx! {
        div { class: "progress-bar progress-bar--{props.size}",
            if let Some(label) = props.label {
                div { class: "progress-label", "{label}" }
            }

            div { class: "progress-track",
                div {
                    class: "progress-fill",
                    style: "width: {clamped_value * 100.0}%",
                }

                if props.show_percentage {
                    span { class: "progress-percentage", "{percentage}%" }
                }
            }
        }
    }
}

/// Icon component
#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    pub name: String,
    #[props(default = "medium".to_string())]
    pub size: String,
    #[props(default)]
    pub color: Option<String>,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx! {
        span {
            class: "icon icon--{props.size}",
            style: if let Some(color) = props.color {
                format!("color: {};", color)
            } else {
                String::new()
            },
            "{props.name}"
        }
    }
}

/// Divider component
#[derive(Props, PartialEq, Clone)]
pub struct DividerProps {
    #[props(default = false)]
    pub vertical: bool,
    #[props(default = "solid".to_string())]
    pub style: String,
    #[props(default)]
    pub label: Option<String>,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    let orientation = if props.vertical {
        "vertical"
    } else {
        "horizontal"
    };

    rsx! {
        if let Some(label) = props.label {
            div { class: "divider divider--with-label",
                hr { class: "divider-line" }
                span { class: "divider-label", "{label}" }
                hr { class: "divider-line" }
            }
        } else {
            div {
                class: "divider divider--{orientation} divider--{props.style}"
            }
        }
    }
}

/// Tooltip component
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    #[props(default)]
    pub children: Element,
    pub content: String,
    #[props(default = "top".to_string())]
    pub position: String,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        div { class: "tooltip-wrapper",
            div { class: "tooltip-trigger",
                {props.children}
            }
            div { class: "tooltip tooltip--{props.position}",
                "{props.content}"
                div { class: "tooltip-arrow" }
            }
        }
    }
}

/// Alert component for notifications
#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    #[props(default)]
    pub children: Element,
    pub variant: String, // success, warning, error, info
    #[props(default)]
    pub on_dismiss: Option<EventHandler<MouseEvent>>,
    #[props(default = false)]
    pub dismissible: bool,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let icon = match props.variant.as_str() {
        "success" => "✅",
        "warning" => "⚠️",
        "error" => "❌",
        "info" => "ℹ️",
        _ => "ℹ️",
    };

    rsx! {
        div { class: "alert alert--{props.variant}",
            span { class: "alert-icon", "{icon}" }
            div { class: "alert-content",
                {props.children}
            }
            if props.dismissible {
                button {
                    class: "alert-dismiss",
                    onclick: move |evt| {
                        if let Some(handler) = props.on_dismiss {
                            handler.call(evt);
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

/// Empty state component
#[derive(Props, PartialEq, Clone)]
pub struct EmptyStateProps {
    pub title: String,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub action: Option<Element>,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        div { class: "empty-state",
            if let Some(icon) = props.icon {
                div { class: "empty-state-icon", "{icon}" }
            }
            h3 { class: "empty-state-title", "{props.title}" }
            if let Some(description) = props.description {
                p { class: "empty-state-description", "{description}" }
            }
            if let Some(action) = props.action {
                div { class: "empty-state-action",
                    {action}
                }
            }
        }
    }
}
