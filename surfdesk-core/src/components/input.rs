//! # Input Component
//!
//! Comprehensive input component with various input types,
//! validation, and accessibility features for all platforms.

use crate::components::{combine_classes, Color, CommonProps, Size, Variant};
use dioxus::prelude::*;

/// Input component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Input value
    #[props(optional)]
    pub value: Option<String>,

    /// Input placeholder
    #[props(optional)]
    pub placeholder: Option<String>,

    /// Input type
    #[props(optional)]
    pub input_type: Option<InputType>,

    /// Input variant
    #[props(optional)]
    pub variant: Option<Variant>,

    /// Input size
    #[props(optional)]
    pub size: Option<Size>,

    /// Input color
    #[props(optional)]
    pub color: Option<Color>,

    /// Whether input is disabled
    #[props(optional)]
    pub disabled: Option<bool>,

    /// Whether input is readonly
    #[props(optional)]
    pub readonly: Option<bool>,

    /// Whether input is required
    #[props(optional)]
    pub required: Option<bool>,

    /// Whether input has error
    #[props(optional)]
    pub error: Option<bool>,

    /// Error message
    #[props(optional)]
    pub error_message: Option<String>,

    /// Help text
    #[props(optional)]
    pub help_text: Option<String>,

    /// Input label
    #[props(optional)]
    pub label: Option<String>,

    /// Whether to show clear button
    #[props(optional)]
    pub clearable: Option<bool>,

    /// Input prefix (text or icon)
    #[props(optional)]
    pub prefix: Option<String>,

    /// Input suffix (text or icon)
    #[props(optional)]
    pub suffix: Option<String>,

    /// Maximum length
    #[props(optional)]
    pub max_length: Option<usize>,

    /// Minimum length
    #[props(optional)]
    pub min_length: Option<usize>,

    /// Pattern for validation
    #[props(optional)]
    pub pattern: Option<String>,

    /// Change handler
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,

    /// Focus handler
    #[props(optional)]
    pub on_focus: Option<EventHandler<()>>,

    /// Blur handler
    #[props(optional)]
    pub on_blur: Option<EventHandler<()>>,

    /// Key press handler
    #[props(optional)]
    pub on_keypress: Option<EventHandler<KeyboardEvent>>,

    /// Input mode for mobile keyboards
    #[props(optional)]
    pub input_mode: Option<InputMode>,

    /// Autocomplete setting
    #[props(optional)]
    pub autocomplete: Option<String>,

    /// Whether input is auto-focused
    #[props(optional)]
    pub autofocus: Option<bool>,
}

/// Input types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTime,
    Month,
    Week,
    Color,
    Range,
    File,
    Hidden,
}

impl InputType {
    /// Get HTML input type string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Color => "color",
            Self::Range => "range",
            Self::File => "file",
            Self::Hidden => "hidden",
        }
    }
}

/// Input modes for mobile keyboards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Text,
    Decimal,
    Numeric,
    Tel,
    Email,
    Url,
    Search,
    None,
}

impl InputMode {
    /// Get HTML inputmode string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Decimal => "decimal",
            Self::Numeric => "numeric",
            Self::Tel => "tel",
            Self::Email => "email",
            Self::Url => "url",
            Self::Search => "search",
            Self::None => "none",
        }
    }
}

