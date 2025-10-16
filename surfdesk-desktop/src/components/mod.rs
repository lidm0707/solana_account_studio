//! # SurfDesk Desktop Component Library
//!
//! Professional component library for the SurfDesk desktop application.
//! Provides reusable, styled components with consistent design system.

pub mod button;
pub mod card;
pub mod loading;

// Re-export main components
pub use button::{Button, ButtonProps, ButtonVariant};
pub use card::{Card, CardProps, CardVariant};
pub use loading::{
    LoadingOverlay, LoadingOverlayProps, LoadingSpinner, LoadingSpinnerProps, LoadingVariant,
    Skeleton, SkeletonProps,
};

/// Common component props and utilities
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Utility function for generating CSS classes
pub fn css_class(base: &str, modifiers: &[&str]) -> String {
    let mut classes = vec![base];
    classes.extend(modifiers.iter().map(|m| format!("{}--{}", base, m)));
    classes.join(" ")
}

/// Common event handlers
pub type ClickHandler = Box<dyn Fn()>;
pub type ChangeHandler = Box<dyn Fn(String)>;
