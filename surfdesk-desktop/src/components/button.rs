//! # Professional Button Component
//!
//! A versatile, accessible button component with multiple variants,
//! sizes, and states for the SurfDesk desktop application.

use super::{css_class, Size};
use dioxus::prelude::*;

/// Button component props
#[derive(Debug, Clone, Props)]
pub struct ButtonProps {
    /// Button variant for different visual styles
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    /// Button size
    #[props(default = Size::Medium)]
    pub size: Size,

    /// Whether the button is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the button is in loading state
    #[props(default = false)]
    pub loading: bool,

    /// Whether the button is full width
    #[props(default = false)]
    pub full_width: bool,

    /// Optional icon class (using emoji or icon font)
    pub icon: Option<String>,

    /// Click handler
    pub onclick: EventHandler<MouseEvent>,

    /// Button content
    pub children: Element,
}

/// Button variants for different visual styles
#[derive(Debug, Clone, PartialEq, Props)]
pub enum ButtonVariant {
    /// Primary action button (purple gradient)
    Primary,
    /// Secondary action button (green)
    Secondary,
    /// Tertiary action button (blue)
    Tertiary,
    /// Success action button (emerald)
    Success,
    /// Warning action button (amber)
    Warning,
    /// Error action button (red)
    Error,
    /// Ghost button (transparent with border)
    Ghost,
    /// Link button (text-only)
    Link,
}

/// Professional button component
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_class = "surf-button";

    // Build CSS classes
    let mut modifiers = Vec::new();

    // Variant modifier
    let variant_str = match props.variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Tertiary => "tertiary",
        ButtonVariant::Success => "success",
        ButtonVariant::Warning => "warning",
        ButtonVariant::Error => "error",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Link => "link",
    };
    modifiers.push(variant_str);

    // Size modifier
    let size_str = match props.size {
        Size::Small => "small",
        Size::Medium => "medium",
        Size::Large => "large",
    };
    modifiers.push(size_str);

    // State modifiers
    if props.disabled {
        modifiers.push("disabled");
    }
    if props.loading {
        modifiers.push("loading");
    }
    if props.full_width {
        modifiers.push("full-width");
    }

    let css_classes = css_class(base_class, &modifiers);

    // Handle click events
    let onclick = move |evt: MouseEvent| {
        if !props.disabled && !props.loading {
            props.onclick.call(evt);
        }
    };

    rsx! {
        button {
            class: "{css_classes}",
            disabled: props.disabled,
            onclick: onclick,
            "aria-disabled": props.disabled,
            "aria-busy": props.loading,

            // Loading spinner
            if props.loading {
                span { class: "surf-button__spinner",
                    "⟳"
                }
            }

            // Icon
            if let Some(icon) = &props.icon {
                span { class: "surf-button__icon",
                    "{icon}"
                }
            }

            // Button content
            span { class: "surf-button__content",
                {props.children}
            }
        }
    }
}

/// Button group for organizing multiple buttons
#[derive(Debug, Clone, Props)]
pub struct ButtonGroupProps {
    /// Group orientation
    #[props(default = ButtonGroupOrientation::Horizontal)]
    pub orientation: ButtonGroupOrientation,

    /// Spacing between buttons
    #[props(default = ButtonGroupSpacing::Small)]
    pub spacing: ButtonGroupSpacing,

    /// Button group content
    pub children: Element,
}

/// Button group orientation
#[derive(Debug, Clone, PartialEq, Props)]
pub enum ButtonGroupOrientation {
    Horizontal,
    Vertical,
}

/// Button group spacing
#[derive(Debug, Clone, PartialEq, Props)]
pub enum ButtonGroupSpacing {
    None,
    Small,
    Medium,
    Large,
}

/// Button group component
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let base_class = "surf-button-group";

    let mut modifiers = Vec::new();

    // Orientation modifier
    let orientation_str = match props.orientation {
        ButtonGroupOrientation::Horizontal => "horizontal",
        ButtonGroupOrientation::Vertical => "vertical",
    };
    modifiers.push(orientation_str);

    // Spacing modifier
    let spacing_str = match props.spacing {
        ButtonGroupSpacing::None => "none",
        ButtonGroupSpacing::Small => "small",
        ButtonGroupSpacing::Medium => "medium",
        ButtonGroupSpacing::Large => "large",
    };
    modifiers.push(spacing_str);

    let css_classes = css_class(base_class, &modifiers);

    rsx! {
        div { class: "{css_classes}",
            {props.children}
        }
    }
}

/// Icon button for actions with only an icon
#[derive(Debug, Clone, Props)]
pub struct IconButtonProps {
    /// Icon to display
    pub icon: String,

    /// Button variant
    #[props(default = ButtonVariant::Ghost)]
    pub variant: ButtonVariant,

    /// Button size
    #[props(default = Size::Medium)]
    pub size: Size,

    /// Tooltip text
    pub tooltip: Option<String>,

    /// Whether disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Click handler
    pub onclick: EventHandler<MouseEvent>,
}

/// Icon button component
#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let base_class = "surf-icon-button";

    let mut modifiers = Vec::new();

    // Variant modifier
    let variant_str = match props.variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Tertiary => "tertiary",
        ButtonVariant::Success => "success",
        ButtonVariant::Warning => "warning",
        ButtonVariant::Error => "error",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Link => "link",
    };
    modifiers.push(variant_str);

    // Size modifier
    let size_str = match props.size {
        Size::Small => "small",
        Size::Medium => "medium",
        Size::Large => "large",
    };
    modifiers.push(size_str);

    if props.disabled {
        modifiers.push("disabled");
    }

    let css_classes = css_class(base_class, &modifiers);

    rsx! {
        button {
            class: "{css_classes}",
            disabled: props.disabled,
            onclick: props.onclick,
            "aria-label": props.tooltip.clone().unwrap_or_default(),

            span { class: "surf-icon-button__icon",
                "{props.icon}"
            }

            // Tooltip
            if let Some(tooltip) = &props.tooltip {
                span { class: "surf-icon-button__tooltip",
                    "{tooltip}"
                }
            }
        }
    }
}
