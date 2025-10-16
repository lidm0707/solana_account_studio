//! Test helpers and utilities for SurfDesk testing
//!
//! This module provides helper functions and utilities for testing Dioxus components,
//! services, and integration scenarios in the SurfDesk application.

use dioxus::prelude::*;
use pretty_assertions::assert_str_eq;
use surfdesk_core::platform::Platform;
use surfdesk_core::state::AppState;

/// Create a test VirtualDom with default configuration
pub fn create_test_vdom() -> VirtualDom {
    VirtualDom::new(|| {
        rsx! {
            div { "Test App" }
        }
    })
}

/// Setup test application state with default values
pub fn setup_test_state() -> Signal<AppState> {
    use_signal(|| AppState::default())
}

/// Create a test component with common props
pub fn create_test_component<T>(
    component: impl FnOnce() -> T,
) -> T {
    component()
}

/// Render a component to string for testing
pub fn render_component(component: Element) -> String {
    dioxus_ssr::render_element(component)
}

/// Assert that two components render identically
pub fn assert_components_eq(expected: Element, actual: Element) {
    let expected_rendered = render_component(expected);
    let actual_rendered = render_component(actual);
    assert_str_eq!(expected_rendered, actual_rendered);
}

/// Create a mock event handler for testing
pub fn mock_event_handler() -> EventHandler<()> {
    EventHandler::new(|_| {})
}

/// Create a mock mouse event handler for testing
pub fn mock_mouse_handler() -> EventHandler<dioxus::events::MouseEvent> {
    EventHandler::new(|_| {})
}

/// Test platform-specific rendering
pub struct PlatformTestHelper {
    platform: Platform,
}

impl PlatformTestHelper {
    /// Create a new platform test helper
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    /// Get the platform being tested
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Get expected platform class name
    pub fn expected_class(&self) -> &'static str {
        match self.platform {
            Platform::Desktop => "platform-desktop",
            Platform::Web => "platform-web",
            Platform::Terminal => "platform-terminal",
        }
    }

    /// Assert component has correct platform class
    pub fn assert_platform_class(&self, rendered: &str) {
        assert!(
            rendered.contains(self.expected_class()),
            "Expected rendered content to contain platform class '{}'\nActual:\n{}",
            self.expected_class(),
            rendered
        );
    }
}

/// Async test helper for testing components with async behavior
pub async fn test_async_component<F, Fut>(test_fn: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    test_fn().await
}

/// Mock data generator for testing
pub struct MockDataGenerator;

impl MockDataGenerator {
    /// Generate a test account public key
    pub fn pubkey() -> String {
        "11111111111111111111111111111112".to_string()
    }

    /// Generate a test account label
    pub fn account_label() -> String {
        "Test Account".to_string()
    }

    /// Generate a test transaction signature
    pub fn tx_signature() -> String {
        "5j7s8X9Be1FQC2sV5V4qQkAyQh8q1LJj8X1q2V4w5X6Y7Z8A9B0C1D2E3F4G5H6".to_string()
    }

    /// Generate test environment name
    pub fn environment_name() -> String {
        "test-environment".to_string()
    }

    /// Generate test project name
    pub fn project_name() -> String {
        "Test Project".to_string()
    }
}

/// Assertion helpers for component testing
pub struct ComponentAssertions;

impl ComponentAssertions {
    /// Assert component contains specific CSS class
    pub fn has_class(rendered: &str, class_name: &str) {
        assert!(
            rendered.contains(&format!("class=\"{}\"", class_name)),
            "Expected component to have class '{}'\nRendered:\n{}",
            class_name,
            rendered
        );
    }

    /// Assert component contains specific text
    pub fn contains_text(rendered: &str, text: &str) {
        assert!(
            rendered.contains(text),
            "Expected component to contain text '{}'\nRendered:\n{}",
            text,
            rendered
        );
    }

    /// Assert component does not contain specific text
    pub fn does_not_contain_text(rendered: &str, text: &str) {
        assert!(
            !rendered.contains(text),
            "Expected component NOT to contain text '{}'\nRendered:\n{}",
            text,
            rendered
        );
    }

    /// Assert component has specific element/tag
    pub fn has_element(rendered: &str, tag: &str) {
        assert!(
            rendered.contains(&format!("<{}", tag)) && rendered.contains(&format!("</{}>", tag)),
            "Expected component to have <{}> element\nRendered:\n{}",
            tag,
            rendered
        );
    }

    /// Assert component has specific ID
    pub fn has_id(rendered: &str, id: &str) {
        assert!(
            rendered.contains(&format!("id=\"{}\"", id)),
            "Expected component to have id '{}'\nRendered:\n{}",
            id,
            rendered
        );
    }
}

/// Performance testing helpers
pub struct PerformanceTestHelper;

impl PerformanceTestHelper {
    /// Measure render time of a component
    pub fn measure_render_time<F, R>(component_fn: F) -> std::time::Duration
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        component_fn();
        start.elapsed()
    }

    /// Assert component renders within time limit
    pub fn assert_render_time_under<F, R>(component_fn: F, max_duration: std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let duration = Self::measure_render_time(component_fn);
        assert!(
            duration <= max_duration,
            "Component took {:?} to render, which exceeds the limit of {:?}",
            duration,
            max_duration
        );
    }
}

/// Error handling test helpers
pub struct ErrorTestHelper;

impl ErrorTestHelper {
    /// Create a mock error for testing
    pub fn mock_error(message: &str) -> surfdesk_core::error::SurfDeskError {
        surfdesk_core::error::SurfDeskError::TestError(message.to_string())
    }

    /// Assert error handling in component
    pub fn assert_error_handling<F, R>(component_fn: F) -> Result<R, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> Result<R, Box<dyn std::error::Error>>,
    {
        component_fn()
    }
}
