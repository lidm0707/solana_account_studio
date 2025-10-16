//! # Professional Card Component
//!
//! A versatile card component with multiple variants, sizes, and
//! interactive states for the SurfDesk desktop application.

use super::{css_class, Size};
use dioxus::prelude::*;

/// Card component props
#[derive(Debug, Clone, Props)]
pub struct CardProps {
    /// Card variant for different visual styles
    #[props(default = CardVariant::Default)]
    pub variant: CardVariant,

    /// Card size
    #[props(default = Size::Medium)]
    pub size: Size,

    /// Whether the card is elevated (has shadow)
    #[props(default = true)]
    pub elevated: bool,

    /// Whether the card is clickable
    #[props(default = false)]
    pub clickable: bool,

    /// Whether the card has a border
    #[props(default = true)]
    pub bordered: bool,

    /// Optional card title
    pub title: Option<String>,

    /// Optional card subtitle
    pub subtitle: Option<String>,

    /// Optional card icon (emoji or icon class)
    pub icon: Option<String>,

    /// Click handler
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Card content
    pub children: Element,
}

/// Card variants for different visual styles
#[derive(Debug, Clone, PartialEq)]
pub enum CardVariant {
    /// Default card with standard styling
    Default,
    /// Primary card with purple gradient
    Primary,
    /// Secondary card with green accent
    Secondary,
    /// Tertiary card with blue accent
    Tertiary,
    /// Success card with emerald theme
    Success,
    /// Warning card with amber theme
    Warning,
    /// Error card with red theme
    Error,
    /// Glass card with frosted glass effect
    Glass,
    /// Gradient card with background gradient
    Gradient,
}

