//! # Components Module
//!
//! This module contains all the UI components for the SurfDesk application.
//! Components are built with Dioxus 0.6+ and designed to be reusable across
//! all platforms (desktop, web, and terminal).

/// Macro for generating Default implementations for enum types
macro_rules! impl_default_enum {
    ($enum_type:ty, $default_variant:ident) => {
        impl Default for $enum_type {
            fn default() -> Self {
                Self::$default_variant
            }
        }
    };
}

pub mod account_explorer;
pub mod app_shell;
pub mod button;
pub mod card;
pub mod dashboard;
pub mod footer;
// pub mod form;  // Temporarily commented out due to syntax issues
pub mod header;
pub mod input;
pub mod loading;
pub mod main_content;
pub mod modal;
pub mod notification;
pub mod program_manager;
// pub mod settings;  // Temporarily commented out
pub mod sidebar;
pub mod status_bar;
pub mod surfpool_control;
pub mod table;
pub mod transaction_builder;

use dioxus::prelude::*;

// Re-export commonly used components
pub use account_explorer::AccountExplorer;
pub use app_shell::AppShell;
pub use button::Button;
pub use card::Card;
pub use dashboard::Dashboard;
// pub use footer::Footer;  // Temporarily commented out
pub use footer::Footer;
// pub use form::Form; // Temporarily commented out
pub use header::Header;
pub use input::Input;
pub use loading::Loading;
pub use main_content::MainContent;
pub use modal::Modal;
pub use notification::Notification;
pub use program_manager::ProgramManager;
// pub use settings::Settings;  // Temporarily commented out
pub use sidebar::Sidebar;
// pub use status_bar::StatusBar;  // TODO: Implement
pub use surfpool_control::SurfPoolControl;
pub use table::Table; // TODO: Implement
pub use transaction_builder::TransactionBuilder; // TODO: Implement

/// Component props for common UI patterns
#[derive(Debug, Clone, PartialEq)]
pub struct CommonProps {
    /// Component class name
    pub class: Option<String>,
    /// Component ID
    pub id: Option<String>,
    /// Component title
    pub title: Option<String>,
    /// Whether component is disabled
    pub disabled: bool,
    /// Whether component is loading
    pub loading: bool,
}

impl Default for CommonProps {
    fn default() -> Self {
        Self {
            class: None,
            id: None,
            title: None,
            disabled: false,
            loading: false,
        }
    }
}

/// Size variants for components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl_default_enum!(Size, Medium);

impl Size {
    /// Get CSS class for size
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Small => "size-sm",
            Self::Medium => "size-md",
            Self::Large => "size-lg",
        }
    }
}

/// Color variants for components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
}

impl_default_enum!(Color, Primary);

impl Color {
    /// Get CSS class for color
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Primary => "color-primary",
            Self::Secondary => "color-secondary",
            Self::Success => "color-success",
            Self::Warning => "color-warning",
            Self::Error => "color-error",
            Self::Info => "color-info",
        }
    }
}

/// Variant for different component styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Outlined,
    Text,
    Contained,
}

impl_default_enum!(Variant, Default);

impl Variant {
    /// Get CSS class for variant
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Default => "variant-default",
            Self::Outlined => "variant-outlined",
            Self::Text => "variant-text",
            Self::Contained => "variant-contained",
        }
    }
}

/// Layout direction for containers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Row,
    Column,
}

impl Direction {
    /// Get CSS class for direction
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Row => "direction-row",
            Self::Column => "direction-column",
        }
    }
}

/// Alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Alignment {
    /// Get CSS class for alignment
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Start => "align-start",
            Self::Center => "align-center",
            Self::End => "align-end",
            Self::Stretch => "align-stretch",
        }
    }
}

/// Spacing values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    None,
    XS,
    SM,
    MD,
    LG,
    XL,
    XXL,
}

impl Spacing {
    /// Get CSS class for spacing
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::None => "spacing-none",
            Self::XS => "spacing-xs",
            Self::SM => "spacing-sm",
            Self::MD => "spacing-md",
            Self::LG => "spacing-lg",
            Self::XL => "spacing-xl",
            Self::XXL => "spacing-xxl",
        }
    }
}

/// Helper function to combine CSS classes
/// Combine CSS classes into a single string
pub fn combine_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Safe function to convert String to &str for CSS classes
fn safe_class(class: &str) -> &str {
    class
}

/// Helper function to create conditional class
pub fn conditional_class(condition: bool, class_name: &str) -> String {
    if condition {
        class_name.to_string()
    } else {
        "".to_string()
    }
}

/// Create a responsive layout component
#[component]
pub fn ResponsiveLayout(class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: combine_classes(&[
                "responsive-layout",
                &class.unwrap_or_default()
            ]),
            {children}
        }
    }
}

/// Create a grid layout component
#[component]
pub fn GridLayout(
    columns: Option<u32>,
    gap: Option<Spacing>,
    class: Option<String>,
    children: Element,
) -> Element {
    let columns_class = columns
        .map(|c| format!("grid-cols-{}", c))
        .unwrap_or_default();
    let gap_class = gap.map(|g| g.css_class()).unwrap_or_default();

    rsx! {
        div {
            class: combine_classes(&[
                "grid-layout",
                &columns_class,
                &gap_class,
                &class.unwrap_or_default()
            ]),
            {children}
        }
    }
}

