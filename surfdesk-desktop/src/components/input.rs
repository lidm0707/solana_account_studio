//! # Input Components Module
//!
//! Professional input components for the SurfDesk desktop application.
//! Provides text, password, and number inputs with validation, styling,
//! and accessibility features.

use super::Size;
use dioxus::prelude::*;
use std::str::FromStr;

/// Input component props
#[derive(Debug, Clone, Props)]
pub struct InputProps {
    /// Input type
    #[props(default = InputType::Text)]
    pub input_type: InputType,

    /// Input size
    #[props(default = Size::Medium)]
    pub size: Size,

    /// Input placeholder
    pub placeholder: Option<String>,

    /// Current value
    #[props(default = String::new())]
    pub value: String,

    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the input is in error state
    #[props(default = false)]
    pub error: bool,

    /// Error message to display
    pub error_message: Option<String>,

    /// Whether the input is required
    #[props(default = false)]
    pub required: bool,

    /// Maximum length for text inputs
    pub max_length: Option<usize>,

    /// Minimum value for number inputs
    pub min_value: Option<f64>,

    /// Maximum value for number inputs
    pub max_value: Option<f64>,

    /// Input label
    pub label: Option<String>,

    /// Helper text
    pub helper_text: Option<String>,

    /// On change handler
    pub onchange: EventHandler<String>,

    /// On focus handler
    pub onfocus: EventHandler<FocusEvent>,

    /// On blur handler
    pub onblur: EventHandler<FocusEvent>,

    /// On key press handler
    pub onkeypress: EventHandler<KeyboardEvent>,
}

/// Input types
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    /// Text input
    Text,
    /// Password input
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
    /// Search input
    Search,
    /// URL input
    Url,
    /// Tel input
    Tel,
    /// Date input
    Date,
    /// Time input
    Time,
}

/// Input validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// Input is valid
    Valid,
    /// Input is invalid with error message
    Invalid(String),
}

/// Professional input component
#[component]
pub fn Input(props: InputProps) -> Element {
    let mut show_password = use_signal(|| false);
    let mut is_focused = use_signal(|| false);

    // Handle focus events
    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        props.onfocus.call(evt);
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        props.onblur.call(evt);
    };

    // Handle password visibility toggle
    let toggle_password = move |_| {
        show_password.set(!show_password());
    };

    // Build CSS classes
    let base_class = "surf-input";
    let mut classes = vec![base_class.to_string()];

    // Size modifier
    let size_class = match props.size {
        Size::Small => "surf-input--small",
        Size::Medium => "surf-input--medium",
        Size::Large => "surf-input--large",
    };
    classes.push(size_class);

    // Type modifier
    let type_class = match props.input_type {
        InputType::Password => "surf-input--password",
        InputType::Number => "surf-input--number",
        InputType::Email => "surf-input--email",
        InputType::Search => "surf-input--search",
        InputType::Url => "surf-input--url",
        _ => "surf-input--text",
    };
    classes.push(type_class);

    // State modifiers
    if props.disabled {
        classes.push("surf-input--disabled");
    }
    if props.error {
        classes.push("surf-input--error");
    }
    if is_focused() {
        classes.push("surf-input--focused");
    }
    if props.required {
        classes.push("surf-input--required");
    }

    let css_classes = classes.join(" ");

    // Determine input type and attributes
    let (input_type, input_value, extra_attrs) = match props.input_type {
        InputType::Password => (
            if show_password() { "text" } else { "password" },
            if show_password() {
                props.value.clone()
            } else {
                "•".repeat(props.value.len())
            },
            vec![],
        ),
        InputType::Number => (
            "number",
            props.value.clone(),
            vec![
                if let Some(min) = props.min_value {
                    format!("min=\"{}\"", min)
                } else {
                    String::new()
                },
                if let Some(max) = props.max_value {
                    format!("max=\"{}\"", max)
                } else {
                    String::new()
                },
                "step=\"any\"".to_string(),
            ],
        ),
        _ => (
            match props.input_type {
                InputType::Email => "email",
                InputType::Search => "search",
                InputType::Url => "url",
                InputType::Tel => "tel",
                InputType::Date => "date",
                InputType::Time => "time",
                _ => "text",
            },
            props.value.clone(),
            vec![],
        ),
    };

    rsx! {
        div { class: "surf-input-wrapper",

            // Label
            if let Some(label) = &props.label {
                label { class: "surf-input-label",
                    "{label}"
                    {if props.required { rsx! { span { class: "required-indicator", "*" } } }}
                }
            }

            // Input container
            div { class: "surf-input-container",
                input {
                    r#type: "{input_type}",
                    class: "{css_classes}",
                    placeholder: props.placeholder.clone().unwrap_or_default(),
                    value: "{input_value}",
                    disabled: props.disabled,
                    required: props.required,
                    maxlength: props.max_length,
                    onfocus: handle_focus,
                    onblur: handle_blur,
                    oninput: move |evt| {
                        let new_value = evt.value();
                        props.onchange.call(new_value);
                    },
                    onkeypress: props.onkeypress,
                }
            }

                // Password visibility toggle
                if matches!(props.input_type, InputType::Password) {
                    button {
                        class: "surf-input-toggle",
                        r#type: "button",
                        r#disabled: props.disabled,
                        onclick: toggle_password,
                        r#tabindex: "-1",

                        span {
                            class: "toggle-icon",
                            if show_password() { "👁️" } else { "👁️" }
                        }
                    }
                }
            }

            // Helper text
            if let Some(helper_text) = &props.helper_text {
                div { class: "surf-input-helper",
                    "{helper_text}"
                }
            }

            // Error message
            if props.error {
                if let Some(error_message) = &props.error_message {
                    div { class: "surf-input-error",
                        "{error_message}"
                    }
                }
            }
        }
    }
}

