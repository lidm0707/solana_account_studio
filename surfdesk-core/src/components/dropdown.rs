//! Dropdown components for SurfDesk
//!
//! Provides various dropdown implementations including single-select,
//! multi-select, and searchable dropdowns with proper Dioxus 0.6+ compatibility.

use dioxus::prelude::*;
use std::collections::HashSet;

/// Represents a dropdown value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DropdownValue {
    String(String),
    Number(i64),
    Boolean(bool),
}

impl DropdownValue {
    /// Convert to string representation
    pub fn as_string(&self) -> String {
        match self {
            DropdownValue::String(s) => s.clone(),
            DropdownValue::Number(n) => n.to_string(),
            DropdownValue::Boolean(b) => b.to_string(),
        }
    }

    /// Check if this value matches another
    pub fn matches(&self, other: &DropdownValue) -> bool {
        self == other
    }
}

/// Represents a dropdown option
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownOption {
    /// Option value
    pub value: DropdownValue,
    /// Display label
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Whether option is disabled
    pub disabled: bool,
    /// Optional icon
    pub icon: Option<String>,
    /// Optional grouping
    pub group: Option<String>,
}

/// Single-select dropdown props
#[derive(Debug, PartialEq, Clone, Props)]
pub struct DropdownProps {
    /// Dropdown options
    pub options: Vec<DropdownOption>,
    /// Currently selected value
    pub value: Option<DropdownValue>,
    /// Placeholder text
    #[props(default = "Select an option".to_string())]
    pub placeholder: String,
    /// Whether dropdown is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Whether field is required
    #[props(default = false)]
    pub required: bool,
    /// Custom CSS classes
    pub class: Option<String>,
    /// Change handler
    pub onchange: EventHandler<DropdownValue>,
    /// Focus handler
    pub onfocus: EventHandler<FocusEvent>,
    /// Blur handler
    pub onblur: EventHandler<FocusEvent>,
}

