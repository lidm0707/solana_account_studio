//! Form Components for Surfdesk
//!
//! This module provides specialized form components including form groups,
//! form layouts, validation helpers, and other form-related UI elements.

use crate::components::common::{Button, Input, StatusIndicator};
use dioxus::prelude::*;

/// Form group component for organizing form fields
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct FormGroupProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub label: Option<String>,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub error: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn FormGroup(props: FormGroupProps) -> Element {
    let base_class = "form-group";
    let class = if let Some(extra_class) = props.class {
        format!("{} {}", base_class, extra_class)
    } else {
        base_class.to_string()
    };

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,

            if let Some(label) = props.label {
                label { class: "form-label",
                    "{label}"
                    if props.required {
                        span { class: "form-required", "*" }
                    }
                }
            }

            if let Some(description) = props.description {
                p { class: "form-description", "{description}" }
            }

            div { class: "form-field",
                {props.children}
            }

            if let Some(error) = props.error {
                div { class: "form-error",
                    StatusIndicator {
                        status: "error".to_string(),
                        message: Some(error),
                        size: "small".to_string()
                    }
                }
            }
        }
    }
}

/// Form layout component for organizing multiple form groups
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct FormLayoutProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "vertical".to_string())]
    pub layout: String, // vertical, horizontal, inline
    #[props(default = "medium".to_string())]
    pub spacing: String, // small, medium, large
}

