//! # Styles Module
//!
//! Provides styling utilities and CSS inclusion functions for SurfDesk components.
//! This module ensures consistent styling across all platforms.

/// Include all CSS styles for the application
///
/// This function returns a string containing all necessary CSS styles
/// including the design system and component-specific styles.
pub fn include_all_styles() -> String {
    let design_system_css = include_str!("design-system.css");
    let styles_css = include_str!("styles.css");

    format!(
        "{}\n\n{}",
        design_system_css,
        styles_css
    )
}

/// Get design system CSS only
pub fn get_design_system_css() -> &'static str {
    include_str!("design-system.css")
}

/// Get component styles CSS only
pub fn get_component_styles_css() -> &'static str {
    include_str!("styles.css")
}

/// Common CSS class names for consistent styling
pub mod classes {
    /// Base application class
    pub const APP_BASE: &str = "surfdesk-app";

    /// Page container class
    pub const PAGE_CONTAINER: &str = "page-container";

    /// Page header class
    pub const PAGE_HEADER: &str = "page-header";

    /// Page content class
    pub const PAGE_CONTENT: &str = "page-content";

    /// Card container class
    pub const CARD: &str = "card";

    /// Button primary class
    pub const BUTTON_PRIMARY: &str = "btn-primary";

    /// Button secondary class
    pub const BUTTON_SECONDARY: &str = "btn-secondary";

    /// Input field class
    pub const INPUT_FIELD: &str = "input-field";

    /// Status indicator class
    pub const STATUS_INDICATOR: &str = "status-indicator";

    /// Loading spinner class
    pub const LOADING_SPINNER: &str = "loading-spinner";

    /// Error message class
    pub const ERROR_MESSAGE: &str = "error-message";

    /// Success message class
    pub const SUCCESS_MESSAGE: &str = "success-message";

    /// Warning message class
    pub const WARNING_MESSAGE: &str = "warning-message";

    /// Info message class
    pub const INFO_MESSAGE: &str = "info-message";
}

/// Theme-related utilities
pub mod theme {
    /// Theme class names
    pub const LIGHT: &str = "theme-light";
    pub const DARK: &str = "theme-dark";
    pub const AUTO: &str = "theme-auto";

    /// Get theme class based on preference
    pub fn get_theme_class(theme: &str) -> &'static str {
        match theme {
            "light" => LIGHT,
            "dark" => DARK,
            _ => AUTO,
        }
    }
}

/// Animation utilities
pub mod animations {
    /// Fade-in animation class
    pub const FADE_IN: &str = "animate-fade-in";

    /// Slide-in animation class
    pub const SLIDE_IN: &str = "animate-slide-in";

    /// Pulse animation class
    pub const PULSE: &str = "animate-pulse";

    /// Spin animation class
    pub const SPIN: &str = "animate-spin";
}

/// Layout utilities
pub mod layout {
    /// Flex container class
    pub const FLEX_CONTAINER: &str = "flex-container";

    /// Grid container class
    pub const GRID_CONTAINER: &str = "grid-container";

    /// Center content class
    pub const CENTER_CONTENT: &str = "center-content";

    /// Space between class
    pub const SPACE_BETWEEN: &str = "space-between";

    /// Vertical center class
    pub const VERTICAL_CENTER: &str = "vertical-center";
}