/// Single-select dropdown component
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: format!("dropdown-container {}", props.class.as_deref().unwrap_or("")),
            div { class: "dropdown-trigger",
                button {
                    class: format!("dropdown-button {} {}",
                        if props.disabled { "disabled" } else { "" },
                        if is_open() { "open" } else { "" }
                    ),
                    r#type: "button",
                    disabled: props.disabled,
                    onclick: move |_| {
                        if !props.disabled {
                            is_open.set(!is_open());
                        }
                    },
                    onfocus: move |e: FocusEvent| {
                        if !props.disabled {
                            is_open.set(true);
                        }
                        props.onfocus.call(e);
                    },
                    onblur: move |e: FocusEvent| {
                        is_open.set(false);
                        props.onblur.call(e);
                    },

                    // Selected value or placeholder
                    span { class: "dropdown-value",
                        {props.value.as_ref().map(|v| v.as_string()).unwrap_or_else(|| props.placeholder.clone())}
                    }

                    // Dropdown arrow
                    span { class: "dropdown-arrow",
                        {if is_open() { "▲" } else { "▼" }}
                    }
                }

                // Dropdown menu
                if is_open() {
                    div { class: "dropdown-menu",
                        {props.options.iter().enumerate().map(|(index, option)| {
                            let is_selected = props.value.as_ref().map_or(false, |v| v.matches(&option.value));
                            let option_clone = option.clone();

                            rsx! {
                                button {
                                    class: format!("dropdown-option {} {}",
                                        if option.disabled { "disabled" } else { "" },
                                        if is_selected { "selected" } else { "" }
                                    ),
                                    r#type: "button",
                                    disabled: option.disabled,
                                    onclick: move |_| {
                                        if !option_clone.disabled {
                                            props.onchange.call(option_clone.value.clone());
                                            is_open.set(false);
                                        }
                                    },
                                    role: "option",
                                    "aria-selected": "{is_selected}",
                                    "aria-disabled": "{option.disabled}",

                                    // Option icon
                                    if let Some(icon) = &option.icon {
                                        span { class: "dropdown-option-icon", "{icon}" }
                                    }

                                    // Option content
                                    div { class: "dropdown-option-content",
                                        div { class: "dropdown-option-label", "{option.label}" }
                                        if let Some(description) = &option.description {
                                            div { class: "dropdown-option-description", "{description}" }
                                        }
                                    }
                                }
                            }
                        })}

                        // No options message
                        if props.options.is_empty() {
                            div { class: "dropdown-no-options",
                                "No options available"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Multi-select dropdown props
#[derive(Debug, PartialEq, Clone, Props)]
pub struct MultiSelectDropdownProps {
    /// Dropdown options
    pub options: Vec<DropdownOption>,
    /// Currently selected values
    pub values: HashSet<DropdownValue>,
    /// Placeholder text
    #[props(default = "Select options".to_string())]
    pub placeholder: String,
    /// Maximum number of selections
    pub max_selections: Option<usize>,
    /// Whether dropdown is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Custom CSS classes
    pub class: Option<String>,
    /// Change handler
    pub onchange: EventHandler<HashSet<DropdownValue>>,
    /// Focus handler
    pub onfocus: EventHandler<FocusEvent>,
    /// Blur handler
    pub onblur: EventHandler<FocusEvent>,
}

/// Multi-select dropdown component
#[component]
pub fn MultiSelectDropdown(props: MultiSelectDropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut selected_values = use_signal(|| props.values.clone());

    rsx! {
        div { class: format!("multi-select-dropdown-container {}", props.class.as_deref().unwrap_or("")),
            div { class: "dropdown-trigger",
                button {
                    class: format!("dropdown-button {} {}",
                        if props.disabled { "disabled" } else { "" },
                        if is_open() { "open" } else { "" }
                    ),
                    r#type: "button",
                    disabled: props.disabled,
                    onclick: move |_| {
                        if !props.disabled {
                            is_open.set(!is_open());
                        }
                    },
                    onfocus: move |e: FocusEvent| {
                        if !props.disabled {
                            is_open.set(true);
                        }
                        props.onfocus.call(e);
                    },
                    onblur: move |e: FocusEvent| {
                        is_open.set(false);
                        props.onblur.call(e);
                    },

                    // Selected values or placeholder
                    span { class: "dropdown-value",
                        if selected_values().is_empty() {
                            {props.placeholder.clone()}
                        } else {
                            {format!("{} selected", selected_values().len())}
                        }
                    }

                    // Dropdown arrow
                    span { class: "dropdown-arrow",
                        {if is_open() { "▲" } else { "▼" }}
                    }
                }

                // Dropdown menu
                if is_open() {
                    div { class: "dropdown-menu",
                        {props.options.iter().enumerate().map(|(index, option)| {
                            let is_selected = selected_values().contains(&option.value);
                            let option_clone = option.clone();
                            let mut values_signal = selected_values.clone();
                            let max_selections = props.max_selections;
                            let onchange = props.onchange.clone();

                            rsx! {
                                button {
                                    class: format!("dropdown-option {} {}",
                                        if option.disabled { "disabled" } else { "" },
                                        if is_selected { "selected" } else { "" }
                                    ),
                                    r#type: "button",
                                    disabled: option.disabled,
                                    onclick: move |_| {
                                        if !option_clone.disabled {
                                            let mut values = values_signal.read().clone();

                                            if values.contains(&option_clone.value) {
                                                values.remove(&option_clone.value);
                                            } else {
                                                // Check max selections
                                                if let Some(max) = max_selections {
                                                    if values.len() >= max {
                                                        return;
                                                    }
                                                }
                                                values.insert(option_clone.value.clone());
                                            }

                                            values_signal.set(values.clone());
                                            onchange.call(values);
                                        }
                                    },
                                    role: "option",
                                    "aria-selected": "{is_selected}",
                                    "aria-disabled": "{option.disabled}",

                                    // Checkbox
                                    span { class: "dropdown-option-checkbox",
                                        if is_selected { "✓" } else { "" }
                                    }

                                    // Option icon
                                    if let Some(icon) = &option.icon {
                                        span { class: "dropdown-option-icon", "{icon}" }
                                    }

                                    // Option content
                                    div { class: "dropdown-option-content",
                                        div { class: "dropdown-option-label", "{option.label}" }
                                        if let Some(description) = &option.description {
                                            div { class: "dropdown-option-description", "{description}" }
                                        }
                                    }
                                }
                            }
                        })}

                        // No options message
                        if props.options.is_empty() {
                            div { class: "dropdown-no-options",
                                "No options available"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Searchable dropdown props
#[derive(Debug, PartialEq, Clone, Props)]
pub struct SearchableDropdownProps {
    /// Dropdown options
    pub options: Vec<DropdownOption>,
    /// Currently selected value
    pub value: Option<DropdownValue>,
    /// Placeholder text
    #[props(default = "Search and select".to_string())]
    pub placeholder: String,
    /// Whether to show search on focus (always open)
    #[props(default = true)]
    pub always_show_search: bool,
    /// Whether dropdown is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Custom search function placeholder
    pub search_fn: Option<String>,
    /// Custom CSS classes
    pub class: Option<String>,
    /// Change handler
    pub onchange: EventHandler<DropdownValue>,
    /// Focus handler
    pub onfocus: EventHandler<FocusEvent>,
    /// Blur handler
    pub onblur: EventHandler<FocusEvent>,
}

/// Searchable dropdown component
#[component]
pub fn SearchableDropdown(props: SearchableDropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());

    let filtered_options = use_memo(move || {
        let query = search_query.read().to_lowercase();
        if query.is_empty() {
            props.options.clone()
        } else {
            props
                .options
                .iter()
                .filter(|option| {
                    option.label.to_lowercase().contains(&query)
                        || option
                            .description
                            .as_ref()
                            .map_or(false, |desc| desc.to_lowercase().contains(&query))
                })
                .cloned()
                .collect()
        }
    });

    rsx! {
        div { class: format!("searchable-dropdown-container {}", props.class.as_deref().unwrap_or("")),
            div { class: "dropdown-trigger",
                button {
                    class: format!("dropdown-button {} {}",
                        if props.disabled { "disabled" } else { "" },
                        if is_open() { "open" } else { "" }
                    ),
                    r#type: "button",
                    disabled: props.disabled,
                    onclick: move |_| {
                        if !props.disabled {
                            is_open.set(!is_open());
                        }
                    },
                    onfocus: move |e: FocusEvent| {
                        if !props.disabled {
                            is_open.set(true);
                        }
                        props.onfocus.call(e);
                    },
                    onblur: move |e: FocusEvent| {
                        is_open.set(false);
                        props.onblur.call(e);
                    },

                    // Selected value or placeholder
                    span { class: "dropdown-value",
                        {props.value.as_ref().map(|v| v.as_string()).unwrap_or_else(|| props.placeholder.clone())}
                    }

                    // Dropdown arrow
                    span { class: "dropdown-arrow",
                        {if is_open() { "▲" } else { "▼" }}
                    }
                }

                // Dropdown menu with search
                if is_open() {
                    div { class: "dropdown-menu searchable",
                        // Search input
                        div { class: "dropdown-search",
                            input {
                                class: "search-input",
                                r#type: "text",
                                placeholder: "Search options...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                                autofocus: true,
                            }
                        }

                        // Filtered options
                        div { class: "dropdown-options",
                            {filtered_options().iter().enumerate().map(|(index, option)| {
                                let is_selected = props.value.as_ref().map_or(false, |v| v.matches(&option.value));
                                let option_clone = option.clone();

                                rsx! {
                                    button {
                                        class: format!("dropdown-option {} {}",
                                            if option.disabled { "disabled" } else { "" },
                                            if is_selected { "selected" } else { "" }
                                        ),
                                        r#type: "button",
                                        disabled: option.disabled,
                                        onclick: move |_| {
                                            if !option_clone.disabled {
                                                props.onchange.call(option_clone.value.clone());
                                                is_open.set(false);
                                                search_query.set(String::new());
                                            }
                                        },
                                        role: "option",
                                        "aria-selected": "{is_selected}",
                                        "aria-disabled": "{option.disabled}",

                                        // Option icon
                                        if let Some(icon) = &option.icon {
                                            span { class: "dropdown-option-icon", "{icon}" }
                                        }

                                        // Option content
                                        div { class: "dropdown-option-content",
                                            div { class: "dropdown-option-label", "{option.label}" }
                                            if let Some(description) = &option.description {
                                                div { class: "dropdown-option-description", "{description}" }
                                            }
                                        }
                                    }
                                }
                            })}

                            // No results message
                            if filtered_options().is_empty() {
                                div { class: "dropdown-no-results",
                                    "No options found"
                                }
                            }
                        }
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
    fn test_dropdown_value() {
        let string_val = DropdownValue::String("test".to_string());
        let number_val = DropdownValue::Number(42);
        let bool_val = DropdownValue::Boolean(true);

        assert_eq!(string_val.as_string(), "test");
        assert_eq!(number_val.as_string(), "42");
        assert_eq!(bool_val.as_string(), "true");
    }

    #[test]
    fn test_dropdown_option() {
        let option = DropdownOption {
            value: DropdownValue::String("test".to_string()),
            label: "Test Option".to_string(),
            description: Some("A test option".to_string()),
            disabled: false,
            icon: Some("🧪".to_string()),
            group: None,
        };

        assert_eq!(option.label, "Test Option");
        assert_eq!(option.description, Some("A test option".to_string()));
        assert!(!option.disabled);
        assert_eq!(option.icon, Some("🧪".to_string()));
    }
}
