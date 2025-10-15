//! # Form Component
//!
//! Comprehensive form component with validation, different input types,
//! and submission handling for all platforms.

use crate::components::{combine_classes, Button, CommonProps, Input, Size, Variant};
use crate::error::SurfDeskError;
use dioxus::prelude::*;

/// Form component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FormProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Form title
    #[props(optional)]
    pub title: Option<String>,

    /// Form description
    #[props(optional)]
    pub description: Option<String>,

    /// Form submission handler
    #[props(optional)]
    pub on_submit: Option<EventHandler<FormSubmitData>>,

    /// Form validation handler
    #[props(optional)]
    pub on_validate: Option<EventHandler<FormValidateData>>,

    /// Whether to show validation errors
    #[props(optional)]
    pub show_errors: Option<bool>,

    /// Form variant
    #[props(optional)]
    pub variant: Option<Variant>,

    /// Form size
    #[props(optional)]
    pub size: Option<Size>,

    /// Form fields
    #[props(optional)]
    pub fields: Option<Vec<FormField>>,

    /// Submit button text
    #[props(optional)]
    pub submit_text: Option<String>,

    /// Cancel button text
    #[props(optional)]
    pub cancel_text: Option<String>,

    /// Whether to show cancel button
    #[props(optional)]
    pub show_cancel: Option<bool>,

    /// Whether form is loading
    #[props(optional)]
    pub loading: Option<bool>,

    /// Form content
    children: Element,
}

/// Form field definition
#[derive(Debug, Clone, PartialEq)]
pub struct FormField {
    /// Field name
    pub name: String,
    /// Field label
    pub label: String,
    /// Field type
    pub field_type: FormFieldType,
    /// Field value
    pub value: String,
    /// Whether field is required
    pub required: bool,
    /// Field placeholder
    pub placeholder: Option<String>,
    /// Field validation rules
    pub validation: Option<Vec<ValidationRule>>,
    /// Field help text
    pub help_text: Option<String>,
    /// Whether field is disabled
    pub disabled: bool,
}

/// Form field types
#[derive(Debug, Clone, PartialEq)]
pub enum FormFieldType {
    Text,
    Email,
    Password,
    Number,
    Textarea,
    Select(Vec<SelectOption>),
    Checkbox,
    Radio(Vec<RadioOption>),
    File,
    Date,
    Time,
    DateTime,
}

/// Select option
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// Option value
    pub value: String,
    /// Option label
    pub label: String,
    /// Whether option is disabled
    pub disabled: bool,
}

/// Radio option
#[derive(Debug, Clone, PartialEq)]
pub struct RadioOption {
    /// Option value
    pub value: String,
    /// Option label
    pub label: String,
    /// Whether option is disabled
    pub disabled: bool,
}

/// Validation rule
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Email,
    Number,
    Min(f64),
    Max(f64),
    Custom(String), // Custom validation function name
}

/// Form submission data
#[derive(Debug, Clone)]
pub struct FormSubmitData {
    /// Form field values
    pub values: std::collections::HashMap<String, String>,
    /// Whether submission was successful
    pub success: bool,
    /// Submission error
    pub error: Option<String>,
}

/// Form validation data
#[derive(Debug, Clone)]
pub struct FormValidateData {
    /// Field name
    pub field_name: String,
    /// Field value
    pub field_value: String,
    /// Validation result
    pub valid: bool,
    /// Validation error message
    pub error: Option<String>,
}

