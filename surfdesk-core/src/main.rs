use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod pages;
mod routes;

use pages::home::Home;
use routes::Route;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Launch the Dioxus application
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// LayoutWrapper is now defined in routes.rs
