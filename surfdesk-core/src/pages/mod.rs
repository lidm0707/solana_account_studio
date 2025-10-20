//! Pages module for Surfdesk
//!
//! This module contains all the page components that render different views
//! in the application based on the current route.

// pub mod accounts;
pub mod home;
pub mod program_builder;
// pub mod programs;
// pub mod surfpool;

// Re-export page components for easier access
// pub use accounts::AccountManager;
pub use home::Home;
pub use program_builder::ProgramBuilderPage;
// pub use programs::ProgramBuilder;
// pub use surfpool::SurfpoolManager;
