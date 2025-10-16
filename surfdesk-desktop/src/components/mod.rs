//! # SurfDesk Desktop Component Library
//!
//! Professional component library for the SurfDesk desktop application.
//! Provides reusable, styled components with consistent design system.

pub mod account_list;
pub mod button;
pub mod card;
pub mod input;
pub mod modal;
pub mod navigation;
pub mod status;
pub mod surfpool_controls;
pub mod transaction_builder;

// Re-export main components
pub use account_list::{AccountList, AccountListProps};
pub use button::{Button, ButtonProps, ButtonSize, ButtonVariant};
pub use card::{Card, CardProps, CardVariant};
pub use input::{Input, InputProps, InputType};
pub use modal::{Modal, ModalProps, ModalSize};
pub use navigation::{MenuBar, NavigationItem, NavigationProps};
pub use status::{StatusIndicator, StatusProps, StatusType};
pub use surfpool_controls::{SurfPoolControls, SurfPoolProps};
pub use transaction_builder::{TransactionBuilder, TransactionBuilderProps};

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
