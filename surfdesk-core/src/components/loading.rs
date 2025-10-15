//! # Loading Component
//!
//! Loading component for displaying loading states, spinners,
//! and progress indicators across all platforms.

use crate::components::{combine_classes, Color, CommonProps, Size};
use dioxus::prelude::*;

/// Loading component properties
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LoadingProps {
    /// Common component properties
    #[props(optional)]
    pub common: Option<CommonProps>,

    /// Loading message
    #[props(optional)]
    pub message: Option<String>,

    /// Loading variant
    #[props(optional)]
    pub variant: Option<LoadingVariant>,

    /// Loading size
    #[props(optional)]
    pub size: Option<Size>,

    /// Loading color
    #[props(optional)]
    pub color: Option<Color>,

    /// Whether to show overlay
    #[props(optional)]
    pub overlay: Option<bool>,

    /// Progress value (0-100)
    #[props(optional)]
    pub progress: Option<f32>,

    /// Whether to show percentage
    #[props(optional)]
    pub show_percentage: Option<bool>,

    /// Whether to show spinner
    #[props(optional)]
    pub show_spinner: Option<bool>,

    /// Whether to show dots animation
    #[props(optional)]
    pub show_dots: Option<bool>,
}

/// Loading variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingVariant {
    Spinner,
    Dots,
    Pulse,
    Progress,
    Skeleton,
    Bars,
}

impl LoadingVariant {
    /// Get CSS class for variant
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Spinner => "loading-spinner",
            Self::Dots => "loading-dots",
            Self::Pulse => "loading-pulse",
            Self::Progress => "loading-progress",
            Self::Skeleton => "loading-skeleton",
            Self::Bars => "loading-bars",
        }
    }
}

/// Loading component
#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let common = props.common.unwrap_or_default();
    let variant = props.variant.unwrap_or(LoadingVariant::Spinner);
    let size = props.size.unwrap_or(Size::Medium);
    let color = props.color.unwrap_or(Color::Primary);
    let overlay = props.overlay.unwrap_or(false);
    let show_spinner = props.show_spinner.unwrap_or(true);
    let show_dots = props.show_dots.unwrap_or(false);
    let show_percentage = props.show_percentage.unwrap_or(false);

    let mut classes = vec!["loading"];
    classes.push(variant.css_class());
    classes.push(size.css_class());
    classes.push(color.css_class());

    if overlay {
        classes.push("loading-overlay");
    }

    if let Some(class) = &common.class {
        classes.push(class);
    }

    rsx! {
        div {
            class: combine_classes(&classes),
            id: common.id,

            // Overlay background
            if overlay {
                div { class: "loading-backdrop" }
            }

            // Loading content
            div { class: "loading-content",
                // Spinner animation
                if show_spinner && matches!(variant, LoadingVariant::Spinner) {
                    div { class: "spinner" }
                }

                // Dots animation
                if show_dots && matches!(variant, LoadingVariant::Dots) {
                    div { class: "dots",
                        span { class: "dot" }
                        span { class: "dot" }
                        span { class: "dot" }
                    }
                }

                // Progress bar
                if let Some(progress) = props.progress {
                    div { class: "progress-container",
                        div { class: "progress-bar",
                            div {
                                class: "progress-fill",
                                style: "width: {progress}%"
                            }
                        }
                        if show_percentage {
                            span { class: "progress-text", "{progress:.0}%" }
                        }
                    }
                }

                // Pulse animation
                if matches!(variant, LoadingVariant::Pulse) {
                    div { class: "pulse" }
                }

                // Skeleton animation
                if matches!(variant, LoadingVariant::Skeleton) {
                    div { class: "skeleton" }
                }

                // Bars animation
                if matches!(variant, LoadingVariant::Bars) {
                    div { class: "bars",
                        div { class: "bar" }
                        div { class: "bar" }
                        div { class: "bar" }
                        div { class: "bar" }
                    }
                }

                // Loading message
                if let Some(message) = props.message {
                    p { class: "loading-message", "{message}" }
                }
            }
        }
    }
}