/// Input component
#[component]
pub fn Input(props: InputProps) -> Element {
    let common = props.common.unwrap_or_default();
    let value = props.value.unwrap_or_default();
    let input_type = props.input_type.unwrap_or(InputType::Text);
    let variant = props.variant.unwrap_or(Variant::Default);
    let size = props.size.unwrap_or(Size::Medium);
    let color = props.color.unwrap_or(Color::Primary);
    let disabled = props.disabled.unwrap_or(false);
    let readonly = props.readonly.unwrap_or(false);
    let required = props.required.unwrap_or(false);
    let error = props.error.unwrap_or(false);
    let clearable = props.clearable.unwrap_or(false);
    let autofocus = props.autofocus.unwrap_or(false);

    let mut input_value = use_signal(|| value.clone());
    let mut is_focused = use_signal(|| false);

    // Update signal when prop changes
    use_effect(use_reactive(&value, move |val| {
        input_value.set(val);
    }));

    let mut classes = vec!["input"];
    classes.push(variant.css_class());
    classes.push(size.css_class());
    classes.push(color.css_class());

    if disabled {
        classes.push("input-disabled");
    }

    if readonly {
        classes.push("input-readonly");
    }

    if required {
        classes.push("input-required");
    }

    if error {
        classes.push("input-error");
    }

    if *is_focused.read() {
        classes.push("input-focused");
    }

    if let Some(class) = &common.class {
        classes.push(class);
    }

    let handle_change = move |evt: FormEvent| {
        let new_value = evt.value();
        input_value.set(new_value.clone());

        if let Some(on_change) = props.on_change {
            on_change(new_value);
        }
    };

    let handle_focus = move |_| {
        is_focused.set(true);
        if let Some(on_focus) = props.on_focus {
            on_focus(());
        }
    };

    let handle_blur = move |_| {
        is_focused.set(false);
        if let Some(on_blur) = props.on_blur {
            on_blur(());
        }
    };

    let handle_clear = move |_| {
        input_value.set(String::new());
        if let Some(on_change) = props.on_change {
            on_change(String::new());
        }
    };

    let handle_keypress = move |evt: KeyboardEvent| {
        if let Some(on_keypress) = props.on_keypress {
            on_keypress(evt);
        }
    };

    rsx! {
        div { class: combine_classes(&["input-wrapper", &classes.join(" ")]),
            id: common.id,

            // Label
            if let Some(label) = props.label {
                label {
                    class: "input-label",
                    for: "input-{common.id.as_deref().unwrap_or(\"default\")}",
                    "{label}"
                    if required {
                        span { class: "input-required-indicator", " *" }
                    }
                }
            }

            // Input container
            div { class: "input-container",
                // Prefix
                if let Some(prefix) = props.prefix {
                    span { class: "input-prefix", "{prefix}" }
                }

                // Actual input
                input {
                    r#type: input_type.as_str(),
                    class: "input-field",
                    id: "input-{common.id.as_deref().unwrap_or(\"default\")}",
                    value: "{input_value}",
                    placeholder: props.placeholder,
                    disabled: disabled,
                    readonly: readonly,
                    required: required,
                    maxlength: props.max_length,
                    minlength: props.min_length,
                    pattern: props.pattern,
                    autofocus: autofocus,
                    autocomplete: props.autocomplete,
                    inputmode: props.input_mode.map(|m| m.as_str()),
                    onchange: handle_change,
                    oninput: handle_change,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                    onkeypress: handle_keypress,
                }

                // Suffix
                if let Some(suffix) = props.suffix {
                    span { class: "input-suffix", "{suffix}" }
                }

                // Clear button
                if clearable && !input_value.read().is_empty() && !disabled && !readonly {
                    button {
                        class: "input-clear",
                        type: "button",
                        onclick: handle_clear,
                        "×"
                    }
                }
            }

            // Error message
            if error {
                if let Some(error_message) = props.error_message {
                    div { class: "input-error-message", "{error_message}" }
                }
            }

            // Help text
            if let Some(help_text) = props.help_text {
                div { class: "input-help-text", "{help_text}" }
            }
        }
    }
}

/// Text input component
#[component]
pub fn TextInput(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Input {
            input_type: InputType::Text,
            value: value,
            placeholder: placeholder,
            on_change: on_change,
            class: class,
        }
    }
}

/// Email input component
#[component]
pub fn EmailInput(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Input {
            input_type: InputType::Email,
            value: value,
            placeholder: placeholder,
            on_change: on_change,
            class: class,
        }
    }
}

/// Password input component
#[component]
pub fn PasswordInput(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Input {
            input_type: InputType::Password,
            value: value,
            placeholder: placeholder,
            on_change: on_change,
            class: class,
        }
    }
}

/// Number input component
#[component]
pub fn NumberInput(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    #[props(optional)] min: Option<f64>,
    #[props(optional)] max: Option<f64>,
    #[props(optional)] step: Option<f64>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Input {
            input_type: InputType::Number,
            value: value,
            placeholder: placeholder,
            on_change: on_change,
            class: class,
        }
    }
}