/// Form component
#[component]
pub fn Form(props: FormProps) -> Element {
    let common = props.common.unwrap_or_default();
    let show_errors = props.show_errors.unwrap_or(true);
    let show_cancel = props.show_cancel.unwrap_or(true);
    let loading = props.loading.unwrap_or(false);
    let variant = props.variant.unwrap_or(Variant::Default);
    let size = props.size.unwrap_or(Size::Medium);
    let submit_text = props.submit_text.unwrap_or_else(|| "Submit".to_string());
    let cancel_text = props.cancel_text.unwrap_or_else(|| "Cancel".to_string());

    let mut form_data = use_signal(|| {
        if let Some(fields) = &props.fields {
            fields
                .iter()
                .map(|f| (f.name.clone(), f.value.clone()))
                .collect()
        } else {
            std::collections::HashMap::new()
        }
    });

    let mut form_errors = use_signal(std::collections::HashMap::<String, String>::new);
    let mut is_valid = use_signal(|| true);

    let mut classes = vec!["form"];
    classes.push(variant.css_class());
    classes.push(size.css_class());

    if loading {
        classes.push("form-loading");
    }

    if let Some(class) = &common.class {
        classes.push(class);
    }

    let handle_submit = move |_| {
        let mut errors = std::collections::HashMap::new();
        let mut valid = true;

        // Validate all fields
        if let Some(fields) = &props.fields {
            for field in fields {
                let value = form_data
                    .read()
                    .get(&field.name)
                    .cloned()
                    .unwrap_or_default();

                if let Some(validation_rules) = &field.validation {
                    for rule in validation_rules {
                        if let Some(error) = validate_field(&value, rule) {
                            errors.insert(field.name.clone(), error);
                            valid = false;
                            break;
                        }
                    }
                }

                if field.required && value.trim().is_empty() {
                    errors.insert(field.name.clone(), "This field is required".to_string());
                    valid = false;
                }
            }
        }

        form_errors.set(errors.clone());
        is_valid.set(valid);

        if valid {
            if let Some(on_submit) = props.on_submit {
                let submit_data = FormSubmitData {
                    values: form_data.read().clone(),
                    success: true,
                    error: None,
                };
                on_submit(submit_data);
            }
        } else if let Some(on_validate) = props.on_validate {
            for (field_name, error) in errors {
                let validate_data = FormValidateData {
                    field_name: field_name.clone(),
                    field_value: form_data
                        .read()
                        .get(&field_name)
                        .cloned()
                        .unwrap_or_default(),
                    valid: false,
                    error: Some(error),
                };
                on_validate(validate_data);
            }
        }
    };

    rsx! {
        form {
            class: combine_classes(&classes),
            id: common.id,
            onsubmit: handle_submit,

            // Form header
            if props.title.is_some() || props.description.is_some() {
                div { class: "form-header",
                    if let Some(title) = props.title {
                        h2 { class: "form-title", "{title}" }
                    }
                    if let Some(description) = props.description {
                        p { class: "form-description", "{description}" }
                    }
                }
            }

            // Form content
            div { class: "form-content",
                {props.children}

                // Render fields if provided
                if let Some(fields) = props.fields {
                    for field in fields {
                        FormFieldComponent {
                            field: field.clone(),
                            form_data: form_data.clone(),
                            form_errors: form_errors.clone(),
                        }
                    }
                }
            }

            // Form actions
            div { class: "form-actions",
                if show_cancel {
                    Button {
                        variant: Variant::Text,
                        disabled: loading,
                        onclick: move |_| {
                            // Handle cancel
                        },
                        "{cancel_text}"
                    }
                }

                Button {
                    variant: Variant::Contained,
                    disabled: loading || !*is_valid.read(),
                    loading: Some(loading),
                    onclick: handle_submit,
                    "{submit_text}"
                }
            }

            // Loading overlay
            if loading {
                div { class: "form-loading-overlay",
                    div { class: "loading-spinner" }
                }
            }
        }
    }
}