/// Simple spinner loading
#[component]
pub fn Spinner(
    #[props(optional)] size: Option<Size>,
    #[props(optional)] color: Option<Color>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Loading {
            variant: LoadingVariant::Spinner,
            size: size,
            color: color,
            class: class,
            show_spinner: true,
        }
    }
}

/// Dots loading animation
#[component]
pub fn Dots(
    #[props(optional)] message: Option<String>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Loading {
            variant: LoadingVariant::Dots,
            message: message,
            class: class,
            show_dots: true,
        }
    }
}

/// Progress bar loading
#[component]
pub fn ProgressBar(
    progress: f32,
    #[props(optional)] show_percentage: Option<bool>,
    #[props(optional)] color: Option<Color>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Loading {
            variant: LoadingVariant::Progress,
            progress: Some(progress),
            show_percentage: show_percentage,
            color: color,
            class: class,
        }
    }
}

/// Full page loading overlay
#[component]
pub fn FullPageLoading(
    #[props(optional)] message: Option<String>,
    #[props(optional)] variant: Option<LoadingVariant>,
    #[props(optional)] class: Option<String>,
) -> Element {
    rsx! {
        Loading {
            message: message,
            variant: variant,
            overlay: true,
            class: class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_default_props() {
        let props = LoadingProps {
            common: None,
            message: None,
            variant: None,
            size: None,
            color: None,
            overlay: None,
            progress: None,
            show_percentage: None,
            show_spinner: None,
            show_dots: None,
        };

        assert_eq!(
            props.variant.unwrap_or(LoadingVariant::Spinner),
            LoadingVariant::Spinner
        );
        assert_eq!(props.size.unwrap_or(Size::Medium), Size::Medium);
        assert_eq!(props.color.unwrap_or(Color::Primary), Color::Primary);
        assert!(!props.overlay.unwrap_or(false));
        assert!(props.show_spinner.unwrap_or(true));
        assert!(!props.show_dots.unwrap_or(false));
    }

    #[test]
    fn test_loading_variant_css_class() {
        assert_eq!(LoadingVariant::Spinner.css_class(), "loading-spinner");
        assert_eq!(LoadingVariant::Dots.css_class(), "loading-dots");
        assert_eq!(LoadingVariant::Pulse.css_class(), "loading-pulse");
        assert_eq!(LoadingVariant::Progress.css_class(), "loading-progress");
        assert_eq!(LoadingVariant::Skeleton.css_class(), "loading-skeleton");
        assert_eq!(LoadingVariant::Bars.css_class(), "loading-bars");
    }

    #[test]
    fn test_spinner() {
        let spinner = Spinner {
            size: Some(Size::Large),
            color: Some(Color::Success),
            class: Some("custom-spinner".to_string()),
        };

        assert_eq!(spinner.size, Some(Size::Large));
        assert_eq!(spinner.color, Some(Color::Success));
        assert_eq!(spinner.class, Some("custom-spinner".to_string()));
    }

    #[test]
    fn test_dots() {
        let dots = Dots {
            message: Some("Loading...".to_string()),
            class: None,
        };

        assert_eq!(dots.message, Some("Loading...".to_string()));
    }

    #[test]
    fn test_progress_bar() {
        let progress = ProgressBar {
            progress: 75.0,
            show_percentage: Some(true),
            color: Some(Color::Info),
            class: None,
        };

        assert_eq!(progress.progress, 75.0);
        assert_eq!(progress.show_percentage, Some(true));
        assert_eq!(progress.color, Some(Color::Info));
    }

    #[test]
    fn test_full_page_loading() {
        let loading = FullPageLoading {
            message: Some("Please wait...".to_string()),
            variant: Some(LoadingVariant::Pulse),
            class: None,
        };

        assert_eq!(loading.message, Some("Please wait...".to_string()));
        assert_eq!(loading.variant, Some(LoadingVariant::Pulse));
    }
}
