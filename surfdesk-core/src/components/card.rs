//! # Card Component Module
//!
//! This module provides a reusable card component for the SurfDesk application.
//! The card component is built with Dioxus 0.6+ and supports multiple variants,
//! sizes, and layouts across all platforms.

use super::{combine_classes, Color, Size, Variant};
use dioxus::prelude::*;

/// Card component properties
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Card title
    #[props(optional)]
    pub title: Option<String>,
    /// Card subtitle
    #[props(optional)]
    pub subtitle: Option<String>,
    /// Card variant
    #[props(default)]
    pub variant: Variant,
    /// Card size
    #[props(default)]
    pub size: Size,
    /// Card color
    #[props(default)]
    pub color: Color,
    /// Whether card is elevated
    #[props(default)]
    pub elevated: bool,
    /// Whether card is clickable
    #[props(default)]
    pub clickable: bool,
    /// Click handler
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Children elements
    children: Element,
}

/// Card component
#[component]
pub fn Card(props: CardProps) -> Element {
    let mut classes = vec![
        "card",
        props.variant.css_class(),
        props.size.css_class(),
        props.color.css_class(),
    ];

    // Add conditional classes
    if props.elevated {
        classes.push("card-elevated");
    }

    if props.clickable {
        classes.push("card-clickable");
    }

    // Add custom class
    if let Some(ref class) = props.class {
        classes.push(class.as_str());
    }

    let class_attr = combine_classes(&classes.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,
            onclick: move |evt| {
                if let Some(handler) = props.onclick {
                    handler.call(evt);
                }
            },

            // Card header with title and subtitle
            if props.title.is_some() || props.subtitle.is_some() {
                div { class: "card-header",
                    if let Some(title) = props.title {
                        h3 { class: "card-title", "{title}" }
                    }
                    if let Some(subtitle) = props.subtitle {
                        p { class: "card-subtitle", "{subtitle}" }
                    }
                }
            }

            // Card content
            div { class: "card-content",
                {props.children}
            }
        }
    }
}

/// Simple stats card component
#[component]
pub fn StatsCard(
    title: String,
    value: String,
    #[props(optional)] subtitle: Option<String>,
    #[props(default)] color: Color,
    #[props(default)] size: Size,
) -> Element {
    rsx! {
        Card {
            variant: Variant::Contained,
            size,
            color,
            elevated: true,
            class: "stats-card",

            div { class: "stats-content",
                div { class: "stats-title", "{title}" }
                div { class: "stats-value", "{value}" }
                if let Some(subtitle) = subtitle {
                    div { class: "stats-subtitle", "{subtitle}" }
                }
            }
        }
    }
}

/// Info card component
#[component]
pub fn InfoCard(
    title: String,
    description: String,
    #[props(optional)] icon: Option<String>,
    #[props(default)] color: Color,
) -> Element {
    rsx! {
        Card {
            variant: Variant::Outlined,
            size: Size::Medium,
            color,
            elevated: true,
            class: "info-card",

            div { class: "info-content",
                if let Some(icon) = icon {
                    div { class: "info-icon", "{icon}" }
                }
                div { class: "info-text",
                    h4 { class: "info-title", "{title}" }
                    p { class: "info-description", "{description}" }
                }
            }
        }
    }
}

/// Action card component with button
#[component]
pub fn ActionCard(
    title: String,
    description: String,
    action_text: String,
    onclick: EventHandler<MouseEvent>,
    #[props(default)] color: Color,
) -> Element {
    rsx! {
        Card {
            variant: Variant::Contained,
            size: Size::Medium,
            color,
            elevated: true,
            class: "action-card",

            div { class: "action-content",
                h4 { class: "action-title", "{title}" }
                p { class: "action-description", "{description}" }
            }

            div { class: "action-footer",
                super::button::Button {
                    variant: Variant::Contained,
                    color,
                    onclick: Some(onclick),
                    full_width: true,
                    "{action_text}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_props_default() {
        let props = CardProps {
            class: None,
            id: None,
            title: None,
            subtitle: None,
            variant: Variant::default(),
            size: Size::default(),
            color: Color::default(),
            elevated: false,
            clickable: false,
            onclick: None,
            children: rsx! { "Test content" },
        };

        assert!(!props.elevated);
        assert!(!props.clickable);
        assert!(props.title.is_none());
        assert!(props.subtitle.is_none());
    }

    #[test]
    fn test_card_props_with_values() {
        let props = CardProps {
            class: Some("custom-card".to_string()),
            id: Some("test-card".to_string()),
            title: Some("Test Title".to_string()),
            subtitle: Some("Test Subtitle".to_string()),
            variant: Variant::Outlined,
            size: Size::Large,
            color: Color::Primary,
            elevated: true,
            clickable: true,
            onclick: None,
            children: rsx! { "Test content" },
        };

        assert!(props.elevated);
        assert!(props.clickable);
        assert_eq!(props.title, Some("Test Title".to_string()));
        assert_eq!(props.subtitle, Some("Test Subtitle".to_string()));
        assert_eq!(props.class, Some("custom-card".to_string()));
        assert_eq!(props.id, Some("test-card".to_string()));
    }
}