/// Individual form field component
#[component]
fn FormFieldComponent(
    field: FormField,
    form_data: Signal<std::collections::HashMap<String, String>>,
    form_errors: Signal<std::collections::HashMap<String, String>>,
) -> Element {
    let field_value = form_data
        .read()
        .get(&field.name)
        .cloned()
        .unwrap_or_default();
    let field_error = form_errors.read().get(&field.name).cloned();

    let handle_change = move |value: String| {
        let mut data = form_data.write();
        data.insert(field.name.clone(), value);
    };

    rsx! {
        div { class: "form-field",
            div { class: "form-field-header",
                label { class: "form-field-label", for: "{field.name}",
                    "{field.label}"
                    if field.required {
                        span { class: "form-field-required", " *" }
                    }
                }
            }

            div { class: "form-field-input",
                match field.field_type {
                    FormFieldType::Text => {
                        Input {
                            id: field.name.clone(),
                            value: field_value,
                            placeholder: field.placeholder.clone(),
                            disabled: field.disabled,
                            on_change: handle_change,
                        }
                    }
                    FormFieldType::Email => {
                        Input {
                            id: field.name.clone(),
                            input_type: "email",
                            value: field_value,
                            placeholder: field.placeholder.clone(),
                            disabled: field.disabled,
                            on_change: handle_change,
                        }
                    }
                    FormFieldType::Password => {
                        Input {
                            id: field.name.clone(),
                            input_type: "password",
                            value: field_value,
                            placeholder: field.placeholder.clone(),
                            disabled: field.disabled,
                            on_change: handle_change,
                        }
                    }
                    FormFieldType::Number => {
                        Input {
                            id: field.name.clone(),
                            input_type: "number",
                            value: field_value,
                            placeholder: field.placeholder.clone(),
                            disabled: field.disabled,
                            on_change: handle_change,
                        }
                    }
                    FormFieldType::Textarea => {
                        textarea {
                            id: "{field.name}",
                            class: "form-textarea",
                            placeholder: field.placeholder.clone().unwrap_or_default(),
                            disabled: field.disabled,
                            value: "{field_value}",
                            oninput: move |evt| handle_change(evt.value()),
                        }
                    }
                    FormFieldType::Select(_options) => {
                        select {
                            id: "{field.name}",
                            class: "form-select",
                            disabled: field.disabled,
                            value: "{field_value}",
                            onchange: move |evt| handle_change(evt.value()),
                            option { value: "option1", "Option 1" }
                            option { value: "option2", "Option 2" }
                        }
                    }
                    FormFieldType::Checkbox => {
                        input {
                            r#type: "checkbox",
                            id: "{field.name}",
                            class: "form-checkbox",
                            disabled: field.disabled,
                            checked: field_value == "true",
                            onchange: move |evt| handle_change(evt.checked().to_string()),
                        }
                    }
                    _ => {
                        Input {
                            id: field.name.clone(),
                            value: field_value,
                            placeholder: field.placeholder.clone(),
                            disabled: field.disabled,
                            on_change: handle_change,
                        }
                    }
                }
            }

            // Field error
            if let Some(error) = field_error {
                div { class: "form-field-error", "{error}" }
            }

            // Field help text
            if let Some(help_text) = field.help_text {
                div { class: "form-field-help", "{help_text}" }
            }
        }
    }
}

