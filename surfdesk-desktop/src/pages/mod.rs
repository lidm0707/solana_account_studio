#![allow(dead_code)]
//! # SurfDesk Desktop Pages Module
//!
//! Contains all page components for the desktop application.
//! Each page represents a major view in the application.

pub mod accounts;
// pub mod analytics;
pub mod dashboard;
// pub mod settings;
pub mod surfpool;
// pub mod transactions;

// Re-export page components
// pub use analytics::AnalyticsPage;
pub use dashboard::DashboardPage;
// pub use settings::SettingsPage;
pub use surfpool::SurfPoolPage;
// pub use transactions::TransactionsPage;

/// Common page props and utilities
#[derive(Debug, Clone, PartialEq)]
pub enum PageState {
    Loading,
    Ready,
    Error(String),
}

/// Utility function for page container styling
pub fn page_container_class() -> &'static str {
    "page-container"
}

/// Utility function for page header styling
pub fn page_header_class() -> &'static str {
    "page-header"
}

/// Utility function for page content styling
pub fn page_content_class() -> &'static str {
    "page-content"
}
