//! # SurfPool Page - Desktop
//!
//! Desktop-specific SurfPool page that imports UI components from core.
//! This provides a clean separation between desktop and core functionality.

/// Desktop SurfPool page component
///
/// This is a thin wrapper around the core SurfPool page component.
/// The actual UI logic and components are defined in the core module
/// to ensure consistency across different platforms.
///
use dioxus::prelude::*;
#[component]
pub fn SurfPoolPage() -> Element {
    rsx! {}
}