/// Validate a field value against a rule
fn validate_field(value: &str, rule: &ValidationRule) -> Option<String> {
    match rule {
        ValidationRule::Required => {
            if value.trim().is_empty() {
                Some("This field is required".to_string())
            } else {
                None
            }
        }
        ValidationRule::MinLength(min) => {
            if value.len() < *min {
                Some(format!("Minimum length is {} characters", min))
            } else {
                None
            }
        }
        ValidationRule::MaxLength(max) => {
            if value.len() > *max {
                Some(format!("Maximum length is {} characters", max))
            } else {
                None
            }
        }
        ValidationRule::Email => {
            if !value.contains('@') || !value.contains('.') {
                Some("Please enter a valid email address".to_string())
            } else {
                None
            }
        }
        ValidationRule::Number => {
            if value.parse::<f64>().is_err() {
                Some("Please enter a valid number".to_string())
            } else {
                None
            }
        }
        ValidationRule::Min(min) => {
            if let Ok(num) = value.parse::<f64>() {
                if num < *min {
                    Some(format!("Minimum value is {}", min))
                } else {
                    None
                }
            } else {
                Some("Please enter a valid number".to_string())
            }
        }
        ValidationRule::Max(max) => {
            if let Ok(num) = value.parse::<f64>() {
                if num > *max {
                    Some(format!("Maximum value is {}", max))
                } else {
                    None
                }
            } else {
                Some("Please enter a valid number".to_string())
            }
        }
        ValidationRule::Pattern(pattern) => {
            // Simple pattern matching (in real implementation, use regex)
            if pattern == "^[a-zA-Z0-9]+$" && !value.chars().all(|c| c.is_alphanumeric()) {
                Some("Only alphanumeric characters are allowed".to_string())
            } else {
                None
            }
        }
        ValidationRule::Custom(_) => {
            // Custom validation would be implemented here
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_default_props() {
        let props = FormProps {
            common: None,
            title: None,
            description: None,
            on_submit: None,
            on_validate: None,
            show_errors: None,
            variant: None,
            size: None,
            fields: None,
            submit_text: None,
            cancel_text: None,
            show_cancel: None,
            loading: None,
            children: rsx! { "Test" },
        };

        assert!(props.show_errors.unwrap_or(true));
        assert!(props.show_cancel.unwrap_or(true));
        assert_eq!(
            props.submit_text.unwrap_or_else(|| "Submit".to_string()),
            "Submit"
        );
        assert_eq!(
            props.cancel_text.unwrap_or_else(|| "Cancel".to_string()),
            "Cancel"
        );
    }

    #[test]
    fn test_form_field() {
        let field = FormField {
            name: "test_field".to_string(),
            label: "Test Field".to_string(),
            field_type: FormFieldType::Text,
            value: "".to_string(),
            required: true,
            placeholder: Some("Enter value".to_string()),
            validation: None,
            help_text: Some("Help text".to_string()),
            disabled: false,
        };

        assert_eq!(field.name, "test_field");
        assert_eq!(field.label, "Test Field");
        assert!(field.required);
        assert_eq!(field.placeholder, Some("Enter value".to_string()));
        assert!(field.help_text.is_some());
    }

    #[test]
    fn test_validation_rules() {
        // Test required validation
        assert!(validate_field("", &ValidationRule::Required).is_some());
        assert!(validate_field("value", &ValidationRule::Required).is_none());

        // Test min length validation
        assert!(validate_field("ab", &ValidationRule::MinLength(3)).is_some());
        assert!(validate_field("abc", &ValidationRule::MinLength(3)).is_none());

        // Test max length validation
        assert!(validate_field("abcd", &ValidationRule::MaxLength(3)).is_some());
        assert!(validate_field("abc", &ValidationRule::MaxLength(3)).is_none());

        // Test email validation
        assert!(validate_field("invalid", &ValidationRule::Email).is_some());
        assert!(validate_field("test@example.com", &ValidationRule::Email).is_none());

        // Test number validation
        assert!(validate_field("abc", &ValidationRule::Number).is_some());
        assert!(validate_field("123", &ValidationRule::Number).is_none());

        // Test min value validation
        assert!(validate_field("5", &ValidationRule::Min(10)).is_some());
        assert!(validate_field("10", &ValidationRule::Min(10)).is_none());

        // Test max value validation
        assert!(validate_field("15", &ValidationRule::Max(10)).is_some());
        assert!(validate_field("10", &ValidationRule::Max(10)).is_none());
    }

    #[test]
    fn test_select_option() {
        let option = SelectOption {
            value: "option1".to_string(),
            label: "Option 1".to_string(),
            disabled: false,
        };

        assert_eq!(option.value, "option1");
        assert_eq!(option.label, "Option 1");
        assert!(!option.disabled);
    }

    #[test]
    fn test_radio_option() {
        let option = RadioOption {
            value: "radio1".to_string(),
            label: "Radio 1".to_string(),
            disabled: true,
        };

        assert_eq!(option.value, "radio1");
        assert_eq!(option.label, "Radio 1");
        assert!(option.disabled);
    }
}
