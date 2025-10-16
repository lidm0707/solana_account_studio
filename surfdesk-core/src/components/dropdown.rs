//! # Dropdown Components
//!
//! Professional dropdown system with search, multi-select, and accessibility features
//! for the SurfDesk desktop application.

use super::css_class;
use dioxus::prelude::*;
use std::collections::HashSet;

/// Dropdown option value type
#[derive(Debug, Clone, PartialEq)]
pub enum DropdownValue {
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Boolean value
    Boolean(bool),
}

impl DropdownValue {
    /// Get string representation
    pub fn as_string(&self) -> String {
        match self {
            DropdownValue::String(s) => s.clone(),
            DropdownValue::Number(n) => n.to_string(),
            DropdownValue::Boolean(b) => b.to_string(),
        }
    }

    /// Check if value matches search query
    pub fn matches(&self, query: &str) -> bool {
        let text = self.as_string().to_lowercase();
        let query = query.to_lowercase();
        text.contains(&query)
    }
}

/// Dropdown option
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
    /// Optional group name
    pub group: Option<String>,
}

/// Dropdown props
#[derive(Debug, Clone, Props)]
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

    /// Whether dropdown is required
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

/// Dropdown component
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut focused_index = use_signal(|| 0);
    let mut search_query = use_signal(|| String::new());

    // Close dropdown when clicking outside
    use_effect(move || {
        if is_open() {
            let close_dropdown = move |_: MouseEvent| {
                is_open.set(false);
            };

            // Add global click listener
            let listener = Closure::wrap(Box::new(close_dropdown) as Box<dyn Fn(MouseEvent)>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                .unwrap();
            listener.forget();
        }
    });

    // Filter options based on search
    let filtered_options = use_memo(move || {
        if search_query().is_empty() {
            props.options.clone()
        } else {
            props
                .options
                .iter()
                .filter(|opt| {
                    opt.value.matches(&search_query())
                        || opt
                            .label
                            .to_lowercase()
                            .contains(&search_query().to_lowercase())
                })
                .cloned()
                .collect()
        }
    });

    // Handle option selection
    let handle_select = move |option: DropdownOption| {
        if !option.disabled {
            props.onchange.call(option.value.clone());
            is_open.set(false);
            search_query.set(String::new());
        }
    };

    // Handle keyboard navigation
    let handle_keydown = move |evt: KeyboardEvent| match evt.key().as_str() {
        "ArrowDown" => {
            evt.prevent_default();
            if is_open() {
                focused_index
                    .set((focused_index() + 1).min(filtered_options().len().saturating_sub(1)));
            } else {
                is_open.set(true);
            }
        }
        "ArrowUp" => {
            evt.prevent_default();
            if is_open() {
                focused_index.set(focused_index().saturating_sub(1));
            }
        }
        "Enter" | " " => {
            evt.prevent_default();
            if is_open() {
                if let Some(option) = filtered_options().get(focused_index()) {
                    handle_select(option.clone());
                }
            } else {
                is_open.set(true);
            }
        }
        "Escape" => {
            evt.prevent_default();
            is_open.set(false);
        }
        _ => {}
    };

    // Get selected option label
    let selected_label = props
        .value
        .as_ref()
        .and_then(|value| props.options.iter().find(|opt| opt.value == *value))
        .map(|opt| opt.label.clone())
        .unwrap_or_else(|| props.placeholder.clone());

    let dropdown_classes = css_class(
        "dropdown",
        &[
            if is_open() { "open" } else { "closed" },
            if props.disabled {
                "disabled"
            } else {
                "enabled"
            },
        ],
    );

    let final_classes = match &props.class {
        Some(custom) => format!("{} {}", dropdown_classes, custom),
        None => dropdown_classes,
    };

    rsx! {
        div { class: "{final_classes}",

            // Dropdown trigger
            button {
                class: "dropdown-trigger",
                r#type: "button",
                disabled: props.disabled,
                onclick: move |_| {
                    if !props.disabled {
                        is_open.set(!is_open());
                    }
                },
                onfocus: props.onfocus,
                onblur: props.onblur,
                onkeydown: handle_keydown,
                "aria-expanded": "{is_open()}",
                "aria-haspopup": "listbox",

                span { class: "dropdown-value", "{selected_label}" }
                span { class: "dropdown-arrow", "▼" }
            }

            // Dropdown menu
            if is_open() {
                div { class: "dropdown-menu",
                    role: "listbox",

                    // Search input
                    div { class: "dropdown-search",
                        input {
                            r#type: "text",
                            class: "dropdown-search-input",
                            placeholder: "Search options...",
                            value: "{search_query()}",
                            oninput: move |evt| {
                                search_query.set(evt.value());
                                focused_index.set(0);
                            },
                            onkeydown: handle_keydown,
                            autofocus: true,
                        }
                    }

                    // Options list
                    div { class: "dropdown-options",
                        for (index, option) in filtered_options().iter().enumerate() {
                            button {
                                class: css_class("dropdown-option", &[
                                    if index == focused_index() { "focused" } else { "" },
                                    if option.disabled { "disabled" } else { "enabled" },
                                    if props.value.as_ref() == Some(&option.value) { "selected" } else { "" },
                                ]),
                                r#type: "button",
                                disabled: option.disabled,
                                onclick: move |_| handle_select(option.clone()),
                                role: "option",
                                "aria-selected": "{props.value.as_ref() == Some(&option.value)}",
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

/// Multi-select dropdown props
#[derive(Debug, Clone, Props)]
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
    let mut focused_index = use_signal(|| 0);
    let mut search_query = use_signal(|| String::new());

    // Filter options
    let filtered_options = use_memo(move || {
        if search_query().is_empty() {
            props.options.clone()
        } else {
            props
                .options
                .iter()
                .filter(|opt| {
                    opt.value.matches(&search_query())
                        || opt
                            .label
                            .to_lowercase()
                            .contains(&search_query().to_lowercase())
                })
                .cloned()
                .collect()
        }
    });

    // Handle option toggle
    let handle_toggle = move |option: DropdownOption| {
        if !option.disabled {
            let mut new_values = props.values.clone();
            if new_values.contains(&option.value) {
                new_values.remove(&option.value);
            } else {
                // Check max selections
                if let Some(max) = props.max_selections {
                    if new_values.len() >= max {
                        return;
                    }
                }
                new_values.insert(option.value.clone());
            }
            props.onchange.call(new_values);
        }
    };

    // Handle remove tag
    let handle_remove = move |value: DropdownValue| {
        let mut new_values = props.values.clone();
        new_values.remove(&value);
        props.onchange.call(new_values);
    };

    // Handle keyboard navigation
    let handle_keydown = move |evt: KeyboardEvent| {
        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if is_open() {
                    focused_index
                        .set((focused_index() + 1).min(filtered_options().len().saturating_sub(1)));
                } else {
                    is_open.set(true);
                }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if is_open() {
                    focused_index.set(focused_index().saturating_sub(1));
                }
            }
            "Enter" | " " => {
                evt.prevent_default();
                if is_open() {
                    if let Some(option) = filtered_options().get(focused_index()) {
                        handle_toggle(option.clone());
                    }
                } else {
                    is_open.set(true);
                }
            }
            "Escape" => {
                evt.prevent_default();
                is_open.set(false);
            }
            "Backspace" => {
                if search_query().is_empty() && !props.values.is_empty() {
                    // Remove last selected value
                    if let Some(last_value) = props.values.iter().last().cloned() {
                        handle_remove(last_value);
                    }
                }
            }
            _ => {}
        }
    };

    // Get selected labels for display
    let selected_labels: Vec<String> = props
        .values
        .iter()
        .filter_map(|value| {
            props
                .options
                .iter()
                .find(|opt| opt.value == *value)
                .map(|opt| opt.label.clone())
        })
        .collect();

    let display_text = if selected_labels.is_empty() {
        props.placeholder.clone()
    } else if selected_labels.len() == 1 {
        selected_labels[0].clone()
    } else {
        format!("{} items selected", selected_labels.len())
    };

    let dropdown_classes = css_class(
        "dropdown",
        &[
            "multi-select",
            if is_open() { "open" } else { "closed" },
            if props.disabled {
                "disabled"
            } else {
                "enabled"
            },
        ],
    );

    let final_classes = match &props.class {
        Some(custom) => format!("{} {}", dropdown_classes, custom),
        None => dropdown_classes,
    };

    rsx! {
        div { class: "{final_classes}",

            // Dropdown trigger
            button {
                class: "dropdown-trigger",
                r#type: "button",
                disabled: props.disabled,
                onclick: move |_| {
                    if !props.disabled {
                        is_open.set(!is_open());
                    }
                },
                onfocus: props.onfocus,
                onblur: props.onblur,
                onkeydown: handle_keydown,
                "aria-expanded": "{is_open()}",
                "aria-haspopup": "listbox",

                // Selected tags
                div { class: "dropdown-selected-tags",
                    for (index, label) in selected_labels.iter().enumerate() {
                        if index < 3 { // Show max 3 tags
                            span { class: "dropdown-tag",
                                "{label}"
                                button {
                                    class: "dropdown-tag-remove",
                                    r#type: "button",
                                    onclick: move |evt: MouseEvent| {
                                        evt.stop_propagation();
                                        if let Some(value) = props.values.iter()
                                            .find(|v| {
                                                props.options.iter()
                                                    .find(|opt| opt.label == *label && opt.value == **v)
                                                    .is_some()
                                            })
                                            .cloned() {
                                            handle_remove(value);
                                        }
                                    },
                                    "×"
                                }
                            }
                        }
                    }

                    // More indicator
                    if selected_labels.len() > 3 {
                        span { class: "dropdown-more-indicator",
                            "+{}"
                        }
                    }

                    // Placeholder or count
                    if selected_labels.is_empty() {
                        span { class: "dropdown-placeholder", "{display_text}" }
                    } else if selected_labels.len() > 3 {
                        span { class: "dropdown-count", "{display_text}" }
                    }
                }

                span { class: "dropdown-arrow", "▼" }
            }

            // Dropdown menu
            if is_open() {
                div { class: "dropdown-menu",
                    role: "listbox",
                    "aria-multiselectable": "true",

                    // Search input
                    div { class: "dropdown-search",
                        input {
                            r#type: "text",
                            class: "dropdown-search-input",
                            placeholder: "Search options...",
                            value: "{search_query()}",
                            oninput: move |evt| {
                                search_query.set(evt.value());
                                focused_index.set(0);
                            },
                            onkeydown: handle_keydown,
                            autofocus: true,
                        }
                    }

                    // Options list
                    div { class: "dropdown-options",
                        for (index, option) in filtered_options().iter().enumerate() {
                            let is_selected = props.values.contains(&option.value);

                            button {
                                class: css_class("dropdown-option", &[
                                    if index == focused_index() { "focused" } else { "" },
                                    if option.disabled { "disabled" } else { "enabled" },
                                    if is_selected { "selected" } else { "" },
                                ]),
                                r#type: "button",
                                disabled: option.disabled,
                                onclick: move |_| handle_toggle(option.clone()),
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

/// Searchable dropdown props
#[derive(Debug, Clone, Props)]
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

    /// Custom search function
    pub search_fn: Option<Box<dyn Fn(&str, &DropdownOption) -> bool>>,

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
    let mut focused_index = use_signal(|| 0);
    let mut search_query = use_signal(|| String::new());
    let mut input_focused = use_signal(|| false);

    // Custom search function or default
    let search_matches = move |query: &str, option: &DropdownOption| -> bool {
        if let Some(search_fn) = &props.search_fn {
            // Use custom search function
            search_fn(query, option)
        } else {
            // Default search: match label or value
            option.label.to_lowercase().contains(&query.to_lowercase())
                || option.value.matches(query)
        }
    };

    // Filter options based on search
    let filtered_options = use_memo(move || {
        if search_query().is_empty() && !props.always_show_search {
            props.options.clone()
        } else {
            props
                .options
                .iter()
                .filter(|opt| search_matches(&search_query(), opt))
                .cloned()
                .collect()
        }
    });

    // Handle option selection
    let handle_select = move |option: DropdownOption| {
        if !option.disabled {
            props.onchange.call(option.value.clone());
            search_query.set(option.label.clone());
            is_open.set(false);
        }
    };

    // Handle input changes
    let handle_input_change = move |value: String| {
        search_query.set(value.clone());
        focused_index.set(0);
        if props.always_show_search || !value.is_empty() {
            is_open.set(true);
        }
    };

    // Handle keyboard navigation
    let handle_keydown = move |evt: KeyboardEvent| match evt.key().as_str() {
        "ArrowDown" => {
            evt.prevent_default();
            if is_open() {
                focused_index
                    .set((focused_index() + 1).min(filtered_options().len().saturating_sub(1)));
            } else {
                is_open.set(true);
            }
        }
        "ArrowUp" => {
            evt.prevent_default();
            if is_open() {
                focused_index.set(focused_index().saturating_sub(1));
            }
        }
        "Enter" => {
            evt.prevent_default();
            if is_open() {
                if let Some(option) = filtered_options().get(focused_index()) {
                    handle_select(option.clone());
                }
            }
        }
        "Escape" => {
            evt.prevent_default();
            is_open.set(false);
        }
        _ => {}
    };

    // Handle input focus
    let handle_input_focus = move |evt: FocusEvent| {
        input_focused.set(true);
        if props.always_show_search {
            is_open.set(true);
        }
        props.onfocus.call(evt);
    };

    // Handle input blur
    let handle_input_blur = move |evt: FocusEvent| {
        input_focused.set(false);
        // Delay closing to allow option clicks
        use_coroutine(|_| {
            let is_open = is_open.clone();
            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                is_open.set(false);
            }
        });
        props.onblur.call(evt);
    };

    let dropdown_classes = css_class(
        "dropdown",
        &[
            "searchable",
            if is_open() { "open" } else { "closed" },
            if input_focused() { "focused" } else { "" },
        ],
    );

    let final_classes = match &props.class {
        Some(custom) => format!("{} {}", dropdown_classes, custom),
        None => dropdown_classes,
    };

    rsx! {
        div { class: "{final_classes}",

            // Search input (always visible)
            div { class: "dropdown-search-container",
                input {
                    r#type: "text",
                    class: "dropdown-search-input",
                    placeholder: "{props.placeholder}",
                    value: "{search_query()}",
                    oninput: move |evt| handle_input_change(evt.value()),
                    onfocus: handle_input_focus,
                    onblur: handle_input_blur,
                    onkeydown: handle_keydown,
                    role: "combobox",
                    "aria-expanded": "{is_open()}",
                    "aria-autocomplete": "list",
                    "aria-activedescendant": if is_open() && !filtered_options().is_empty() {
                        format!("dropdown-option-{}", focused_index())
                    } else {
                        String::new()
                    },
                }

                // Clear button
                if !search_query().is_empty() {
                    button {
                        class: "dropdown-clear",
                        r#type: "button",
                        onclick: move |_| {
                            search_query.set(String::new());
                            is_open.set(false);
                        },
                        "×"
                    }
                }
            }

            // Dropdown menu
            if is_open() && (!filtered_options().is_empty() || !search_query().is_empty()) {
                div { class: "dropdown-menu",
                    role: "listbox",

                    // Options list
                    div { class: "dropdown-options",
                        for (index, option) in filtered_options().iter().enumerate() {
                            let is_selected = props.value.as_ref() == Some(&option.value);

                            button {
                                class: css_class("dropdown-option", &[
                                    if index == focused_index() { "focused" } else { "" },
                                    if option.disabled { "disabled" } else { "enabled" },
                                    if is_selected { "selected" } else { "" },
                                ]),
                                r#type: "button",
                                disabled: option.disabled,
                                onclick: move |_| handle_select(option.clone()),
                                role: "option",
                                id: "dropdown-option-{index}",
                                "aria-selected": "{is_selected}",
                                "aria-disabled": "{option.disabled}",

                                // Option icon
                                if let Some(icon) = &option.icon {
                                    span { class: "dropdown-option-icon", "{icon}" }
                                }

                                // Option content with highlighted search match
                                div { class: "dropdown-option-content",
                                    div { class: "dropdown-option-label",
                                        // Basic highlighting (could be enhanced)
                                        "{option.label}"
                                    }
                                    if let Some(description) = &option.description {
                                        div { class: "dropdown-option-description", "{description}" }
                                    }
                                }
                            }
                        }

                        // No results message
                        if filtered_options().is_empty() {
                            div { class: "dropdown-no-results",
                                "No options found for \"{search_query()}\""
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
        let number_val = DropdownValue::Number(42.0);
        let bool_val = DropdownValue::Boolean(true);

        assert_eq!(string_val.as_string(), "test");
        assert_eq!(number_val.as_string(), "42");
        assert_eq!(bool_val.as_string(), "true");

        assert!(string_val.matches("test"));
        assert!(string_val.matches("tes"));
        assert!(!string_val.matches("xyz"));
    }

    #[test]
    fn test_dropdown_option() {
        let option = DropdownOption {
            value: DropdownValue::String("option1".to_string()),
            label: "Option 1".to_string(),
            description: Some("Description".to_string()),
            disabled: false,
            icon: Some("📁".to_string()),
            group: Some("Group 1".to_string()),
        };

        assert_eq!(option.label, "Option 1");
        assert_eq!(option.description, Some("Description".to_string()));
        assert!(!option.disabled);
        assert_eq!(option.icon, Some("📁".to_string()));
    }

    #[test]
    fn test_css_class_generation() {
        let result = css_class("dropdown", &["open", "enabled"]);
        assert_eq!(result, "dropdown dropdown--open dropdown--enabled");

        let result = css_class("dropdown", &[]);
        assert_eq!(result, "dropdown");
    }
}
