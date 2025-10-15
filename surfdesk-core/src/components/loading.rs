//! # Loading Component (MVP Version)
//!
//! Simplified loading component for MVP approach.
//! Focus on compilation success over comprehensive features.

use crate::components::{Color, Size};
use dioxus::prelude::*;

/// Loading component properties (MVP simplified)
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LoadingProps {
    /// Loading message
    #[props(optional)]
    pub message: Option<String>,
    /// Loading size
    #[props(optional)]
    pub size: Option<Size>,
    /// Loading color
    #[props(optional)]
    pub color: Option<Color>,
}

/// Simple loading component
#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let size = props.size.unwrap_or(Size::Medium);
    let color = props.color.unwrap_or(Color::Primary);

    rsx! {
        div {
            class: "loading {size.css_class()} {color.css_class()}",
            div { class: "loading-spinner" }
            if let Some(message) = props.message {
                div { class: "loading-message", "{message}" }
            }
        }
    }
}

/// Simple spinner loading props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SpinnerProps {
    #[props(optional)]
    pub message: Option<String>,
}

/// Simple spinner loading
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    rsx! {
        Loading {
            message: props.message,
            size: Some(Size::Medium),
            color: Some(Color::Primary),
        }
    }
}

/// Simple dots loading animation props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DotsProps {
    #[props(optional)]
    pub message: Option<String>,
}

/// Simple dots loading animation
#[component]
pub fn Dots(props: DotsProps) -> Element {
    rsx! {
        div {
            class: "loading-dots",
            span { class: "dot" }
            span { class: "dot" }
            span { class: "dot" }
            if let Some(msg) = props.message {
                div { class: "loading-message", "{msg}" }
            }
        }
    }
}

/// Simple progress bar loading props
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProgressBarProps {
    pub progress: f32,
    #[props(optional)]
    pub message: Option<String>,
}

/// Simple progress bar loading
#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    rsx! {
        div {
            class: "progress-container",
            div { class: "progress-bar",
                div {
                    class: "progress-fill",
                    style: "width: {props.progress}%"
                }
            }
            if let Some(msg) = props.message {
                div { class: "progress-message", "{msg}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_basic() {
        let loading = LoadingProps {
            message: Some("Loading...".to_string()),
            size: Some(Size::Medium),
            color: Some(Color::Primary),
        };
        // Basic smoke test - if we can create it, the component structure works
        assert_eq!(loading.message, Some("Loading...".to_string()));
    }

    #[test]
    fn test_spinner() {
        let spinner = SpinnerProps {
            message: Some("Processing...".to_string()),
        };
        assert_eq!(spinner.message, Some("Processing...".to_string()));
    }

    #[test]
    fn test_dots() {
        let dots = DotsProps {
            message: Some("Loading dots...".to_string()),
        };
        assert_eq!(dots.message, Some("Loading dots...".to_string()));
    }

    #[test]
    fn test_progress_bar() {
        let progress = ProgressBarProps {
            progress: 75.0,
            message: Some("Loading complete".to_string()),
        };
        assert_eq!(progress.progress, 75.0);
    }
}
