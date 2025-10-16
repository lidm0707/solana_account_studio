//! # Styles Module
//!
//! Centralized styling system for SurfDesk cross-platform application.
//! Provides consistent styling across desktop, web, and terminal platforms.

// Re-export all style files
pub use design_system::*;
pub use dropdown::*;
pub use input::*;
pub use keyboard::*;
pub use loading::*;
pub use modal::*;
pub use toast::*;

/// Include all CSS styles for compilation
pub fn include_all_styles() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("design-system.css"),
        include_str!("dropdown.css"),
        include_str!("input.css"),
        include_str!("keyboard.css"),
        include_str!("loading.css"),
        include_str!("modal.css"),
        include_str!("toast.css")
    )
}

/// Style constants for common values
pub mod constants {
    /// Common spacing values
    pub const SPACE_1: &str = "0.25rem";
    pub const SPACE_2: &str = "0.5rem";
    pub const SPACE_3: &str = "0.75rem";
    pub const SPACE_4: &str = "1rem";
    pub const SPACE_5: &str = "1.25rem";
    pub const SPACE_6: &str = "1.5rem";
    pub const SPACE_8: &str = "2rem";
    pub const SPACE_10: &str = "2.5rem";
    pub const SPACE_12: &str = "3rem";

    /// Common font sizes
    pub const TEXT_XS: &str = "0.75rem";
    pub const TEXT_SM: &str = "0.875rem";
    pub const TEXT_BASE: &str = "1rem";
    pub const TEXT_LG: &str = "1.125rem";
    pub const TEXT_XL: &str = "1.25rem";
    pub const TEXT_2XL: &str = "1.5rem";
    pub const TEXT_3XL: &str = "1.875rem";

    /// Common border radius values
    pub const RADIUS_SM: &str = "0.125rem";
    pub const RADIUS_MD: &str = "0.375rem";
    pub const RADIUS_LG: &str = "0.5rem";
    pub const RADIUS_XL: &str = "0.75rem";
    pub const RADIUS_2XL: &str = "1rem";

    /// Common transition durations
    pub const DURATION_150: &str = "150ms";
    pub const DURATION_200: &str = "200ms";
    pub const DURATION_300: &str = "300ms";

    /// Common z-index values
    pub const Z_DROPDOWN: &str = "1000";
    pub const Z_MODAL: &str = "1050";
    pub const Z_TOAST: &str = "1100";
}

/// Utility functions for generating CSS classes
pub mod utils {
    /// Combine multiple CSS classes into a single string
    pub fn combine_classes(classes: &[&str]) -> String {
        classes
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Generate a conditional CSS class
    pub fn conditional_class(condition: bool, class_name: &str) -> String {
        if condition {
            class_name.to_string()
        } else {
            String::new()
        }
    }

    /// Generate CSS class with modifiers
    pub fn class_with_modifiers(base: &str, modifiers: &[&str]) -> String {
        let mut classes = vec![base];
        for modifier in modifiers {
            if !modifier.is_empty() {
                classes.push(&format!("{}--{}", base, modifier));
            }
        }
        classes.join(" ")
    }
}

// Re-export design system variables and utilities
pub mod design_system;
pub mod dropdown;
pub mod input;
pub mod keyboard;
pub mod loading;
pub mod modal;
pub mod toast;
