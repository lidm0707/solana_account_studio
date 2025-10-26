//! Program Builder Page
//!
//! This page provides a visual interface for designing Solana programs
//! and generating JSON schemas automatically.

use dioxus::prelude::*;

use crate::components::program_builder::ProgramBuilder;

// Import the program builder component

#[component]
pub fn ProgramBuilderPage() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background-color: #f9fafb;",

            // Header
            header {
                style: "background-color: white; border-bottom: 1px solid #e5e7eb; padding: 1rem 0;",
                div {
                    style: "max-width: 1200px; margin: 0 auto; padding: 0 1rem;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        div {
                            h1 {
                                style: "font-size: 1.5rem; font-weight: bold; color: #1f2937; margin: 0;",
                                "Program Builder"
                            }
                            p {
                                style: "color: #6b7280; margin: 0.25rem 0 0 0;",
                                "Design Solana programs visually and generate JSON schemas"
                            }
                        }
                        div {
                            style: "display: flex; gap: 0.5rem;",
                            button {
                                style: "padding: 0.5rem 1rem; background-color: #3b82f6; color: white; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem;",
                                "Save Project"
                            }
                            button {
                                style: "padding: 0.5rem 1rem; background-color: #10b981; color: white; border: none; border-radius: 0.375rem; cursor: pointer; font-size: 0.875rem;",
                                "Load Project"
                            }
                        }
                    }
                }
            }

            // Main Content
            main {
                style: "padding: 2rem 0;",
                ProgramBuilder {}
            }
        }
    }
}