#[component]
pub fn FormLayout(props: FormLayoutProps) -> Element {
    let class = format!(
        "form-layout form-layout--{} form-layout--spacing-{}",
        props.layout, props.spacing
    );

    rsx! {
        div {
            class: "{class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Form validation state
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationState {
    Valid,
    Invalid(String),
    Pending,
    Unknown,
}

/// Form field with built-in validation
#[derive(Props, PartialEq, Clone)]
pub struct ValidatedFieldProps {
    pub name: String,
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub label: Option<String>,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub validator: Option<fn(String) -> ValidationState>,
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    #[props(default)]
    pub on_validate: Option<EventHandler<ValidationState>>,
}

#[component]
pub fn ValidatedField(props: ValidatedFieldProps) -> Element {
    let validation_state = use_signal(|| ValidationState::Unknown);
    let current_value = use_signal(|| props.value.clone().unwrap_or_default());

    let validate_value = move |value: String| {
        if let Some(validator) = props.validator {
            let state = validator(value.clone());
            validation_state.set(state.clone());

            if let Some(handler) = props.on_validate {
                handler.call(state);
            }

            state
        } else {
            ValidationState::Valid
        }
    };

    let handle_input = move |evt: FormEvent| {
        let value = evt.value();
        current_value.set(value.clone());

        if let Some(handler) = props.on_change {
            handler.call(value.clone());
        }

        // Validate on input
        validate_value(value);
    };

    let error_message = match validation_state() {
        ValidationState::Invalid(msg) => Some(msg),
        _ => None,
    };

    rsx! {
        FormGroup {
            label: props.label.clone(),
            required: props.required,
            error: error_message,

            Input {
                input_type: "text".to_string(),
                placeholder: props.placeholder,
                value: Some(current_value()),
                oninput: handle_input,
                required: props.required,
            }
        }
    }
}

/// Checkbox component
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct CheckboxProps {
    #[props(default)]
    pub label: Option<String>,
    #[props(default = false)]
    pub checked: bool,
    #[props(default)]
    pub on_change: Option<EventHandler<bool>>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub description: Option<String>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let checked = use_signal(|| props.checked);

    let handle_change = move |_| {
        let new_checked = !checked();
        checked.set(new_checked);

        if let Some(handler) = props.on_change {
            handler.call(new_checked);
        }
    };

    rsx! {
        div { class: "checkbox-wrapper",
            input {
                r#type: "checkbox",
                class: "checkbox-input",
                checked: checked(),
                disabled: props.disabled,
                onchange: handle_change,
                ..props.attributes
            }

            div { class: "checkbox-custom" }

            if let Some(label) = props.label {
                label { class: "checkbox-label",
                    onclick: handle_change,
                    "{label}"
                }
            }

            if let Some(description) = props.description {
                p { class: "checkbox-description", "{description}" }
            }
        }
    }
}

/// Radio button group component
#[derive(Props, PartialEq, Clone)]
pub struct RadioGroupProps {
    pub name: String,
    #[props(default)]
    pub options: Vec<RadioOption>,
    #[props(default)]
    pub selected_value: Option<String>,
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    #[props(default)]
    pub label: Option<String>,
    #[props(default)]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let selected_value = use_signal(|| props.selected_value.clone());

    let handle_change = move |value: String| {
        selected_value.set(Some(value.clone()));

        if let Some(handler) = props.on_change {
            handler.call(value);
        }
    };

    rsx! {
        div { class: "radio-group",
            if let Some(label) = props.label {
                div { class: "radio-group-label", "{label}" }
            }

            if let Some(description) = props.description {
                p { class: "radio-group-description", "{description}" }
            }

            div { class: "radio-options",
                for option in &props.options {
                    div { class: "radio-option",
                        input {
                            r#type: "radio",
                            class: "radio-input",
                            name: "{props.name}",
                            value: "{option.value}",
                            checked: selected_value().as_ref().map(|v| v == &option.value).unwrap_or(false),
                            onchange: move |_| handle_change(option.value.clone())
                        }

                        div { class: "radio-custom" }

                        div { class: "radio-content",
                            label { class: "radio-label", "{option.label}" }
                            if let Some(description) = &option.description {
                                p { class: "radio-description", "{description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Select dropdown component
#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    #[props(default)]
    pub options: Vec<SelectOption>,
    #[props(default)]
    pub selected_value: Option<String>,
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    #[props(default)]
    pub label: Option<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl Default for SelectOption {
    fn default() -> Self {
        Self {
            value: String::new(),
            label: String::new(),
            disabled: false,
        }
    }
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let selected_value = use_signal(|| props.selected_value.clone());

    let handle_change = move |evt: FormEvent| {
        let value = evt.value();
        selected_value.set(Some(value.clone()));

        if let Some(handler) = props.on_change {
            handler.call(value);
        }
    };

    rsx! {
        FormGroup {
            label: props.label.clone(),
            required: props.required,
            description: props.description.clone(),
            error: props.error.clone(),

            select {
                class: "select-input",
                onchange: handle_change,

                if let Some(placeholder) = props.placeholder {
                    option {
                        value: "",
                        disabled: true,
                        selected: selected_value().is_none(),
                        "{placeholder}"
                    }
                }

                for option in &props.options {
                    option {
                        value: "{option.value}",
                        disabled: option.disabled,
                        selected: selected_value().as_ref().map(|v| v == &option.value).unwrap_or(false),
                        "{option.label}"
                    }
                }
            }
        }
    }
}

/// Textarea component
#[derive(Props, PartialEq, Clone)]
#[props(extends = GlobalAttributes)]
pub struct TextareaProps {
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub rows: Option<i32>,
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
    #[props(default)]
    pub label: Option<String>,
    #[props(default = false)]
    pub required: bool,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub error: Option<String>,
    #[props(default = false)]
    pub resize: bool,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let current_value = use_signal(|| props.value.clone().unwrap_or_default());

    let handle_change = move |evt: FormEvent| {
        let value = evt.value();
        current_value.set(value.clone());

        if let Some(handler) = props.on_change {
            handler.call(value);
        }
    };

    rsx! {
        FormGroup {
            label: props.label.clone(),
            required: props.required,
            description: props.description.clone(),
            error: props.error.clone(),

            textarea {
                class: "textarea-input",
                placeholder: props.placeholder,
                rows: props.rows.unwrap_or(4),
                value: current_value(),
                onchange: handle_change,
                style: if !props.resize {
                    "resize: none;"
                } else {
                    ""
                },
                ..props.attributes
            }
        }
    }
}

/// Form actions component for submit/cancel buttons
#[derive(Props, PartialEq, Clone)]
pub struct FormActionsProps {
    #[props(default)]
    pub children: Element,
    #[props(default = "right".to_string())]
    pub alignment: String, // left, center, right, space-between
}

#[component]
pub fn FormActions(props: FormActionsProps) -> Element {
    rsx! {
        div { class: "form-actions form-actions--{props.alignment}",
            {props.children}
        }
    }
}

/// File input component
#[derive(Props, PartialEq, Clone)]
pub struct FileInputProps {
    #[props(default)]
    pub accept: Option<String>,
    #[props(default = false)]
    pub multiple: bool,
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,
    #[props(default)]
    pub label: Option<String>,
    #[props(default)]
    pub description: Option<String>,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
    let selected_files = use_signal(Vec::<String>::new);

    let handle_change = move |evt: FormEvent| {
        // In a real implementation, this would handle file selection
        // For now, simulate file selection
        let files = vec!["selected_file.txt".to_string()];
        selected_files.set(files.clone());

        if let Some(handler) = props.on_change {
            handler.call(files);
        }
    };

    rsx! {
        FormGroup {
            label: props.label.clone(),
            description: props.description.clone(),

            div { class: "file-input-wrapper",
                input {
                    r#type: "file",
                    class: "file-input",
                    accept: props.accept.unwrap_or_default(),
                    multiple: props.multiple,
                    onchange: handle_change
                }

                div { class: "file-input-dropzone",
                    div { class: "file-input-icon", "📁" }
                    p { class: "file-input-text",
                        {
                            if selected_files().is_empty() {
                                "Click to select files or drag and drop".to_string()
                            } else {
                                format!("{} file(s) selected", selected_files().len())
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Form stepper component for multi-step forms
#[derive(Props, PartialEq, Clone)]
pub struct FormStepperProps {
    pub steps: Vec<Step>,
    #[props(default = 0)]
    pub current_step: usize,
    #[props(default)]
    pub on_step_change: Option<EventHandler<usize>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub error: bool,
}

#[component]
pub fn FormStepper(props: FormStepperProps) -> Element {
    rsx! {
        div { class: "form-stepper",
            ol { class: "stepper-list",
                for (index, step) in props.steps.iter().enumerate() {
                    li {
                        key: "{step.id}",
                        class: "stepper-item {if index == props.current_step { 'active' } else { '' }} {if step.completed { 'completed' } else { '' }} {if step.error { 'error' } else { '' }}",

                        div { class: "stepper-indicator",
                            if step.completed {
                                "✓"
                            } else {
                                "{index + 1}"
                            }
                        }

                        div { class: "stepper-content",
                            h3 { class: "stepper-title", "{step.title}" }
                            if let Some(description) = &step.description {
                                p { class: "stepper-description", "{description}" }
                            }
                        }

                        if index < props.steps.len() - 1 {
                            div { class: "stepper-connector" }
                        }
                    }
                }
            }
        }
    }
}