/// Search input component
#[component]
pub fn SearchInput(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] on_change: Option<EventHandler<String>>,
    #[props(optional)] on_search: Option<EventHandler<String>>,
    #[props(optional)] class: Option<String>,
) -> Element {
    let handle_search = move |evt: KeyboardEvent| {
        if evt.key() == "Enter" {
            if let Some(on_search) = on_search {
                on_search(value.clone());
            }
        }
    };

    rsx! {
        Input {
            input_type: InputType::Search,
            value: value,
            placeholder: placeholder,
            on_change: on_change,
            on_keypress: Some(handle_search),
            class: class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_default_props() {
        let props = InputProps {
            common: None,
            value: None,
            placeholder: None,
            input_type: None,
            variant: None,
            size: None,
            color: None,
            disabled: None,
            readonly: None,
            required: None,
            error: None,
            error_message: None,
            help_text: None,
            label: None,
            clearable: None,
            prefix: None,
            suffix: None,
            max_length: None,
            min_length: None,
            pattern: None,
            on_change: None,
            on_focus: None,
            on_blur: None,
            on_keypress: None,
            input_mode: None,
            autocomplete: None,
            autofocus: None,
        };

        assert_eq!(props.value.unwrap_or_default(), "");
        assert_eq!(props.input_type.unwrap_or(InputType::Text), InputType::Text);
        assert_eq!(props.variant.unwrap_or(Variant::Default), Variant::Default);
        assert_eq!(props.size.unwrap_or(Size::Medium), Size::Medium);
        assert_eq!(props.color.unwrap_or(Color::Primary), Color::Primary);
        assert!(!props.disabled.unwrap_or(false));
        assert!(!props.readonly.unwrap_or(false));
        assert!(!props.required.unwrap_or(false));
        assert!(!props.error.unwrap_or(false));
        assert!(!props.clearable.unwrap_or(false));
        assert!(!props.autofocus.unwrap_or(false));
    }

    #[test]
    fn test_input_type_as_str() {
        assert_eq!(InputType::Text.as_str(), "text");
        assert_eq!(InputType::Email.as_str(), "email");
        assert_eq!(InputType::Password.as_str(), "password");
        assert_eq!(InputType::Number.as_str(), "number");
        assert_eq!(InputType::Search.as_str(), "search");
    }

    #[test]
    fn test_input_mode_as_str() {
        assert_eq!(InputMode::Text.as_str(), "text");
        assert_eq!(InputMode::Decimal.as_str(), "decimal");
        assert_eq!(InputMode::Numeric.as_str(), "numeric");
        assert_eq!(InputMode::Email.as_str(), "email");
    }

    #[test]
    fn test_text_input() {
        let input = TextInput {
            value: "test".to_string(),
            placeholder: Some("Enter text".to_string()),
            on_change: None,
            class: Some("custom-class".to_string()),
        };

        assert_eq!(input.value, "test");
        assert_eq!(input.placeholder, Some("Enter text".to_string()));
        assert_eq!(input.class, Some("custom-class".to_string()));
    }

    #[test]
    fn test_email_input() {
        let input = EmailInput {
            value: "test@example.com".to_string(),
            placeholder: None,
            on_change: None,
            class: None,
        };

        assert_eq!(input.value, "test@example.com");
    }

    #[test]
    fn test_password_input() {
        let input = PasswordInput {
            value: "secret".to_string(),
            placeholder: Some("Enter password".to_string()),
            on_change: None,
            class: None,
        };

        assert_eq!(input.value, "secret");
        assert_eq!(input.placeholder, Some("Enter password".to_string()));
    }

    #[test]
    fn test_number_input() {
        let input = NumberInput {
            value: "42".to_string(),
            placeholder: None,
            on_change: None,
            min: Some(0.0),
            max: Some(100.0),
            step: Some(1.0),
            class: None,
        };

        assert_eq!(input.value, "42");
        assert_eq!(input.min, Some(0.0));
        assert_eq!(input.max, Some(100.0));
        assert_eq!(input.step, Some(1.0));
    }

    #[test]
    fn test_search_input() {
        let input = SearchInput {
            value: "search query".to_string(),
            placeholder: Some("Search...".to_string()),
            on_change: None,
            on_search: None,
            class: None,
        };

        assert_eq!(input.value, "search query");
        assert_eq!(input.placeholder, Some("Search...".to_string()));
    }
}