/// Textarea component for multi-line input
#[derive(Debug, Clone, Props)]
pub struct TextareaProps {
    /// Textarea value
    #[props(default = String::new())]
    pub value: String,

    /// Placeholder text
    pub placeholder: Option<String>,

    /// Number of rows
    #[props(default = 4)]
    pub rows: usize,

    /// Maximum number of characters
    pub max_length: Option<usize>,

    /// Whether the textarea is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the textarea is in error state
    #[props(default = false)]
    pub error: bool,

    /// Error message
    pub error_message: Option<String>,

    /// Whether the textarea is required
    #[props(default = false)]
    pub required: bool,

    /// On change handler
    pub onchange: EventHandler<String>,

    /// On focus handler
    pub onfocus: EventHandler<FocusEvent>,

    /// On blur handler
    pub onblur: EventHandler<FocusEvent>,
}

/// Textarea component
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let mut is_focused = use_signal(|| false);

    let handle_focus = move |evt: FocusEvent| {
        is_focused.set(true);
        props.onfocus.call(evt);
    };

    let handle_blur = move |evt: FocusEvent| {
        is_focused.set(false);
        props.onblur.call(evt);
    };

    let base_class = "surf-textarea";
    let mut classes = vec![base_class.to_string()];

    if props.disabled {
        classes.push("surf-textarea--disabled");
    }
    if props.error {
        classes.push("surf-textarea--error");
    }
    if is_focused() {
        classes.push("surf-textarea--focused");
    }
    if props.required {
        classes.push("surf-textarea--required");
    }

    let css_classes = classes.join(" ");

    rsx! {
        div { class: "surf-textarea-wrapper",
            textarea {
                class: "{css_classes}",
                placeholder: props.placeholder.clone().unwrap_or_default(),
                rows: props.rows,
                value: props.value,
                disabled: props.disabled,
                required: props.required,
                maxlength: props.max_length,
                onfocus: handle_focus,
                onblur: handle_blur,
                oninput: move |evt| {
                    let new_value = evt.value();
                    props.onchange.call(new_value);
                }
            }
        }
    }
}

