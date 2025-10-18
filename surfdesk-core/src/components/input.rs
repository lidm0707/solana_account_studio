//! # Input Component (MVP Version)
//!
//! Simplified input component for MVP approach.
//! Focus on compilation success over comprehensive features.

use crate::components::{Color, Size};
use dioxus::prelude::*;

/// Input component properties (MVP simplified)
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputProps {
    /// Input value
    #[props(optional)]
    pub value: Option<String>,

    /// Input placeholder
    #[props(optional)]
    pub placeholder: Option<String>,

    /// Input type
    #[props(optional)]
    pub input_type: Option<InputType>,

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

    /// Input name
    #[props(optional)]
    pub name: Option<String>,

    /// Input ID
    #[props(optional)]
    pub id: Option<String>,

    /// On input change handler
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,

    /// On input focus handler
    #[props(optional)]
    pub on_focus: Option<EventHandler<Event<FocusData>>>,

    /// On input blur handler
    #[props(optional)]
    pub on_blur: Option<EventHandler<Event<FocusData>>>,

    /// CSS class
    #[props(optional)]
    pub class: Option<String>,
}

/// Input types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Search,
    Tel,
    Url,
}

impl InputType {
    /// Get HTML input type
    pub fn html_type(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Number => "number",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

/// Simple input component
#[component]
pub fn Input(props: InputProps) -> Element {
    let input_type = props.input_type.unwrap_or(InputType::Text);
    let size = props.size.unwrap_or(Size::Medium);
    let color = props.color.unwrap_or(Color::Primary);
    let disabled = props.disabled.unwrap_or(false);
    let readonly = props.readonly.unwrap_or(false);
    let required = props.required.unwrap_or(false);

    let mut classes = vec!["input"];
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

    if let Some(class) = &props.class {
        classes.push(class);
    }

    rsx! {
        input {
            class: "{classes.join(\" \")}",
            r#type: "{input_type.html_type()}",
            value: props.value.unwrap_or_default(),
            placeholder: props.placeholder.unwrap_or_default(),
            name: props.name.unwrap_or_default(),
            id: props.id.unwrap_or_default(),
            disabled: disabled,
            readonly: readonly,
            required: required,
            oninput: move |evt| {
                if let Some(on_input) = props.on_input {
                    on_input(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(on_focus) = props.on_focus {
                    on_focus(evt);
                }
            },
            onblur: move |evt| {
                if let Some(on_blur) = props.on_blur {
                    on_blur(evt);
                }
            },
        }
    }
}

/// Text input props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextInputProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,
}

/// Text input component
#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    rsx! {
        Input {
            input_type: Some(InputType::Text),
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_input,
        }
    }
}

/// Password input props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PasswordInputProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,
}

/// Password input component
#[component]
pub fn PasswordInput(props: PasswordInputProps) -> Element {
    rsx! {
        Input {
            input_type: Some(InputType::Password),
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_input,
        }
    }
}

/// Email input props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct EmailInputProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,
}

/// Email input component
#[component]
pub fn EmailInput(props: EmailInputProps) -> Element {
    rsx! {
        Input {
            input_type: Some(InputType::Email),
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_input,
        }
    }
}

/// Number input props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NumberInputProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,
}

/// Number input component
#[component]
pub fn NumberInput(props: NumberInputProps) -> Element {
    rsx! {
        Input {
            input_type: Some(InputType::Number),
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_input,
        }
    }
}

/// Search input props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SearchInputProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub on_input: Option<EventHandler<Event<FormData>>>,
}

/// Search input component
#[component]
pub fn SearchInput(props: SearchInputProps) -> Element {
    rsx! {
        Input {
            input_type: Some(InputType::Search),
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_basic() {
        let input = InputProps {
            value: Some("test".to_string()),
            placeholder: Some("Enter text".to_string()),
            input_type: Some(InputType::Text),
            size: Some(Size::Medium),
            color: Some(Color::Primary),
            disabled: None,
            readonly: None,
            required: None,
            name: None,
            id: None,
            on_input: None,
            on_focus: None,
            on_blur: None,
            class: None,
        };

        assert_eq!(input.value, Some("test".to_string()));
        assert_eq!(input.placeholder, Some("Enter text".to_string()));
        assert_eq!(input.input_type, Some(InputType::Text));
        assert_eq!(input.size, Some(Size::Medium));
        assert_eq!(input.color, Some(Color::Primary));
    }

    #[test]
    fn test_input_type_html() {
        assert_eq!(InputType::Text.html_type(), "text");
        assert_eq!(InputType::Password.html_type(), "password");
        assert_eq!(InputType::Email.html_type(), "email");
        assert_eq!(InputType::Number.html_type(), "number");
        assert_eq!(InputType::Search.html_type(), "search");
        assert_eq!(InputType::Tel.html_type(), "tel");
        assert_eq!(InputType::Url.html_type(), "url");
    }

    #[test]
    fn test_input_type_default() {
        assert_eq!(InputType::default(), InputType::Text);
    }

    #[test]
    fn test_text_input() {
        let text_input = TextInputProps {
            value: Some("Hello".to_string()),
            placeholder: Some("Type here".to_string()),
            on_input: None,
        };

        assert_eq!(text_input.value, Some("Hello".to_string()));
        assert_eq!(text_input.placeholder, Some("Type here".to_string()));
    }

    #[test]
    fn test_password_input() {
        let password_input = PasswordInputProps {
            value: Some("secret".to_string()),
            placeholder: Some("Enter password".to_string()),
            on_input: None,
        };

        assert_eq!(password_input.value, Some("secret".to_string()));
        assert_eq!(
            password_input.placeholder,
            Some("Enter password".to_string())
        );
    }

    #[test]
    fn test_email_input() {
        let email_input = EmailInputProps {
            value: Some("test@example.com".to_string()),
            placeholder: Some("Enter email".to_string()),
            on_input: None,
        };

        assert_eq!(email_input.value, Some("test@example.com".to_string()));
        assert_eq!(email_input.placeholder, Some("Enter email".to_string()));
    }

    #[test]
    fn test_number_input() {
        let number_input = NumberInputProps {
            value: Some("123".to_string()),
            placeholder: Some("Enter number".to_string()),
            on_input: None,
        };

        assert_eq!(number_input.value, Some("123".to_string()));
        assert_eq!(number_input.placeholder, Some("Enter number".to_string()));
    }

    #[test]
    fn test_search_input() {
        let search_input = SearchInputProps {
            value: Some("search query".to_string()),
            placeholder: Some("Search...".to_string()),
            on_input: None,
        };

        assert_eq!(search_input.value, Some("search query".to_string()));
        assert_eq!(search_input.placeholder, Some("Search...".to_string()));
    }
}