/// Create a flex layout component
#[component]
pub fn FlexLayout(
    direction: Option<Direction>,
    align: Option<Alignment>,
    justify: Option<Alignment>,
    gap: Option<Spacing>,
    wrap: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let direction_class = direction.map(|d| d.css_class()).unwrap_or_default();
    let align_class = align.map(|a| a.css_class()).unwrap_or_default();
    let justify_class = justify
        .map(|j| format!("justify-{}", j.css_class().replace("align-", "")))
        .unwrap_or_default();
    let gap_class = gap.map(|g| g.css_class()).unwrap_or_default();
    let wrap_class = wrap.filter(|w| *w).map(|_| "flex-wrap").unwrap_or_default();

    rsx! {
        div {
            class: combine_classes(&[
                "flex-layout",
                &direction_class,
                &align_class,
                &justify_class,
                &gap_class,
                &wrap_class,
                &class.unwrap_or_default()
            ]),
            {children}
        }
    }
}

/// Create a container component
#[component]
pub fn Container(
    max_width: Option<Size>,
    padding: Option<Spacing>,
    center: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let max_width_class = max_width
        .map(|s| format!("max-w-{}", s.css_class().replace("size-", "")))
        .unwrap_or_default();
    let padding_class = padding
        .map(|p| format!("p-{}", p.css_class().replace("spacing-", "")))
        .unwrap_or_default();
    let center_class = center.filter(|c| *c).map(|_| "mx-auto").unwrap_or_default();

    rsx! {
        div {
            class: combine_classes(&[
                "container",
                &max_width_class,
                &padding_class,
                &center_class,
                &class.unwrap_or_default()
            ]),
            {children}
        }
    }
}

/// Create a spacer component
#[component]
pub fn Spacer(
    size: Option<Spacing>,
    direction: Option<Direction>,
    class: Option<String>,
) -> Element {
    let size_class = size.map(|s| s.css_class()).unwrap_or_default();
    let direction_class = direction.map(|d| d.css_class()).unwrap_or_default();

    rsx! {
        div {
            class: combine_classes(&[
                "spacer",
                &size_class,
                &direction_class,
                &class.unwrap_or_default()
            ])
        }
    }
}

/// Create a divider component
#[component]
pub fn Divider(
    vertical: Option<bool>,
    thickness: Option<Size>,
    color: Option<Color>,
    class: Option<String>,
) -> Element {
    let vertical_class = vertical
        .filter(|v| *v)
        .map(|_| "divider-vertical")
        .unwrap_or_default();
    let thickness_class = thickness.map(|t| t.css_class()).unwrap_or_default();
    let color_class = color.map(|c| c.css_class()).unwrap_or_default();

    rsx! {
        div {
            class: combine_classes(&[
                "divider",
                &vertical_class,
                &thickness_class,
                &color_class,
                &class.unwrap_or_default()
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_classes() {
        let result = combine_classes(&["class1", "class2", ""]);
        assert_eq!(result, "class1 class2");

        let result = combine_classes(&["", "", ""]);
        assert_eq!(result, "");

        let result = combine_classes(&["single-class"]);
        assert_eq!(result, "single-class");
    }

    #[test]
    fn test_conditional_class() {
        assert_eq!(conditional_class(true, "active"), "active");
        assert_eq!(conditional_class(false, "active"), "");
    }

    #[test]
    fn test_size_css_class() {
        assert_eq!(Size::Small.css_class(), "size-sm");
        assert_eq!(Size::Medium.css_class(), "size-md");
        assert_eq!(Size::Large.css_class(), "size-lg");
    }

    #[test]
    fn test_color_css_class() {
        assert_eq!(Color::Primary.css_class(), "color-primary");
        assert_eq!(Color::Success.css_class(), "color-success");
        assert_eq!(Color::Error.css_class(), "color-error");
    }

    #[test]
    fn test_variant_css_class() {
        assert_eq!(Variant::Default.css_class(), "variant-default");
        assert_eq!(Variant::Outlined.css_class(), "variant-outlined");
        assert_eq!(Variant::Text.css_class(), "variant-text");
    }

    #[test]
    fn test_direction_css_class() {
        assert_eq!(Direction::Row.css_class(), "direction-row");
        assert_eq!(Direction::Column.css_class(), "direction-column");
    }

    #[test]
    fn test_alignment_css_class() {
        assert_eq!(Alignment::Start.css_class(), "align-start");
        assert_eq!(Alignment::Center.css_class(), "align-center");
        assert_eq!(Alignment::End.css_class(), "align-end");
        assert_eq!(Alignment::Stretch.css_class(), "align-stretch");
    }

    #[test]
    fn test_spacing_css_class() {
        assert_eq!(Spacing::None.css_class(), "spacing-none");
        assert_eq!(Spacing::XS.css_class(), "spacing-xs");
        assert_eq!(Spacing::MD.css_class(), "spacing-md");
        assert_eq!(Spacing::XL.css_class(), "spacing-xl");
    }

    #[test]
    fn test_common_props_default() {
        let props = CommonProps::default();
        assert!(props.class.is_none());
        assert!(props.id.is_none());
        assert!(props.title.is_none());
        assert!(!props.disabled);
        assert!(!props.loading);
    }
}