/// Professional card component
#[component]
pub fn Card(props: CardProps) -> Element {
    let base_class = "surf-card";

    // Build CSS classes
    let mut modifiers = Vec::new();

    // Variant modifier
    let variant_str = match props.variant {
        CardVariant::Default => "default",
        CardVariant::Primary => "primary",
        CardVariant::Secondary => "secondary",
        CardVariant::Tertiary => "tertiary",
        CardVariant::Success => "success",
        CardVariant::Warning => "warning",
        CardVariant::Error => "error",
        CardVariant::Glass => "glass",
        CardVariant::Gradient => "gradient",
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
    if props.elevated {
        modifiers.push("elevated");
    }
    if props.clickable {
        modifiers.push("clickable");
    }
    if !props.bordered {
        modifiers.push("borderless");
    }

    let css_classes = css_class(base_class, &modifiers);

    // Handle click events
    let onclick = move |evt: MouseEvent| {
        if let Some(handler) = &props.onclick {
            handler.call(evt);
        }
    };

    rsx! {
        div {
            class: "{css_classes}",
            onclick: props.onclick.is_some().then_some(onclick),
            role: props.clickable.then_some("button"),
            "tabindex": props.clickable.then_some("0"),

            // Card header (if title, subtitle, or icon provided)
            if props.title.is_some() || props.subtitle.is_some() || props.icon.is_some() {
                div { class: "surf-card__header",

                    // Icon
                    if let Some(icon) = &props.icon {
                        span { class: "surf-card__icon",
                            "{icon}"
                        }
                    }

                    // Title and subtitle container
                    if props.title.is_some() || props.subtitle.is_some() {
                        div { class: "surf-card__header-content",

                            // Title
                            if let Some(title) = &props.title {
                                h3 { class: "surf-card__title",
                                    "{title}"
                                }
                            }

                            // Subtitle
                            if let Some(subtitle) = &props.subtitle {
                                p { class: "surf-card__subtitle",
                                    "{subtitle}"
                                }
                            }
                        }
                    }
                }
            }

            // Card body
            div { class: "surf-card__body",
                {props.children}
            }

            // Card actions slot (for buttons, links, etc.)
            div { class: "surf-card__actions",
                slot: "actions"
            }
        }
    }
}

/// Simple card for minimal content display
#[derive(Debug, Clone, Props)]
pub struct SimpleCardProps {
    /// Card title
    pub title: String,

    /// Card value or main content
    pub value: String,

    /// Optional trend indicator (up, down, neutral)
    pub trend: Option<CardTrend>,

    /// Optional change value
    pub change: Option<String>,

    /// Card variant
    #[props(default = CardVariant::Default)]
    pub variant: CardVariant,
}

/// Trend indicator for cards
#[derive(Debug, Clone, PartialEq)]
pub enum CardTrend {
    Up,
    Down,
    Neutral,
}

/// Simple card component for metrics and data display
#[component]
pub fn SimpleCard(props: SimpleCardProps) -> Element {
    let base_class = "surf-simple-card";

    let mut modifiers = Vec::new();

    // Variant modifier
    let variant_str = match props.variant {
        CardVariant::Default => "default",
        CardVariant::Primary => "primary",
        CardVariant::Secondary => "secondary",
        CardVariant::Tertiary => "tertiary",
        CardVariant::Success => "success",
        CardVariant::Warning => "warning",
        CardVariant::Error => "error",
        CardVariant::Glass => "glass",
        CardVariant::Gradient => "gradient",
    };
    modifiers.push(variant_str);

    let css_classes = css_class(base_class, &modifiers);

    rsx! {
        div { class: "{css_classes}",

            // Title
            div { class: "surf-simple-card__title",
                "{props.title}"
            }

            // Value
            div { class: "surf-simple-card__value",
                "{props.value}"
            }

            // Trend and change
            if props.trend.is_some() || props.change.is_some() {
                div { class: "surf-simple-card__trend",

                    // Trend indicator
                    if let Some(trend) = &props.trend {
                        span { class: format!("surf-simple-card__trend-indicator surf-simple-card__trend--{:?}", trend),
                            match trend {
                                CardTrend::Up => "↑",
                                CardTrend::Down => "↓",
                                CardTrend::Neutral => "→",
                            }
                        }
                    }

                    // Change value
                    if let Some(change) = &props.change {
                        span { class: "surf-simple-card__change",
                            "{change}"
                        }
                    }
                }
            }
        }
    }
}

/// Stats card for displaying metrics with charts
#[derive(Debug, Clone, Props)]
pub struct StatsCardProps {
    /// Card title
    pub title: String,

    /// Main value
    pub value: String,

    /// Optional description
    pub description: Option<String>,

    /// Progress percentage (0-100)
    pub progress: Option<f32>,

    /// Color variant for progress bar
    #[props(default = CardVariant::Primary)]
    pub variant: CardVariant,

    /// Optional icon
    pub icon: Option<String>,
}

/// Stats card component
#[component]
pub fn StatsCard(props: StatsCardProps) -> Element {
    let base_class = "surf-stats-card";

    let mut modifiers = Vec::new();

    // Variant modifier
    let variant_str = match props.variant {
        CardVariant::Default => "default",
        CardVariant::Primary => "primary",
        CardVariant::Secondary => "secondary",
        CardVariant::Tertiary => "tertiary",
        CardVariant::Success => "success",
        CardVariant::Warning => "warning",
        CardVariant::Error => "error",
        CardVariant::Glass => "glass",
        CardVariant::Gradient => "gradient",
    };
    modifiers.push(variant_str);

    let css_classes = css_class(base_class, &modifiers);

    rsx! {
        div { class: "{css_classes}",

            // Header with icon and title
            div { class: "surf-stats-card__header",

                // Icon
                if let Some(icon) = &props.icon {
                    span { class: "surf-stats-card__icon",
                        "{icon}"
                    }
                }

                // Title
                div { class: "surf-stats-card__title",
                    "{props.title}"
                }
            }

            // Value
            div { class: "surf-stats-card__value",
                "{props.value}"
            }

            // Description
            if let Some(description) = &props.description {
                div { class: "surf-stats-card__description",
                    "{description}"
                }
            }

            // Progress bar
            if let Some(progress) = props.progress {
                div { class: "surf-stats-card__progress",

                    // Progress bar
                    div { class: "surf-stats-card__progress-bar",
                        div {
                            class: "surf-stats-card__progress-fill",
                            style: "width: {progress}%"
                        }
                    }

                    // Progress percentage
                    span { class: "surf-stats-card__progress-text",
                        "{progress:.1}%"
                    }
                }
            }
        }
    }
}
