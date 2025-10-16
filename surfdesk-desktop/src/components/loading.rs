//! # Loading Spinner Component
//!
//! Simple, reusable loading spinner component for the SurfDesk desktop application.
//! Provides multiple sizes and variants for different loading scenarios.

use super::Size;
use dioxus::prelude::*;

/// Loading spinner component props
#[derive(Debug, Clone, Props)]
pub struct LoadingSpinnerProps {
    /// Spinner size
    #[props(default = Size::Medium)]
    pub size: Size,

    /// Whether the spinner is active
    #[props(default = true)]
    pub active: bool,

    /// Optional text to display below the spinner
    pub text: Option<String>,

    /// Spinner variant
    #[props(default = LoadingVariant::Default)]
    pub variant: LoadingVariant,
}

/// Loading spinner variants
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingVariant {
    /// Default spinner
    Default,
    /// Dots animation
    Dots,
    /// Pulse animation
    Pulse,
    /// Bounce animation
    Bounce,
}

/// Loading spinner component
#[component]
pub fn LoadingSpinner(props: LoadingSpinnerProps) -> Element {
    if !props.active {
        return rsx! { div {} };
    }

    let size_class = match props.size {
        Size::Small => "loading-spinner--small",
        Size::Medium => "loading-spinner--medium",
        Size::Large => "loading-spinner--large",
    };

    let variant_class = match props.variant {
        LoadingVariant::Default => "loading-spinner--default",
        LoadingVariant::Dots => "loading-spinner--dots",
        LoadingVariant::Pulse => "loading-spinner--pulse",
        LoadingVariant::Bounce => "loading-spinner--bounce",
    };

    rsx! {
        div {
            class: "loading-spinner {size_class} {variant_class}",

            match props.variant {
                LoadingVariant::Default => {
                    rsx! {
                        div { class: "spinner-circle" }
                        div { class: "spinner-circle" }
                        div { class: "spinner-circle" }
                        div { class: "spinner-circle" }
                    }
                }
                LoadingVariant::Dots => {
                    rsx! {
                        div { class: "dot" }
                        div { class: "dot" }
                        div { class: "dot" }
                    }
                }
                LoadingVariant::Pulse => {
                    rsx! {
                        div { class: "pulse-circle" }
                    }
                }
                LoadingVariant::Bounce => {
                    rsx! {
                        div { class: "bounce-dot" }
                        div { class: "bounce-dot" }
                        div { class: "bounce-dot" }
                    }
                }
            }

            if let Some(text) = &props.text {
                div { class: "loading-text",
                    "{text}"
                }
            }
        }
    }
}

/// Simple loading overlay component
#[derive(Debug, Clone, Props)]
pub struct LoadingOverlayProps {
    /// Whether the overlay is active
    #[props(default = true)]
    pub active: bool,

    /// Loading text
    pub text: Option<String>,

    /// Background opacity
    #[props(default = 0.8)]
    pub opacity: f32,
}

/// Loading overlay component
#[component]
pub fn LoadingOverlay(props: LoadingOverlayProps) -> Element {
    if !props.active {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "loading-overlay",
            style: "opacity: {props.opacity};",

            div { class: "loading-overlay-content",
                LoadingSpinner {
                    size: Size::Large,
                    text: props.text.clone(),
                    variant: LoadingVariant::Default,
                }
            }
        }
    }
}

/// Skeleton loading component for placeholder content
#[derive(Debug, Clone, Props)]
pub struct SkeletonProps {
    /// Skeleton height
    #[props(default = "1rem")]
    pub height: String,

    /// Skeleton width
    #[props(default = "100%")]
    pub width: String,

    /// Whether to animate
    #[props(default = true)]
    pub animated: bool,

    /// Border radius
    #[props(default = "0.25rem")]
    pub radius: String,
}

/// Skeleton loading component
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    rsx! {
        div {
            class: format!("skeleton {}", if props.animated { "skeleton--animated" } else { "" }),
            style: "height: {props.height}; width: {props.width}; border-radius: {props.radius};",
        }
    }
}
