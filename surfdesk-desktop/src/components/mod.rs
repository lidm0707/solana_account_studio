//! # SurfDesk Desktop Component Library
//!
//! Professional component library for the SurfDesk desktop application.
//! Provides reusable, styled components with consistent design system.

pub mod button;
pub mod card;
pub mod input;
pub mod loading;
pub mod modal;
pub mod toast;

// Re-export main components
pub use button::{Button, ButtonProps, ButtonVariant};
pub use card::{Card, CardProps, CardVariant};
pub use input::{
    validate_email, validate_length, validate_number, validate_required, validate_url, Field,
    FieldProps, Input, InputGroup, InputGroupProps, InputProps, InputType, Textarea, TextareaProps,
    ValidationResult,
};
pub use loading::{
    LoadingOverlay, LoadingOverlayProps, LoadingSpinner, LoadingSpinnerProps, LoadingVariant,
    Skeleton, SkeletonProps,
};
pub use modal::{
    AlertModal, AlertModalProps, ConfirmModal, ConfirmModalProps, Modal, ModalProps, ModalSize,
    ModalVariant,
};
pub use toast::{
    NotificationToast, NotificationToastProps, ToastManager, ToastManagerProps, ToastPosition,
    ToastVariant,
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