/// Input group for combining inputs with labels and actions
#[derive(Debug, Clone, Props)]
pub struct InputGroupProps {
    /// Group label
    pub label: Option<String>,

    /// Group description
    pub description: Option<String>,

    /// Whether the group is required
    #[props(default = false)]
    pub required: bool,

    /// Group content
    pub children: Element,
}

/// Input group component
#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    rsx! {
        div { class: "surf-input-group",
            // Group header
            if let Some(label) = &props.label {
                div { class: "surf-input-group-header",
                    h3 { class: "surf-input-group-label",
                        "{label}"
                        {if props.required { rsx! { span { class: "required-indicator", "*" } } }}
                    }
                }
            }

            // Group description
            if let Some(description) = &props.description {
                div { class: "surf-input-group-description",
                    "{description}"
                }
            }

            // Group content
            div { class: "surf-input-group-content",
                {props.children}
            }
        }
    }
}

/// Form field component (input + label combined)
#[derive(Debug, Clone, Props)]
pub struct FieldProps {
    /// Field label
    pub label: String,

    /// Field name (for form submission)
    pub name: String,

    /// Input type
    #[props(default = InputType::Text)]
    pub input_type: InputType,

    /// Field value
    #[props(default = String::new())]
    pub value: String,

    /// Placeholder text
    pub placeholder: Option<String>,

    /// Whether the field is required
    #[props(default = false)]
    pub required: bool,

    /// Whether the field is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the field is in error state
    #[props(default = false)]
    pub error: bool,

    /// Error message
    pub error_message: Option<String>,

    /// Helper text
    pub helper_text: Option<String>,

    /// On change handler
    pub onchange: EventHandler<String>,
}

/// Form field component
#[component]
pub fn Field(props: FieldProps) -> Element {
    let field_id = use_signal(|| props.name.clone());

    rsx! {
        InputGroup {
            label: Some(props.label),
            required: props.required,
            Input {
                r#input_type: props.input_type,
                r#name: props.name,
                r#value: props.value,
                r#placeholder: props.placeholder,
                r#required: props.required,
                r#disabled: props.disabled,
                r#error: props.error,
                r#error_message: props.error_message,
                r#onchange: props.onchange,
            }
        }
    }
}

/// Utility function for email validation
pub fn validate_email(email: &str) -> ValidationResult {
    // Simple email validation - placeholder implementation
    if email.contains('@') && email.contains('.') {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid("Please enter a valid email address".to_string())
    }
}

/// Utility function for URL validation
pub fn validate_url(url: &str) -> ValidationResult {
    // Simple URL validation - placeholder implementation
    if url.starts_with("http://") || url.starts_with("https://") {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(
            "Please enter a valid URL (starting with http:// or https://)".to_string(),
        )
    }
}

/// Utility function for number validation
pub fn validate_number(value: &str, min: Option<f64>, max: Option<f64>) -> ValidationResult {
    if let Ok(num) = value.parse::<f64>() {
        if let Some(min_val) = min {
            if num < min_val {
                return ValidationResult::Invalid(format!("Value must be at least {}", min_val));
            }
        }
        if let Some(max_val) = max {
            if num > max_val {
                return ValidationResult::Invalid(format!("Value must be at most {}", max_val));
            }
        }
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid("Please enter a valid number".to_string())
    }
}

/// Utility function for required field validation
pub fn validate_required(value: &str, field_name: &str) -> ValidationResult {
    if value.trim().is_empty() {
        ValidationResult::Invalid(format!("{} is required", field_name))
    } else {
        ValidationResult::Valid
    }
}

/// Utility function for length validation
pub fn validate_length(value: &str, min: Option<usize>, max: Option<usize>) -> ValidationResult {
    let len = value.len();

    if let Some(min_len) = min {
        if len < min_len {
            return ValidationResult::Invalid(format!("Must be at least {} characters", min_len));
        }
    }
    if let Some(max_len) = max {
        if len > max_len {
            return ValidationResult::Invalid(format!("Must be at most {} characters", max_len));
        }
    }

    ValidationResult::Valid
}
