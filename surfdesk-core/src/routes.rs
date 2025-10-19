//! Routes module for Surfdesk
//!
//! This module defines the application routes and navigation structure
//! using Dioxus Router for client-side routing.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Import only the working home page for now
use crate::pages::home::Home;

#[component]
pub fn LayoutWrapper() -> Element {
    rsx! {
        div {
            style: "padding: 20px; max-width: 1200px; margin: 0 auto;",
            Outlet::<Route> {}
        }
    }
}

/// Application routes enumeration - simplified for now
#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LayoutWrapper)]
    #[route("/")]
    Home {},
}
