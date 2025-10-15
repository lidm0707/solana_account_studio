//! # Main Content Component
//!
//! Main content area component for displaying the primary application
//! content with support for different layouts and content types.

use crate::components::{combine_classes, CommonProps, Size};
use dioxus::prelude::*;

/// Main content component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct MainContentProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Content title
    #[props(optional)]
    pub title: Option<String>,

    /// Content subtitle
    #[props(optional)]
    pub subtitle: Option<String>,

    /// Content layout
    #[props(optional)]
    pub layout: Option<ContentLayout>,

    /// Whether content is scrollable
    #[props(optional)]
    pub scrollable: Option<bool>,

    /// Content padding
    #[props(optional)]
    pub padding: Option<Size>,

    /// Content background
    #[props(optional)]
    pub background: Option<String>,

    /// Whether to show header
    #[props(optional)]
    pub show_header: Option<bool>,

    /// Header actions
    #[props(optional)]
    pub header_actions: Option<Element>,

    /// Content
    children: Element,
}

/// Content layout types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentLayout {
    Default,
    Centered,
    FullWidth,
    Narrow,
    Wide,
}

impl ContentLayout {
    /// Get CSS class for layout
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Default => "content-default",
            Self::Centered => "content-centered",
            Self::FullWidth => "content-full-width",
            Self::Narrow => "content-narrow",
            Self::Wide => "content-wide",
        }
    }
}

/// Main content component
#[component]
pub fn MainContent(props: MainContentProps) -> Element {
    let common = props.common.unwrap_or_default();
    let layout = props.layout.unwrap_or(ContentLayout::Default);
    let scrollable = props.scrollable.unwrap_or(true);
    let padding = props.padding.unwrap_or(Size::Large);
    let show_header = props.show_header.unwrap_or(true);

    let mut classes = vec!["main-content"];
    classes.push(layout.css_class());
    classes.push(padding.css_class());

    if scrollable {
        classes.push("content-scrollable");
    }

    if show_header && (props.title.is_some() || props.subtitle.is_some()) {
        classes.push("content-with-header");
    }

    if let Some(class) = &common.class {
        classes.push(class);
    }

    rsx! {
        main {
            class: combine_classes(&classes),
            id: common.id,
            style: if let Some(background) = props.background {
                format!("background-color: {}", background)
            } else {
                String::new()
            },

            // Content header
            if show_header && (props.title.is_some() || props.subtitle.is_some()) {
                header { class: "content-header",
                    div { class: "content-header-text",
                        if let Some(title) = props.title {
                            h1 { class: "content-title", "{title}" }
                        }
                        if let Some(subtitle) = props.subtitle {
                            p { class: "content-subtitle", "{subtitle}" }
                        }
                    }

                    if let Some(actions) = props.header_actions {
                        div { class: "content-header-actions",
                            {actions}
                        }
                    }
                }
            }

            // Content body
            div { class: "content-body",
                {props.children}
            }
        }
    }
}

/// Simple main content with title
#[component]
pub fn SimpleMainContent(
    title: String,
    #[props(optional)] children: Element,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        MainContent {
            title: title,
            children: children,
            class: class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_content_default_props() {
        let props = MainContentProps {
            common: None,
            title: None,
            subtitle: None,
            layout: None,
            scrollable: None,
            padding: None,
            background: None,
            show_header: None,
            header_actions: None,
            children: rsx! { "Test content" },
        };

        assert_eq!(
            props.layout.unwrap_or(ContentLayout::Default),
            ContentLayout::Default
        );
        assert!(props.scrollable.unwrap_or(true));
        assert_eq!(props.padding.unwrap_or(Size::Large), Size::Large);
        assert!(props.show_header.unwrap_or(true));
    }

    #[test]
    fn test_content_layout_css_class() {
        assert_eq!(ContentLayout::Default.css_class(), "content-default");
        assert_eq!(ContentLayout::Centered.css_class(), "content-centered");
        assert_eq!(ContentLayout::FullWidth.css_class(), "content-full-width");
        assert_eq!(ContentLayout::Narrow.css_class(), "content-narrow");
        assert_eq!(ContentLayout::Wide.css_class(), "content-wide");
    }

    #[test]
    fn test_simple_main_content() {
        let content = SimpleMainContent {
            title: "Test Title".to_string(),
            children: rsx! { "Test content" },
            class: Some("custom-class".to_string()),
        };

        assert_eq!(content.title, "Test Title");
        assert_eq!(content.class, Some("custom-class".to_string()));
    }
}
