//! # Button Component Module
//!
//! This module provides a reusable button component for the SurfDesk application.
//! The button component is built with Dioxus 0.6+ and supports multiple variants,
//! sizes, colors, and states across all platforms.

use super::{combine_classes, Color, Size, Variant};
use dioxus::prelude::*;

/// Button component properties
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Button variant
    #[props(default)]
    pub variant: Variant,
    /// Button size
    #[props(default)]
    pub size: Size,
    /// Button color
    #[props(default)]
    pub color: Color,
    /// Whether button is disabled
    #[props(default)]
    pub disabled: bool,
    /// Whether button is loading
    #[props(default)]
    pub loading: bool,
    /// Whether button is full width
    #[props(default)]
    pub full_width: bool,
    /// Click handler
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Button type (for forms)
    #[props(default)]
    pub button_type: ButtonType,
    /// Children elements
    children: Element,
}

/// Button type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    /// Get the HTML button type
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

/// Button component
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = [
        "btn",
        props.variant.css_class(),
        props.size.css_class(),
        props.color.css_class(),
    ];

    let mut classes = vec![];

    // Add base classes
    classes.extend_from_slice(&base_classes);

    // Add conditional classes
    if props.disabled {
        classes.push("btn-disabled");
    }

    if props.loading {
        classes.push("btn-loading");
    }

    if props.full_width {
        classes.push("btn-full-width");
    }

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class.as_str());
    }

    let class_attr = combine_classes(&classes.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    rsx! {
        button {
            class: "{class_attr}",
            id: props.id,
            type: "{props.button_type.as_str()}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(handler) = props.onclick {
                    handler.call(evt);
                }
            },

            // Loading spinner
            if props.loading {
                span { class: "btn-spinner", "⟳" }
            }

            // Button content
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_type_as_str() {
        assert_eq!(ButtonType::Button.as_str(), "button");
        assert_eq!(ButtonType::Submit.as_str(), "submit");
        assert_eq!(ButtonType::Reset.as_str(), "reset");
    }

    #[test]
    fn test_button_props_default() {
        let props = ButtonProps {
            class: None,
            id: None,
            variant: Variant::default(),
            size: Size::default(),
            color: Color::default(),
            disabled: false,
            loading: false,
            full_width: false,
            onclick: None,
            button_type: ButtonType::default(),
            children: rsx! { "Test" },
        };

        assert!(!props.disabled);
        assert!(!props.loading);
        assert!(!props.full_width);
    }
}
